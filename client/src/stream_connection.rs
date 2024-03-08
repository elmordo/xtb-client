use async_trait::async_trait;
use futures_util::stream::SplitSink;
use futures_util::StreamExt;
use serde_json::Value;
use thiserror::Error;
use tokio::sync::broadcast::{channel, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use crate::listener::Stream;
use crate::message_processing::ProcessedMessage;

/// Common interface for stream command api of the XTB.
#[async_trait]
pub trait XtbStreamConnection {
    /// Subscribe for data stream from the XTB server
    ///
    /// The `arguments` must be `Value::Null` or `Value::Object`. Any other variants causes an error
    async fn subscribe(&mut self, command: &str, arguments: Value) -> Result<(), XtbStreamConnectionError>;

    /// Unsubscribe from data stream from the XTB server
    ///
    /// The `arguments` must be `Value::Null` or `Value::Object`. Any other variants causes an error
    async fn unsubscribe(&mut self, command: &str, arguments: Value) -> Result<(), XtbStreamConnectionError>;

    /// Create message stream builder
    async fn make_message_stream(&mut self, filter: StreamFilter) -> MessageStream;
}


pub struct BasicXtbStreamConnection {
    stream_session_id: String,
    sender: Sender<ProcessedMessage>,
    sink: SplitSink<Stream, Message>,
}


impl BasicXtbStreamConnection {
    pub async fn new(url: Url, stream_session_id: String) -> Result<Self, XtbStreamConnectionError> {
        let (sender, _) = channel(64usize);
        let host_clone = url.as_str().to_owned();
        let (conn, _) = connect_async(url).await.map_err(|_| XtbStreamConnectionError::CannotConnect(host_clone))?;
        let (sink, stream) = conn.split();
        Ok(Self {
            stream_session_id,
            sender,
            sink
        })
    }
}


#[async_trait]
impl XtbStreamConnection for BasicXtbStreamConnection {
    async fn subscribe(&mut self, command: &str, arguments: Value) -> Result<(), XtbStreamConnectionError> {
        todo!()
    }

    async fn unsubscribe(&mut self, command: &str, arguments: Value) -> Result<(), XtbStreamConnectionError> {
        todo!()
    }

    async fn make_message_stream(&mut self, filter: StreamFilter) -> MessageStream {
        todo!()
    }
}


pub enum StreamFilter {
    Noop,
    Command(String),
    And(Vec<StreamFilter>),
    Or(Vec<StreamFilter>),
    FieldValue { name: String, value: Value },
    Custom(Box<dyn Fn(&ProcessedMessage) -> bool + Send + Sync>)
}


#[derive(Debug, Error)]
pub enum XtbStreamConnectionError {
    #[error("Cannot connect to server ({0}")]
    CannotConnect(String),
}


struct MessageStream {}
