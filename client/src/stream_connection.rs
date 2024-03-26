use async_trait::async_trait;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::{Map, to_string, to_value, Value};
use thiserror::Error;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info};
use url::Url;
use crate::schema::{StreamDataMessage, SubscribeRequest, UnsubscribeRequest};

use crate::listener::{listen_for_stream_data, Stream, StreamDataMessageHandler};

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
    /// The `arguments` value must be `Value::Object` or `Value::Null`. Any other variants causes an error
    async fn unsubscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError>;

    /// Create message stream builder
    async fn make_message_stream(&mut self, filter: DataMessageFilter) -> Self::MessageStream;
}


#[derive(Debug)]
pub struct BasicXtbStreamConnection {
    /// Stream session id used to identify for the stream server
    stream_session_id: String,
    /// Sender of messages used for delivering messages to `MessageStream` implementors
    sender: Sender<StreamDataMessage>,
    /// Sink used for sending messages to the XTB server
    sink: SplitSink<Stream, Message>,
    /// Handle used for join of listening task
    listener_join: JoinHandle<()>,
}


impl BasicXtbStreamConnection {
    /// Create new instance of the stream connection.
    pub async fn new(url: Url, stream_session_id: String) -> Result<Self, XtbStreamConnectionError> {
        let (sender, _) = channel(64usize);
        let host_clone = url.as_str().to_owned();
        let (conn, _) = connect_async(url).await.map_err(|_| XtbStreamConnectionError::CannotConnect(host_clone))?;
        let (sink, stream) = conn.split();
        let listener_join = listen_for_stream_data(stream, MessageHandler::new(sender.clone()));
        Ok(Self {
            stream_session_id,
            sender,
            sink,
            listener_join,
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
    /// The arguments must be None or Some(Value::Object) or Some(Value::Null). Otherwise, an error is returned.
    fn prepare_arguments(arguments: Option<Value>) -> Result<Option<Map<String, Value>>, XtbStreamConnectionError> {
        match arguments {
            // No arguments are provided
            None => Ok(None),
            // There is arguments object
            Some(Value::Object(obj)) => Ok(Some(obj)),
            Some(Value::Null) => Ok(None),
            // Invalid input data
            _ => Err(XtbStreamConnectionError::InvalidArgumentsType)
        }
    }
}


impl Drop for BasicXtbStreamConnection {
    fn drop(&mut self) {
        self.listener_join.abort();
    }
}


#[async_trait]
impl XtbStreamConnection for BasicXtbStreamConnection {
    type MessageStream = BasicMessageStream;

    async fn subscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError> {
        let request = SubscribeRequest::default()
            .with_command(command)
            .with_stream_session_id(&self.stream_session_id);
        info!("Subscribing for {command}");
        debug!("Subscription arguments are {arguments:?}");
        self.assemble_and_send(request, arguments).await
    }

    async fn unsubscribe(&mut self, command: &str, arguments: Option<Value>) -> Result<(), XtbStreamConnectionError> {
        let request = UnsubscribeRequest::default().with_command(command);
        info!("Unsubscribing from {command}");
        debug!("Unsubscription arguments are {arguments:?}");
        self.assemble_and_send(request, arguments).await
    }

    async fn make_message_stream(&mut self, filter: DataMessageFilter) -> Self::MessageStream {
        BasicMessageStream::new(filter, self.sender.subscribe())
    }
}


/// Handle incoming data messages from stream
struct MessageHandler {
    /// Broadcast sender for messages
    sender: Sender<StreamDataMessage>,
}


impl MessageHandler {
    /// Create new instance of the MessageHandler
    pub fn new(sender: Sender<StreamDataMessage>) -> Self {
        Self { sender }
    }
}


#[async_trait]
impl StreamDataMessageHandler for MessageHandler {
    async fn handle_message(&self, message: StreamDataMessage) {
        let cmd = message.command.to_owned();
        info!("Handling incoming message {cmd}");
        debug!("Incoming message: {message:?}");
        match self.sender.send(message) {
            Err(err) => error!("Cannot broadcast message: {}", err),
            _ => debug!("Message {cmd} was broadcast to the {} receivers", self.sender.len())
        }
    }
}


#[derive(Default)]
pub enum DataMessageFilter {
    /// Always true
    #[default]
    Always,
    /// Always false
    Never,
    /// Command name must match
    Command(String),
    /// Value of field in `data` must match
    /// Return true if and only if the `data` field is type of `Object::Value`, contains key
    /// defined by `name` and the field is equal to `value`.
    FieldValue { name: String, value: Value },
    /// Apply custom filter fn
    Custom(Box<dyn Fn(&StreamDataMessage) -> bool + Send + Sync>),
    /// All inner filters must match. If list of predicates is empty, return true.
    All(Vec<DataMessageFilter>),
    /// Any inner filter must match. If list of predicates is empty, return false
    Any(Vec<DataMessageFilter>),
}


impl DataMessageFilter {
    /// Return true if the filter match, return false otherwise.
    pub fn test_message(&self, msg: &StreamDataMessage) -> bool {
        match self {
            Self::Always => Self::resolve_always(msg),
            Self::Never => Self::resolve_never(msg),
            Self::Command(cmd) => Self::resolve_command(msg, cmd),
            Self::All(ops) => Self::resolve_all(msg, ops),
            Self::Any(ops) => Self::resolve_any(msg, ops),
            Self::FieldValue { name, value } => Self::resolve_field_value(msg, name, value),
            Self::Custom(cbk) => Self::resolve_custom(msg, cbk),
        }
    }

    /// resolve StreamFilter::Always
    fn resolve_always(_: &StreamDataMessage) -> bool {
        true
    }

    /// resolve StreamFilter::Never
    fn resolve_never(_: &StreamDataMessage) -> bool {
        false
    }

    /// resolve StreamFilter::Command
    fn resolve_command(msg: &StreamDataMessage, command: &str) -> bool {
        return msg.command.as_str() == command;
    }

    /// resolve StreamFilter::All
    fn resolve_all(msg: &StreamDataMessage, ops: &Vec<DataMessageFilter>) -> bool {
        ops.iter().all(|f| f.test_message(msg))
    }

    /// resolve StreamFilter::Any
    fn resolve_any(msg: &StreamDataMessage, ops: &Vec<DataMessageFilter>) -> bool {
        ops.iter().any(|f| f.test_message(msg))
    }

    /// resolve StreamFilter::FieldValue
    fn resolve_field_value(msg: &StreamDataMessage, field_name: &str, field_value: &Value) -> bool {
        match &msg.data {
            Value::Object(data_obj) => {
                if let Some(field_content) = data_obj.get(field_name) {
                    field_content == field_value
                } else {
                    false
                }
            }
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
    filter: DataMessageFilter,
    /// Stream with incoming messages
    stream: Receiver<StreamDataMessage>,
}


impl BasicMessageStream {
    /// Create new instance
    pub fn new(filter: DataMessageFilter, stream: Receiver<StreamDataMessage>) -> Self {
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
                return Some(msg);
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    mod data_message_filter {
        use rstest::rstest;
        use serde_json::{from_str, Value};
        use crate::schema::StreamDataMessage;
        use crate::DataMessageFilter;

        #[test]
        fn always() {
            let msg = StreamDataMessage::default();
            assert!(DataMessageFilter::Always.test_message(&msg));
        }

        #[test]
        fn never() {
            let msg = StreamDataMessage::default();
            assert!(!DataMessageFilter::Never.test_message(&msg));
        }

        #[rstest]
        #[case("command", true)]
        #[case("other_command", false)]
        fn command(#[case] cmd: &str, #[case] expected_result: bool) {
            let msg = StreamDataMessage { command: "command".to_string(), data: Value::Null };
            assert_eq!(DataMessageFilter::Command(cmd.to_string()).test_message(&msg), expected_result);
        }

        #[rstest]
        #[case(vec ! [], true)]
        #[case(vec ! [DataMessageFilter::Always], true)]
        #[case(vec ! [DataMessageFilter::Never], false)]
        #[case(vec ! [DataMessageFilter::Command("command".to_string()), DataMessageFilter::Always], true)]
        #[case(vec ! [DataMessageFilter::Command("command".to_string()), DataMessageFilter::Never], false)]
        #[case(vec ! [DataMessageFilter::Command("other_command".to_string()), DataMessageFilter::Never], false)]
        #[case(vec ! [DataMessageFilter::Command("other_command".to_string()), DataMessageFilter::Always], false)]
        fn all(#[case] filters: Vec<DataMessageFilter>, #[case] expected_result: bool) {
            let msg = StreamDataMessage { command: "command".to_owned(), data: Value::Null };
            let f = DataMessageFilter::All(filters);
            assert_eq!(f.test_message(&msg), expected_result);
        }

        #[rstest]
        #[case(vec ! [], false)]
        #[case(vec ! [DataMessageFilter::Always], true)]
        #[case(vec ! [DataMessageFilter::Never], false)]
        #[case(vec ! [DataMessageFilter::Command("command".to_string()), DataMessageFilter::Always], true)]
        #[case(vec ! [DataMessageFilter::Command("command".to_string()), DataMessageFilter::Never], true)]
        #[case(vec ! [DataMessageFilter::Command("other_command".to_string()), DataMessageFilter::Never], false)]
        #[case(vec ! [DataMessageFilter::Command("other_command".to_string()), DataMessageFilter::Always], true)]
        fn any(#[case] filters: Vec<DataMessageFilter>, #[case] expected_result: bool) {
            let msg = StreamDataMessage { command: "command".to_owned(), data: Value::Null };
            let f = DataMessageFilter::Any(filters);
            assert_eq!(f.test_message(&msg), expected_result);
        }

        #[rstest]
        #[case(r#"{"field": "value"}"#, true)]
        #[case(r#"{"field": 10}"#, false)]
        #[case(r#"{"other_field": 10}"#, false)]
        #[case(r#"null"#, false)]
        fn filed_value(#[case] source_data: &str, #[case] expected_value: bool) {
            let data: Value = from_str(source_data).unwrap();
            let msg = StreamDataMessage { data, command: "".to_owned() };
            let f = DataMessageFilter::FieldValue { name: "field".to_owned(), value: Value::String("value".to_owned()) };
            assert_eq!(f.test_message(&msg), expected_value)
        }

        #[test]
        fn custom_true() {
            let msg = StreamDataMessage::default();
            let f = DataMessageFilter::Custom(Box::new(|msg| true));
            assert_eq!(f.test_message(&msg), true)
        }

        #[test]
        fn custom_false() {
            let msg = StreamDataMessage::default();
            let f = DataMessageFilter::Custom(Box::new(|msg| false));
            assert_eq!(f.test_message(&msg), false)
        }
    }
}
