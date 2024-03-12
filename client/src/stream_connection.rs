use async_trait::async_trait;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::{Map, to_string, to_value, Value};
use thiserror::Error;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use crate::api::{SubscribeRequest, UnsubscribeRequest};

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
    async fn make_message_stream(&mut self, filter: StreamFilter) -> BasicMessageStream;
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

    async fn assemble_and_send<T: Serialize>(&mut self, request: T, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError> {
        let mut obj = to_value(request).map_err(|err| XtbStreamConnectionError::SerializationFailed(err))?;
        let prepared_arguments = Self::prepare_arguments(arguments)?;

        if let Some(mut prepared_obj) = prepared_arguments {
            // unwrap is safe here.
            obj.as_object_mut().unwrap().append(&mut prepared_obj);
        }
        let serialized = to_string(&obj).map_err(|err| XtbStreamConnectionError::SerializationFailed(err))?;
        let message = Message::text(serialized);
        self.sink.send(message).await.map_err(|err| XtbStreamConnectionError::CannotSend(err))
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
        let request = SubscribeRequest::default()
            .with_command(command)
            .with_stream_session_id(&self.stream_session_id);
        self.assemble_and_send(request, arguments).await
    }

    async fn unsubscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError> {
        let request = UnsubscribeRequest::default().with_command(command);
        self.assemble_and_send(request, arguments).await
    }

    async fn make_message_stream(&mut self, filter: StreamFilter) -> BasicMessageStream {
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


impl StreamFilter {
    pub fn test_message(&self, msg: &ProcessedMessage) -> bool {
        todo!()
    }

    fn resolve_noop(msg: &ProcessedMessage) -> bool {
        true
    }

    fn resolve_command(msg: &ProcessedMessage, command: &str) -> bool {
        todo!()
    }
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


#[async_trait]
pub trait MessageStream {
    /// Get next message from stream
    ///
    /// # Returns
    ///
    /// `Some(x)` - next message in stream
    /// `None` - there is no more message
    async fn next(&mut self) -> Option<ProcessedMessage>;
}


pub struct BasicMessageStream {
    filter: StreamFilter,
    stream: Receiver<ProcessedMessage>
}


#[async_trait]
impl MessageStream for BasicMessageStream {
    async fn next(&mut self) -> Option<ProcessedMessage> {
        while let Some(msg) = self.stream.recv().await.ok() {
            if self.filter.test_message(&msg) {
                return Some(msg)
            }
        }
        None
    }
}
