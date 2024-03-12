use async_trait::async_trait;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use serde_json::{Map, Value};
use thiserror::Error;
use tokio::sync::broadcast::{channel, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use crate::listener::Stream;
use crate::message_processing::{ProcessedMessage};

/// Common interface for stream command api of the XTB.
#[async_trait]
pub trait XtbStreamConnection {
    /// Subscribe for data stream from the XTB server
    ///
    /// The `arguments` must be `Value::Object`. Any other variants causes an error
    async fn subscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError>;

    /// Unsubscribe from data stream from the XTB server
    ///
    /// The `arguments` must be `Value::Object`. Any other variants causes an error
    async fn unsubscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError>;

    /// Create message stream builder
    async fn make_message_stream(&mut self, filter: StreamFilter) -> MessageStream;
}


pub struct BasicXtbStreamConnection {
    /// Stream session id used to identify for the stream server
    stream_session_id: String,
    sender: Sender<ProcessedMessage>,
    /// Sink used for sending messages to the XTB server
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

    async fn build_and_send(&mut self, command: &str, arguments: Option<Value>, include_stream_id: bool) -> Result<(), XtbStreamConnectionError> {
        let arguments_obj = Self::prepare_arguments(arguments)?;
        let message = self.build_stream_message(command.to_owned(), include_stream_id, arguments_obj)?;
        self.sink.send(message).await.map_err(|err| XtbStreamConnectionError::CannotSend(err))
    }

    /// Build message for subscription
    fn build_stream_message(&self, command: String, include_stream_id: bool, payload: Option<Map<String, Value>>) -> Result<Message, XtbStreamConnectionError> {
        let mut content = Map::new();
        content.insert("command".to_string(), Value::String(command.to_string()));
        if include_stream_id {
            content.insert("streamSessionId".to_string(), Value::String(self.stream_session_id.clone()));
        }
        if let Some(mut payload_data) = payload {
            content.append(&mut payload_data);
        }
        Ok(Message::Text(serde_json::to_string(&Value::Object(content)).map_err(|err| XtbStreamConnectionError::SerializationFailed(err))?))
    }

    fn prepare_arguments(arguments: Option<Value>) -> Result<Option<Map<String, Value>>, XtbStreamConnectionError> {
        match arguments {
            None => Ok(None),
            Some(Value::Object(obj)) => Ok(Some(obj)),
            _ => Err(XtbStreamConnectionError::InvalidArgumentsType)
        }
    }
}


#[async_trait]
impl XtbStreamConnection for BasicXtbStreamConnection {
    async fn subscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError> {
        self.build_and_send(command, arguments, true).await
    }

    async fn unsubscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError> {
        self.build_and_send(command, arguments, false).await
    }

    async fn make_message_stream(&mut self, filter: StreamFilter) -> MessageStream {
        todo!()
    }
}


#[derive(Default)]
pub enum StreamFilter {
    #[default]
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
    #[error("Cannot send message")]
    CannotSend(tokio_tungstenite::tungstenite::Error),
    #[error("Cannot serialize data")]
    SerializationFailed(serde_json::Error),
    #[error("Only Value::Object can be used for the arguments")]
    InvalidArgumentsType,
}


pub struct MessageStream {}
