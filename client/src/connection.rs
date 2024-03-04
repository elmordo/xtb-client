use std::collections::HashMap;
use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use serde_json::{from_str, from_value, Value};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use tracing::error;
use url::Url;

use crate::api::{ErrorResponse, Request, Response};

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
    #[error("The stream session id is not set. Maybe login was not performed?")]
    NoStreamSessionId,
    #[error("Cannot serialize command payload")]
    SerializationError(serde_json::Error),
    #[error("Cannot send request to the XTB server.")]
    CannotSendRequest(tokio_tungstenite::tungstenite::Error),
}


/// Helper type making variable and field declaration shorter.
type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;


/// Common implementation of the `XtbConnection` trait.
pub struct BasicXtbConnection {
    sink: SplitSink<Stream, Message>,
    stream_session_id: Option<String>,
    tag_maker: TagMaker,
    promise_state_by_tag: Arc<Mutex<HashMap<String, Arc<Mutex<ResponsePromiseState>>>>>
}


impl BasicXtbConnection {
    /// Create new instance from server url
    pub async fn new(host: Url) -> Result<Self, XtbConnectionError> {
        let host_clone = host.as_str().to_owned();
        let (conn, _) = connect_async(host).await.map_err(|_| XtbConnectionError::CannotConnect(host_clone))?;

        let (sink, stream) = conn.split();

        let instance = Self {
            sink,
            stream_session_id: None,
            tag_maker: TagMaker::default(),
            promise_state_by_tag: Arc::new(Mutex::new(HashMap::new())),
        };
        instance.run_listener(stream).await;
        Ok(instance)
    }

    fn build_request(&mut self, command: &str, payload: Option<Value>, is_stream: bool) -> Result<(Request, String), XtbConnectionError> {
        let tag = self.tag_maker.next();

        let request = Request::default()
            .with_command(command)
            .with_arguments(payload)
            .with_custom_tag(&tag);
        if is_stream {
            let ssi = self.stream_session_id.clone().ok_or(XtbConnectionError::NoStreamSessionId)?;
            Ok((request.with_stream_session_id(ssi), tag))
        } else {
            Ok((request, tag))
        }
    }

    async fn run_listener(&self, mut stream: SplitStream<Stream>) {
        let lookup = self.promise_state_by_tag.clone();
        spawn(async move {
            while let Some(message) = stream.next().await {
                let (response, tag) = match process_message(message) {
                    Some((r, t)) => (r, t),
                    None => continue,
                };

                if let Some(state) = lookup.lock().await.remove(&tag) {
                    state.lock().await.set_response(response);
                }
            }
        });
    }
}


fn process_message(message: Result<Message, tokio_tungstenite::tungstenite::Error>) -> Option<(Result<Response, ErrorResponse>, String)> {
    let message = match message {
        Ok(msg) => msg,
        Err(err) => {
            error!("Error when receiving message: {:?}", err);
            return None;
        }
    };

    let text_content = message.to_text().ok()?;
    let v: Value = from_str(text_content).ok()?;
    let status = v.as_object()?.get("status")?.as_bool()?;
    let tag = v.as_object()?.get("customTag")?.as_str()?.to_owned();

    let result = if status {
        Ok(from_value(v).ok()?)
    } else {
        Err(from_value(v).ok()?)
    };
    Some((result, tag))
}



#[async_trait]
impl XtbConnection for BasicXtbConnection {
    async fn send_command(&mut self, command: &str, payload: Option<Value>) -> Result<ResponsePromise, XtbConnectionError> {
        let (request, tag) = self.build_request(command, payload, false)?;
        let request_json = serde_json::to_string(&request).map_err(|err| XtbConnectionError::SerializationError(err))?;
        let message = Message::Text(request_json);

        let (promise, state) = ResponsePromise::new();
        self.promise_state_by_tag.lock().await.insert(tag, state);
        self.sink.send(message).await.map_err(|err| XtbConnectionError::CannotSendRequest(err))?;

        Ok(promise)
    }
}


/// Internal state shared between the ResponsePromise and BasicXtbConnection instance.
/// This state is used to deliver response to the consumer.
#[derive(Debug)]
struct ResponsePromiseState {
    /// The response.
    ///
    /// * `None` - the response is not ready yet.
    /// * `Some(response)` - the response is ready to be delivered.
    response: Option<Result<Response, ErrorResponse>>,
    /// If the `ResponsePromise` was palled, the `Waker` is stored here.
    /// When response is set and the waker is set, the waker is called.
    waker: Option<Waker>,
}


impl ResponsePromiseState {
    /// Create new instance waiting for a response
    pub fn new() -> Self {
        Self { response: None, waker: None }
    }

    /// Set response. If a waker is set in the state, it is notified.
    pub fn set_response(&mut self, response: Result<Response, ErrorResponse>) {
        self.response = Some(response);
        if let Some(waker) = self.waker.take() {
            waker.wake();
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
    fn new() -> (Self, Arc<Mutex<ResponsePromiseState>>) {
        let state = ResponsePromiseState::new();
        let wrapped_state = Arc::new(Mutex::new(state));
        (Self { state: wrapped_state.clone() }, wrapped_state)
    }
}


impl Future for ResponsePromise {
    type Output = Result<Response, ErrorResponse>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Try to get the lock
        if let Poll::Ready(mut guard) = pin!(self.state.lock()).poll(cx) {
            // If response is set, return it as `Poll::Ready`
            if let Some(response) = guard.response.take() {
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
        format!("message_{}", self.0.to_string())
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

        use crate::api::Response;
        use crate::connection::ResponsePromiseState;
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
            lock.set_response(Ok(response));
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
