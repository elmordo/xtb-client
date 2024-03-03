use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use async_trait::async_trait;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, connect_async_with_config, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use url::Url;
use crate::api::{ErrorResponse, Request, Response};

#[async_trait]
pub trait XtbConnection {
    async fn send_command(&mut self, command: &str, payload: Option<Value>) -> Result<Response, XtbConnectionError>;
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
    CannotSendRequest(tokio_tungstenite::tungstenite::Error)
}


type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;


pub struct BasicXtbConnection {
    sink: SplitSink<Stream, Message>,
    stream: SplitStream<Stream>,
    stream_session_id: Option<String>,
    tag_maker: TagMaker,
}


impl BasicXtbConnection {
    pub async fn new(host: Url) -> Result<Self, XtbConnectionError> {
        let host_clone = host.as_str().to_owned();
        let (conn, _) = connect_async(host).await.map_err(|_| XtbConnectionError::CannotConnect(host_clone))?;

        let (sink, stream) = conn.split();

        Ok(Self {
            sink,
            stream,
            stream_session_id: None,
            tag_maker: TagMaker::default(),
        })
    }

    fn build_request(&mut self, command: &str, payload: Option<Value>, is_stream: bool) -> Result<Request, XtbConnectionError> {
        let request = Request::default()
            .with_command(command)
            .with_arguments(payload)
            .with_custom_tag(self.tag_maker.next());
        if is_stream {
            let ssi = self.stream_session_id.clone().ok_or(XtbConnectionError::NoStreamSessionId)?;
            Ok(request.with_stream_session_id(ssi))
        } else {
            Ok(request)
        }
    }
}


#[async_trait]
impl XtbConnection for BasicXtbConnection {
    async fn send_command(&mut self, command: &str, payload: Option<Value>) -> Result<Response, XtbConnectionError> {
        let request = self.build_request(command, payload, false)?;
        let request_json = serde_json::to_string(&request).map_err(|err| XtbConnectionError::SerializationError(err))?;
        let message = Message::Text(request_json);
        self.sink.send(message).await.map_err(|err| XtbConnectionError::CannotSendRequest(err))?;
        todo!()
    }
}


type PromisedResponseData = Option<Result<Response, ErrorResponse>>;


#[derive(Debug)]
pub struct PromisedResponse {
    response: Arc<RwLock<PromisedResponseData>>,
}


impl PromisedResponse {
    pub fn new() -> (Self, Arc<RwLock<PromisedResponseData>>) {
        let data = Arc::new(RwLock::new(PromisedResponseData::default()));
        (Self { response: data.clone() }, data)
    }
}


impl Future for PromisedResponse {
    type Output = Result<Response, ErrorResponse>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}


/// Helper struct generating message tags.
#[derive(Default, Debug)]
struct TagMaker(u64);


impl TagMaker {
    fn next(&mut self) -> String {
        self.0 += 1;
        format!("message_{}", self.0.to_string())
    }
}
