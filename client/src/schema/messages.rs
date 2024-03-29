use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::schema::api_errors::XtbErrorCode;


/// Message sent to XTB servers
#[derive(Clone, Default, Debug, Serialize, Setters)]
#[serde(rename_all = "camelCase")]
#[setters(into, prefix = "with_", strip_option)]
pub struct Request {
    /// The command name
    pub command: String,
    /// Data (payload) send with a command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
    /// Custom tag for message identification
    pub custom_tag: Option<String>,
}


impl Request {

    /// Correctly set arguments.
    ///
    /// The arguments can be:
    ///
    /// * None - set payload to None
    /// * Some(Value::Null) - set payload to None
    /// * Some(Value::Object) - set payload to given value
    ///
    /// # Panics
    ///
    /// Any other payload configuration than supported one
    pub fn with_maybe_arguments(mut self, arguments: Option<Value>) -> Self {
        match arguments {
            None | Some(Value::Null) => self.arguments = None,
            Some(Value::Object(obj)) => self.arguments = Some(Value::Object(obj)),
            _ => panic!("Unsupported argument type. RTFM")
        }
        self
    }
}


/// Response message returned from server when operation succeeds.
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Response status is always TRUE
    pub status: bool,
    /// This is set only for response to the "login" request
    pub stream_session_id: Option<String>,
    /// Returning data.
    pub return_data: Option<Value>,
    /// Custom tag from original message
    pub custom_tag: Option<String>,
}


/// Subscribe for stream of data
///
/// # Note
///
/// This struct does not carry any arguments itself. The arguments are merged into serialized object
/// before the send operation is done.
#[derive(Clone, Default, Debug, Serialize, Setters)]
#[serde(rename_all = "camelCase")]
#[setters(into, prefix = "with_", strip_option)]
pub struct SubscribeRequest {
    /// The command name
    pub command: String,
    /// The stream session id (identify the connection)
    pub stream_session_id: String
}


/// Unsubscribe from stream of data
///
/// # Note
///
/// This struct does not carry any arguments itself. The arguments are merged into serialized object
/// before the send operation is done.
#[derive(Clone, Default, Debug, Serialize, Setters)]
#[serde(rename_all = "camelCase")]
#[setters(into, prefix = "with_", strip_option)]
pub struct UnsubscribeRequest {
    /// Command to be unsubscribed from
    pub command: String,
}


/// Data stream item representation
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamDataMessage {
    /// Source command
    pub command: String,
    /// Payload
    pub data: Value
}


/// Response message returned from server when operation fails.
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// Error response status is always FALSE
    pub status: bool,
    /// The machine-readable error code
    pub error_code: XtbErrorCode,
    /// The human-readable error code
    pub error_descr: Option<String>,
    /// Custom tag from original message
    pub custom_tag: Option<String>,
}


#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::{from_str, to_value, Value};
    use crate::schema::Request;

    #[rstest]
    #[case(Request::default().with_command("command").with_arguments("argument").with_custom_tag("tag"), "{\"command\": \"command\", \"arguments\": \"argument\", \"customTag\": \"tag\"}")]
    fn serialize_request(#[case] request: Request, #[case] expected_json: &str) {
        let request_value = to_value(request).unwrap();
        let expected_value: Value = from_str(expected_json).unwrap();
        assert_eq!(request_value, expected_value)
    }
}
