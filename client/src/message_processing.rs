use serde_json::{from_str, from_value, Value};
use thiserror::Error;
use tokio_tungstenite::tungstenite::Message;
use tracing::error;

use crate::api::{ErrorResponse, Response};



#[derive(Debug, Error)]
pub enum MessageProcessingError {
    #[error("Cannot deserialize response payload")]
    DeserializationError(serde_json::Error),
    #[error("A response format is malformed: {0}")]
    MalformedResponse(String),
    #[error("Expected to receive text data but something wrong was received.")]
    ReceivedInvalidData(tokio_tungstenite::tungstenite::Error),
}


#[derive(Debug, Clone)]
pub enum ProcessedMessage {
    Response(Response),
    ErrorResponse(ErrorResponse),
}

impl ProcessedMessage {
    pub fn unwrap(self) -> Response {
        match self {
            Self::Response(r) => r,
            _ => panic!("Cannot unwrap response")
        }
    }
    pub fn unwrap_err(self) -> ErrorResponse {
        match self {
            Self::ErrorResponse(r) => r,
            _ => panic!("Cannot unwrap error response")
        }
    }
}


/// Get received message from tungstenite and tries to construct a response
pub fn process_message(message: Message) -> Result<ProcessedMessage, MessageProcessingError> {
    // deconstruct and deserialize data received from a server
    let text_content = message.to_text().map_err(MessageProcessingError::ReceivedInvalidData)?;
    let value = from_str(text_content).map_err(MessageProcessingError::DeserializationError)?;
    process_message_value(value)
}


/// Get `Value` read from response payload and tries to construct Response or ErrorResponse
fn process_message_value(value: Value) -> Result<ProcessedMessage, MessageProcessingError> {
    // read s `status` first to determine response type
    let status = value
        .as_object().ok_or_else(|| MessageProcessingError::MalformedResponse("Value is not object".to_string()))?
        .get("status").ok_or_else(|| MessageProcessingError::MalformedResponse("The response 'status' field is missing".to_owned()))?
        .as_bool().ok_or_else(|| MessageProcessingError::MalformedResponse("The response 'status' field is not boolean".to_owned()))?;

    if status {
        let response = from_value(value).map_err(MessageProcessingError::DeserializationError)?;
        Ok(ProcessedMessage::Response(response))
    } else {
        let response = from_value(value).map_err(MessageProcessingError::DeserializationError)?;
        Ok(ProcessedMessage::ErrorResponse(response))
    }
}


#[cfg(test)]
mod tests {

    mod process_message_and_process_message_value {
        use rstest::rstest;
        use serde_json::{from_str, Value};
        use tokio_tungstenite::tungstenite::Message;

        use crate::api::XtbErrorCode;
        use crate::message_processing::MessageProcessingError;

        #[rstest]
        #[case(r#"{"returnData": {"field": 12}, "status": true, "customTag": "myTag"}"#, "myTag", true)]
        #[case(r#"{"status": true, "customTag": "myTag"}"#, "myTag", false)]
        fn process_valid_response(#[case] payload: &str, #[case] expected_tag: &str, #[case] has_data: bool) {
            let msg = Message::text(payload);
            let response = crate::message_processing::process_message(msg).unwrap().unwrap();
            let tag = response.custom_tag.clone().unwrap();
            let expected_data = from_str::<Value>(payload).unwrap().as_object().unwrap().get("returnData").map(|v| v.to_owned());

            if has_data {
                assert!(response.return_data.is_some());
            } else {
                assert!(response.return_data.is_none());
            }

            assert!(response.status);
            assert_eq!(&tag, expected_tag);
            assert_eq!(response.return_data, expected_data);
        }

        #[rstest]
        #[case(r#"{"status": false, "errorCode": "BE001", "errorDescr": "BE001", "customTag": "myTag"}"#, "myTag", XtbErrorCode::BE001)]
        fn process_valid_error_response(#[case] payload: &str, #[case] expected_tag: &str, #[case] expected_error: XtbErrorCode) {
            let msg = Message::text(payload);
            let error_response = crate::message_processing::process_message(msg).unwrap().unwrap_err();

            let tag = error_response.custom_tag.unwrap();
            assert_eq!(&tag, expected_tag);
            assert_eq!(error_response.error_code, expected_error);
            assert!(error_response.error_descr.unwrap().len() > 0);
        }

        #[test]
        fn process_invalid_message_not_json() {
            let msg = Message::text(r#"{foo bar}"#);
            let err = crate::message_processing::process_message(msg).unwrap_err();
            match err {
                MessageProcessingError::DeserializationError(_) => (),
                _ => panic!("Expected XtbConnectionError::DeserializationError")
            };
        }

        #[rstest]
        #[case(r#"{"errorCode": "BE001", "errorDescr": "BE001", "customTag": "myTag"}"#)]
        fn process_invalid_message_malformed_response(#[case] payload: &str) {
            let msg = Message::text(payload);
            let err = crate::message_processing::process_message(msg).unwrap_err();
            match err {
                MessageProcessingError::MalformedResponse(_) => (),
                _ => panic!("Expected XtbConnectionError::MalformedResponse, but {:?}", err)
            };
        }
    }
}
