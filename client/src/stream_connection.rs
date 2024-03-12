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
use crate::api::{StreamDataMessage, SubscribeRequest, UnsubscribeRequest};

use crate::listener::Stream;

/// Common interface for stream command api of the XTB.
#[async_trait]
pub trait XtbStreamConnection {

    /// Type of message stream returned by the `make_message_stream` method.
    type MessageStream: MessageStream;

    /// Subscribe for data stream from the XTB server
    ///
    /// The `arguments` must be `Value::Object`. Any other variants causes an error
    async fn subscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError>;

    /// Unsubscribe from data stream from the XTB server
    ///
    /// The `arguments` must be `Value::Object`. Any other variants causes an error
    async fn unsubscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError>;

    /// Create message stream builder
    async fn make_message_stream(&mut self, filter: StreamFilter) -> Self::MessageStream;
}


pub struct BasicXtbStreamConnection {
    /// Stream session id used to identify for the stream server
    stream_session_id: String,
    /// Sender of messages used for delivering messages to `MessageStream` implementors
    sender: Sender<StreamDataMessage>,
    /// Sink used for sending messages to the XTB server
    sink: SplitSink<Stream, Message>,
}


impl BasicXtbStreamConnection {
    /// Create new instance of the stream connection.
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

    /// Build message from request and arguments and send it to the server.
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

    /// Check and prepare arguments.
    ///
    /// The arguments must be None or Some(Value::Object). Otherwise, an error is returned.
    fn prepare_arguments(arguments: Option<Value>) -> Result<Option<Map<String, Value>>, XtbStreamConnectionError> {
        match arguments {
            // No arguments are provided
            None => Ok(None),
            // There is arguments object
            Some(Value::Object(obj)) => Ok(Some(obj)),
            // Invalid input data
            _ => Err(XtbStreamConnectionError::InvalidArgumentsType)
        }
    }
}


#[async_trait]
impl XtbStreamConnection for BasicXtbStreamConnection {

    type MessageStream = BasicMessageStream;

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

    async fn make_message_stream(&mut self, filter: StreamFilter) -> Self::MessageStream {
        BasicMessageStream::new(filter, self.sender.subscribe())
    }
}


#[derive(Default)]
pub enum StreamFilter {
    /// Always true
    #[default]
    Always,
    /// Always false
    Never,
    /// Command name must match
    Command(String),
    /// All inner filters must match
    All(Vec<StreamFilter>),
    /// Any inner filter must match
    Any(Vec<StreamFilter>),
    /// Value of field in `data` must match
    /// Return true if and only if the `data` field is type of `Object::Value`, contains key
    /// defined by `name` and the field is equal to `value`.
    FieldValue { name: String, value: Value },
    /// Apply custom filter fn
    Custom(Box<dyn Fn(&StreamDataMessage) -> bool + Send + Sync>)
}


impl StreamFilter {
    /// Return true if the filter match, return false otherwise.
    pub fn test_message(&self, msg: &StreamDataMessage) -> bool {
        match self {
            Self::Always => Self::resolve_always(msg),
            Self::Never => Self::resolve_never(msg),
            Self::Command(cmd) => Self::resolve_command(msg, cmd),
            Self::All(ops) => Self::resolve_all(msg, ops),
            Self::Any(ops) => Self::resolve_any(msg, ops),
            Self::FieldValue {name, value} => Self::resolve_field_value(msg, name, value),
            Self::Custom(cbk) => Self::resolve_custom(msg, cbk),
        }
    }

    /// resolve StreamFilter::Always
    fn resolve_always(msg: &StreamDataMessage) -> bool {
        true
    }

    /// resolve StreamFilter::Never
    fn resolve_never(msg: &StreamDataMessage) -> bool {
        false
    }

    /// resolve StreamFilter::Command
    fn resolve_command(msg: &StreamDataMessage, command: &str) -> bool {
        return msg.command.as_str() == command
    }

    /// resolve StreamFilter::All
    fn resolve_all(msg: &StreamDataMessage, ops: &Vec<StreamFilter>) -> bool {
        ops.iter().all(|f| f.test_message(msg))
    }

    /// resolve StreamFilter::Any
    fn resolve_any(msg: &StreamDataMessage, ops: &Vec<StreamFilter>) -> bool {
        ops.iter().any(|f| f.test_message(msg))
    }

    /// resolve StreamFilter::FieldValue
    fn resolve_field_value(msg: &StreamDataMessage, field_name: &str, field_value: &Value) -> bool {
        match field_value {
            Value::Object(obj) => {
                if let Some(field_content) = obj.get(field_name) {
                    field_content == field_value
                } else {
                    false
                }
            },
            _ => false
        }
    }

    /// resolve StreamFilter::Custom
    fn resolve_custom(msg: &StreamDataMessage, cbk: &Box<dyn Fn(&StreamDataMessage) -> bool + Send + Sync>) -> bool {
        (*cbk)(msg)
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
    async fn next(&mut self) -> Option<StreamDataMessage>;
}


pub struct BasicMessageStream {
    /// The filter for messages
    filter: StreamFilter,
    /// Stream with incoming messages
    stream: Receiver<StreamDataMessage>
}


impl BasicMessageStream {
    /// Create new instance
    pub fn new(filter: StreamFilter, stream: Receiver<StreamDataMessage>) -> Self {
        BasicMessageStream {
            filter,
            stream,
        }
    }
}


#[async_trait]
impl MessageStream for BasicMessageStream {
    async fn next(&mut self) -> Option<StreamDataMessage> {
        while let Some(msg) = self.stream.recv().await.ok() {
            if self.filter.test_message(&msg) {
                return Some(msg)
            }
        }
        None
    }
}
