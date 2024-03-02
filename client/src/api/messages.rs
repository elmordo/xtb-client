use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::api_errors::XtbErrorCode;


/// Message sent to XTB servers
#[derive(Clone, Default, Debug, Serialize, Setters)]
#[serde(rename_all = "camelCase")]
#[setters(into, prefix = "with_", strip_option)]
pub struct Request {
    /// The command name
    pub command: String,
    /// Used for stream api commands
    pub stream_session_id: Option<String>,
    /// Data (payload) send with a command
    pub arguments: Option<Value>,
    /// Custom tag for message identification
    pub custom_tag: Option<String>,
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
    use crate::api::Request;

    #[rstest]
    #[case(Request::default().with_command("command").with_arguments("argument").with_stream_session_id("sess_id").with_custom_tag("tag"), "{\"command\": \"command\", \"arguments\": \"argument\", \"streamSessionId\": \"sess_id\", \"customTag\": \"tag\"}")]
    fn serialize_request(#[case] request: Request, #[case] expected_json: &str) {
        let request_value = to_value(request).unwrap();
        let expected_value: Value = from_str(expected_json).unwrap();
        assert_eq!(request_value, expected_value)
    }
}
