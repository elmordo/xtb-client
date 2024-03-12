use std::str::FromStr;

use derive_setters::Setters;
use serde_json::to_value;
use thiserror::Error;
use url::Url;

use crate::{BasicXtbConnection, BasicXtbStreamConnection, XtbConnection, XtbConnectionError, XtbStreamConnectionError};
use crate::message_processing::ProcessedMessage;
use crate::schema::LoginRequest;

#[derive(Default, Setters)]
#[setters(into, prefix = "with_", strip_option)]
pub struct XtbClientBuilder {
    api_url: Option<String>,
    stream_api_url: Option<String>,
    app_id: Option<String>,
    app_name: Option<String>,
}


impl XtbClientBuilder {
    pub fn new(api_url: &str, stream_api_url: &str) -> Self {
        XtbClientBuilder {
            api_url: Some(api_url.to_string()),
            stream_api_url: Some(stream_api_url.to_string()),
            app_id: None,
            app_name: None,
        }
    }

    pub async fn build(self, user_id: &str, password: &str) -> Result<XtbClient, XtbClientBuilderError> {
        let api_url = Self::make_url(self.api_url)?;
        let stream_api_url = Self::make_url(self.stream_api_url)?;

        // create connection and perform login
        let mut connection = BasicXtbConnection::new(api_url).await.map_err(|err| XtbClientBuilderError::CannotMakeConnection(err))?;
        let mut login_request = LoginRequest::default().with_user_id(user_id).with_password(password);

        if let Some(app_id) = self.app_id {
            login_request = login_request.with_app_id(app_id);
        }
        if let Some(app_name) = self.app_name {
            login_request = login_request.with_app_name(app_name);
        }

        let login_request_value = to_value(login_request).map_err(|err| XtbClientBuilderError::UnexpectedError(format!("{:?}", err)))?;

        let response = connection
            .send_command("login", Some(login_request_value)).await
            .map_err(|err| XtbClientBuilderError::UnexpectedError(format!("{:?}", err)))?.await
            .map_err(|err| XtbClientBuilderError::UnexpectedError(format!("{:?}", err)))?;

        let stream_session_id = match response {
            ProcessedMessage::ErrorResponse(msg) => return Err(XtbClientBuilderError::LoginFailed {user_id: user_id.to_string(), extra_info: format!("{:?}", msg)}),
            ProcessedMessage::Response(response) => response.stream_session_id.unwrap(),
        };

        let stream_connection = BasicXtbStreamConnection::new(stream_api_url, stream_session_id).await.map_err(|err| XtbClientBuilderError::CannotMakeStreamConnection(err))?;

        Ok(XtbClient::new(connection, stream_connection))
    }

    fn make_url(source: Option<String>) -> Result<Url, XtbClientBuilderError> {
        let source_str = source.ok_or_else(|| XtbClientBuilderError::RequiredFieldMissing("api_url".to_owned()))?;
        Url::from_str(&source_str).map_err(|err| XtbClientBuilderError::InvalidUrl(source_str, err))
    }
}


#[derive(Debug, Error)]
pub enum XtbClientBuilderError {
    #[error("Required configuration field is missing: {0}")]
    RequiredFieldMissing(String),
    #[error("Url is invalid or malformed: {0} ({1})")]
    InvalidUrl(String, url::ParseError),
    #[error("Cannot connect to server")]
    CannotMakeConnection(XtbConnectionError),
    #[error("Cannot connect to stream server")]
    CannotMakeStreamConnection(XtbStreamConnectionError),
    #[error("Login failed for user: {user_id} ({extra_info:?})")]
    LoginFailed { user_id: String, extra_info:  String },
    #[error("Something gets horribly wrong: {0}")]
    UnexpectedError(String)
}



pub struct XtbClient {
    connection: BasicXtbConnection,
    stream_connection: BasicXtbStreamConnection,
}


impl XtbClient {
    pub fn builder() -> XtbClientBuilder {
        XtbClientBuilder::default()
    }

    pub fn new(connection: BasicXtbConnection, stream_connection: BasicXtbStreamConnection) -> Self {
        Self {
            connection,
            stream_connection,
        }
    }
}
