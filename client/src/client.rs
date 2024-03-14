use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use derive_setters::Setters;
use serde_json::{to_value, Value};
use thiserror::Error;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error};
use url::Url;

use crate::{BasicXtbConnection, BasicXtbStreamConnection, XtbConnection, XtbConnectionError, XtbStreamConnection, XtbStreamConnectionError};
use crate::message_processing::ProcessedMessage;
use crate::schema::{COMMAND_LOGIN, COMMAND_PING, LoginRequest, PingRequest, STREAM_PING, StreamPingSubscribe};

#[derive(Default, Setters)]
#[setters(into, prefix = "with_", strip_option)]
pub struct XtbClientBuilder {
    api_url: Option<String>,
    stream_api_url: Option<String>,
    app_id: Option<String>,
    app_name: Option<String>,
    ping_period: Option<u64>
}


impl XtbClientBuilder {
    pub fn new(api_url: &str, stream_api_url: &str) -> Self {
        XtbClientBuilder {
            api_url: Some(api_url.to_string()),
            stream_api_url: Some(stream_api_url.to_string()),
            app_id: None,
            app_name: None,
            ping_period: None
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
            .send_command(COMMAND_LOGIN, Some(login_request_value)).await
            .map_err(|err| XtbClientBuilderError::UnexpectedError(format!("{:?}", err)))?.await
            .map_err(|err| XtbClientBuilderError::UnexpectedError(format!("{:?}", err)))?;

        let stream_session_id = match response {
            ProcessedMessage::ErrorResponse(msg) => return Err(XtbClientBuilderError::LoginFailed {user_id: user_id.to_string(), extra_info: format!("{:?}", msg)}),
            ProcessedMessage::Response(response) => response.stream_session_id.unwrap(),
        };

        let stream_connection = BasicXtbStreamConnection::new(stream_api_url, stream_session_id).await.map_err(|err| XtbClientBuilderError::CannotMakeStreamConnection(err))?;

        Ok(XtbClient::new(connection, stream_connection, self.ping_period.unwrap_or(120)))
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
    connection: Arc<Mutex<BasicXtbConnection>>,
    stream_connection: Arc<Mutex<BasicXtbStreamConnection>>,
    ping_join_handle: JoinHandle<()>,
    stream_ping_join_handle: JoinHandle<()>,
}


impl XtbClient {
    pub fn builder() -> XtbClientBuilder {
        XtbClientBuilder::default()
    }

    pub fn new(connection: BasicXtbConnection, stream_connection: BasicXtbStreamConnection, ping_period: u64) -> Self {
        let connection = Arc::new(Mutex::new(connection));
        let stream_connection = Arc::new(Mutex::new(stream_connection));

        let ping_join_handle = spawn_ping(connection.clone(), ping_period);
        let stream_ping_join_handle = spawn_stream_ping(stream_connection.clone(), ping_period);

        let mut instance = Self {
            connection,
            stream_connection,
            ping_join_handle,
            stream_ping_join_handle,
        };

        instance
    }
}


impl Drop for XtbClient {
    fn drop(&mut self) {
        self.ping_join_handle.abort();
        self.stream_ping_join_handle.abort();
    }
}


/// Spawn tokio green thread and to send ping periodically to sync connection
///
/// # Arguments
///
/// * conn - the stream connection
/// * ping_secs - number of seconds between each ping
///
/// # Panics
///
/// The ping message cannot be serialized. The serialization is done before the green thread is run
///
/// # Returns
///
/// `JoinHandle` of the green thread
fn spawn_ping(conn: Arc<Mutex<BasicXtbConnection>>, ping_secs: u64) -> JoinHandle<()> {
    let ping_value = to_value(PingRequest::default()).expect("Cannot serialize ping message");
    spawn(async move {
        let mut idx = 1u64;
        loop {
            let response_promise = {
                let mut conn = conn.lock().await;
                debug!("Sending ping #{} to connection", idx);
                match conn.send_command(COMMAND_PING, Some(ping_value.clone())).await {
                    Ok(resp) => Some(resp),
                    Err(err) => {
                        error!("Cannot send ping #{}: {:?}", idx, err);
                        None
                    }
                }
            };
            if let Some(response_promise) = response_promise {
                match response_promise.await {
                    Ok(_) => (),
                    Err(err) => error!("Cannot await the ping response #{}", idx)
                }
            }
            idx += 1;
            sleep(Duration::from_secs(ping_secs)).await;
        }
    })
}


/// Spawn tokio green thread and to send ping periodically to stream connection
///
/// # Arguments
///
/// * conn - the stream connection
/// * ping_secs - number of seconds between each ping
///
/// # Panics
///
/// The ping message cannot be serialized. The serialization is done before the green thread is run
///
/// # Returns
///
/// `JoinHandle` of the green thread
fn spawn_stream_ping(conn: Arc<Mutex<BasicXtbStreamConnection>>, ping_secs: u64) -> JoinHandle<()> {
    let ping_value = to_value(StreamPingSubscribe::default()).expect("Cannot serialize the stream ping message");
    spawn(async move {
        loop {
            let mut idx = 1u64;
            {
                debug!("Sending ping #{} to stream connection", idx);
                let mut conn = conn.lock().await;
                match conn.subscribe(STREAM_PING, Some(ping_value.clone())).await {
                    Ok(_) => (),
                    Err(err) => error!("Cannot send ping #{}: {:?}", idx, err)
                }
            }
            idx += 1;
            sleep(Duration::from_secs(ping_secs)).await;
        }
    })
}
