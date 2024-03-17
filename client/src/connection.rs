use std::collections::HashMap;
use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink};
use serde_json::Value;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async};
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, warn};
use url::Url;

use crate::schema::Request;
use crate::listener::{listen_for_responses, ResponseHandler, Stream};
use crate::message_processing::ProcessedMessage;

/// Interface for XTB servers connectors.
#[async_trait]
pub trait XtbConnection {
    /// Send standard command to the server.
    async fn send_command(&mut self, command: &str, payload: Option<Value>) -> Result<ResponsePromise, XtbConnectionError>;
}


#[derive(Debug, Error)]
pub enum XtbConnectionError {
    #[error("Cannot connect to server ({0}")]
    CannotConnect(String),
    #[error("Cannot serialize command payload")]
    SerializationError(serde_json::Error),
    #[error("Cannot send request to the XTB server.")]
    CannotSendRequest(tokio_tungstenite::tungstenite::Error),
}


/// Common implementation of the `XtbConnection` trait.
pub struct BasicXtbConnection {
    sink: SplitSink<Stream, Message>,
    tag_maker: TagMaker,
    promise_state_by_tag: Arc<Mutex<HashMap<String, Arc<Mutex<ResponsePromiseState>>>>>,
    listener_join: JoinHandle<()>,
}


impl BasicXtbConnection {
    /// Create new instance from server url
    pub async fn new(url: Url) -> Result<Self, XtbConnectionError> {
        let host_clone = url.as_str().to_owned();
        let (conn, _) = connect_async(url).await.map_err(|err| {
            error!("Cannot connect to server {}: {:?}", host_clone, err);
            XtbConnectionError::CannotConnect(host_clone)
        })?;

        let (sink, stream) = conn.split();
        let lookup = Arc::new(Mutex::new(HashMap::new()));
        let listener_join = listen_for_responses(stream, BasicConnectionResponseHandler(lookup.clone()));
        let instance = Self {
            sink,
            tag_maker: TagMaker::default(),
            promise_state_by_tag: lookup,
            listener_join
        };
        Ok(instance)
    }

    /// Build a request from command and payload.
    /// Return request and its tag.
    fn build_request(&mut self, command: &str, mut payload: Option<Value>) -> (Request, String) {
        let tag = self.tag_maker.next();

        if let Some(p) = &payload {
            if p.is_null() {
                payload = None;
            }
        }

        let r = Request::default()
            .with_command(command)
            .with_maybe_arguments(payload)
            .with_custom_tag(&tag);
        (r, tag)
    }
}



#[async_trait]
impl XtbConnection for BasicXtbConnection {
    async fn send_command(&mut self, command: &str, payload: Option<Value>) -> Result<ResponsePromise, XtbConnectionError> {
        let (request, tag) = self.build_request(command, payload);
        let request_json = serde_json::to_string(&request).map_err(XtbConnectionError::SerializationError)?;
        let message = Message::Text(request_json);

        let (promise, state) = ResponsePromise::new();
        self.promise_state_by_tag.lock().await.insert(tag, state);
        self.sink.send(message).await.map_err(XtbConnectionError::CannotSendRequest)?;

        Ok(promise)
    }
}


impl Drop for BasicXtbConnection {
    fn drop(&mut self) {
        // Stop the listening task
        self.listener_join.abort();
    }
}


/// Internal state shared between the ResponsePromise and BasicXtbConnection instance.
/// This state is used to deliver response to the consumer.
#[derive(Default, Debug)]
pub struct ResponsePromiseState {
    /// The response.
    ///
    /// * `None` - the response is not ready yet.
    /// * `Some(response)` - the response is ready to be delivered.
    result: Option<Result<ProcessedMessage, XtbConnectionError>>,
    /// If the `ResponsePromise` was palled, the `Waker` is stored here.
    /// When response is set and the waker is set, the waker is called.
    waker: Option<Waker>,
}


impl ResponsePromiseState {
    /// Set response. If a waker is set in the state, it is notified.
    pub fn set_result(&mut self, result: Result<ProcessedMessage, XtbConnectionError>) {
        self.result = Some(result);
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}


/// Handle messages delivered by XTB server
struct BasicConnectionResponseHandler(Arc<Mutex<HashMap<String, Arc<Mutex<ResponsePromiseState>>>>>);

#[async_trait]
impl ResponseHandler for BasicConnectionResponseHandler {
    async fn handle_response(&self, response: ProcessedMessage) {
        let maybe_tag = match &response {
            ProcessedMessage::Response(resp) => resp.custom_tag.as_ref(),
            ProcessedMessage::ErrorResponse(resp) => resp.custom_tag.as_ref(),
        };

        // if there is no tag, continue (the message cannot be routed to consumer)
        let tag = match maybe_tag {
            Some(t) => t,
            _ => {
                warn!("Response has no tag and cannot be routed: {:?}", response);
                return;
            }
        };

        // try to deliver message to its consumer
        if let Some(state) = self.0.lock().await.remove(tag) {
            state.lock().await.set_result(Ok(response));
        }
    }
}


/// Represent promise of a response delivery in a future.
///
/// Implements the `Future` trait and when the future is awaited, it is resolved by response
/// returned from a server. The response is type of `Result<Response, ErrorResponse>`.
#[derive(Debug)]
pub struct ResponsePromise {
    /// Shared internal state. The second "point" is in the source connection.
    state: Arc<Mutex<ResponsePromiseState>>,
}


impl ResponsePromise {
    /// Create new instance and return tuple:
    ///
    /// 1. instance of `Self`
    /// 2. thread safe `ResponsePromiseState` for response delivery.
    pub fn new() -> (Self, Arc<Mutex<ResponsePromiseState>>) {
        let state = ResponsePromiseState::default();
        let wrapped_state = Arc::new(Mutex::new(state));
        (Self { state: wrapped_state.clone() }, wrapped_state)
    }
}


impl Future for ResponsePromise {
    type Output = Result<ProcessedMessage, XtbConnectionError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Try to get the lock
        if let Poll::Ready(mut guard) = pin!(self.state.lock()).poll(cx) {
            // If response is set, return it as `Poll::Ready`
            if let Some(response) = guard.result.take() {
                return Poll::Ready(response);
            }
            // If response is not ready yet, register the waker.
            guard.waker = Some(cx.waker().clone());
        }
        // Wait until response is ready
        Poll::Pending
    }
}


/// Helper struct generating message tags.
///
/// It generates unique tags with prefix "message_" followed by incremented positive integer number.
/// The increment step is 1 and the first number is 1.
///
/// Example of series is: "message_1", "message_2", "message_3", ...
#[derive(Default, Debug)]
struct TagMaker(u64);


impl TagMaker {
    fn next(&mut self) -> String {
        self.0 += 1;
        format!("message_{}", self.0)
    }
}


#[cfg(test)]
mod tests {
    mod response_promise {
        use std::sync::Arc;
        use std::time::Duration;

        use rstest::*;
        use serde_json::to_value;
        use tokio::spawn;
        use tokio::sync::Mutex;
        use tokio::time::sleep;

        use crate::schema::Response;
        use crate::connection::ResponsePromiseState;
        use crate::message_processing::ProcessedMessage;
        use crate::ResponsePromise;

        #[rstest]
        #[case(0)]
        #[case(1)]
        #[case(50)]
        #[case(100)]
        #[timeout(Duration::from_millis(500))]
        #[tokio::test]
        async fn deliver_data(#[case] delay_ms: u64) {
            let (instance, target) = ResponsePromise::new();
            spawn(write_data(target, delay_ms));
            let result = instance.await;
        }

        async fn write_data(target: Arc<Mutex<ResponsePromiseState>>, delay: u64) {
            if delay > 0 {
                sleep(Duration::from_millis(delay)).await;
            }
            let mut lock = target.lock().await;
            let mut response = Response::default();
            response.return_data = Some(to_value(42).unwrap());
            lock.set_result(Ok(ProcessedMessage::Response(response)));
        }
    }

    mod tag_maker {
        use crate::connection::TagMaker;

        #[test]
        fn make_series() {
            let mut maker = TagMaker::default();

            let tag = maker.next();
            assert_eq!(tag, "message_1");
            let tag = maker.next();
            assert_eq!(tag, "message_2");
            let tag = maker.next();
            assert_eq!(tag, "message_3");
        }
    }
}
