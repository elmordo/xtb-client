use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::api_errors::XtbErrorCode;


/// Message sent to XTB servers
#[derive(Clone, Default, Debug, Serialize, Setters)]
#[setters(into, prefix = "with_", strip_option)]
pub struct Request {
    /// The command name
    pub command: String,
    /// Data (payload) send with a command
    pub arguments: Option<Value>,
}


/// Response message returned from server when operation succeeds.
#[derive(Clone, Default, Debug, Deserialize)]
pub struct Response {
    /// Response status is always TRUE
    pub status: bool,
    /// This is set only for response to the "login" request
    pub stream_session_id: Option<String>,
    /// Returning data.
    pub return_data: Option<Value>,
}


/// Response message returned from server when operation fails.
#[derive(Clone, Default, Debug, Deserialize)]
pub struct ErrorResponse {
    /// Error response status is always FALSE
    pub status: bool,
    /// The machine-readable error code
    pub error_code: XtbErrorCode,
    /// The human-readable error code
    pub error_descr: Option<String>,
}
