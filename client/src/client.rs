use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;

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

use crate::{BasicXtbConnection, BasicXtbStreamConnection, ResponsePromise, XtbConnection, XtbConnectionError, XtbStreamConnection, XtbStreamConnectionError};
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


#[async_trait]
pub trait ApiClient {
    /// Returns array of all symbols available for the user.
    async fn get_all_symbols(&mut self) -> ResponsePromise;

    /// Returns calendar with market events.
    async fn get_calendar(&mut self) -> ResponsePromise;

    /// Please note that this function can be usually replaced by its streaming equivalent
    /// getCandles which is the preferred way of retrieving current candle data. Returns chart info,
    /// from start date to the current time. If the chosen period of CHART_LAST_INFO_RECORD is
    /// greater than 1 minute, the last candle returned by the API can change until the end of the
    /// period (the candle is being automatically updated every minute).
    //
    // Limitations: there are limitations in charts data availability. Detailed ranges for charts
    // data, what can be accessed with specific period, are as follows:
    //
    // * PERIOD_M1 --- <0-1) month, i.e. one-month time
    // * PERIOD_M30 --- <1-7) month, six months time
    // * PERIOD_H4 --- <7-13) month, six months time
    // * PERIOD_D1 --- 13 month, and earlier on
    //
    // Note, that specific PERIOD_ is the lowest (i.e. the most detailed) period, accessible
    // in listed range. For instance, in months range <1-7) you can access periods: PERIOD_M30,
    // PERIOD_H1, PERIOD_H4, PERIOD_D1, PERIOD_W1, PERIOD_MN1. Specific data ranges availability
    // is guaranteed, however those ranges may be wider, e.g.: PERIOD_M1 may be accessible
    // for 1.5 months back from now, where 1.0 months is guaranteed.
    //
    // Example scenario:
    //
    // * request charts of 5 minutes period, for 3 months time span, back from now;
    // * response: you are guaranteed to get 1 month of 5 minutes charts; because, 5 minutes period
    // charts are not accessible 2 months and 3 months back from now.
    async fn get_chart_last_request(&mut self) -> ResponsePromise;

    /// Returns calculation of commission and rate of exchange. The value is calculated as expected
    /// value, and therefore might not be perfectly accurate.
    async fn get_commission_def(&mut self) -> ResponsePromise;

    /// Returns information about account currency, and account leverage.
    async fn get_current_user_data(&mut self) -> ResponsePromise;

    /// Returns IBs data from the given time range.
    async fn get_ibs_history(&mut self) -> ResponsePromise;

    /// Please note that this function can be usually replaced by its streaming equivalent
    /// getBalance which is the preferred way of retrieving account indicators. Returns various
    /// account indicators.
    async fn get_margin_level(&mut self) -> ResponsePromise;

    /// Returns expected margin for given instrument and volume. The value is calculated as expected
    /// margin value, and therefore might not be perfectly accurate.
    async fn get_margin_trade(&mut self) -> ResponsePromise;

    /// Please note that this function can be usually replaced by its streaming equivalent getNews
    /// which is the preferred way of retrieving news data. Returns news from trading server which
    /// were sent within specified period of time.
    async fn get_news(&mut self) -> ResponsePromise;

    /// Calculates estimated profit for given deal data Should be used for calculator-like apps
    /// only. Profit for opened transactions should be taken from server, due to higher precision of
    /// server calculation.
    async fn get_profit_calculation(&mut self) -> ResponsePromise;

    /// Returns current time on trading server.
    async fn get_server_time(&mut self) -> ResponsePromise;

    /// Returns a list of step rules for DMAs.
    async fn get_step_rules(&mut self) -> ResponsePromise;

    /// Returns information about symbol available for the user.
    async fn get_symbol(&mut self) -> ResponsePromise;

    /// Please note that this function can be usually replaced by its streaming equivalent
    /// getTickPrices which is the preferred way of retrieving ticks data. Returns array of current
    /// quotations for given symbols, only quotations that changed from given timestamp are
    /// returned. New timestamp obtained from output will be used as an argument of the next call
    /// of this command.
    async fn get_tick_prices(&mut self) -> ResponsePromise;

    /// Returns array of trades listed in orders argument.
    async fn get_trade_records(&mut self) -> ResponsePromise;

    /// Please note that this function can be usually replaced by its streaming equivalent getTrades
    /// which is the preferred way of retrieving trades data. Returns array of user's trades.
    async fn get_trades(&mut self) -> ResponsePromise;

    /// Please note that this function can be usually replaced by its streaming equivalent getTrades
    /// which is the preferred way of retrieving trades data. Returns array of user's trades which
    /// were closed within specified period of time.
    async fn get_trades_history(&mut self) -> ResponsePromise;

    /// Returns quotes and trading times.
    async fn get_trading_hours(&mut self) -> ResponsePromise;

    /// Returns the current API version.
    async fn get_version(&mut self) -> ResponsePromise;

    /// Starts trade transaction. tradeTransaction sends main transaction information to the server.
    ///
    /// # Note
    ///
    /// How to verify that the trade request was accepted?
    /// The status field set to 'true' does not imply that the transaction was accepted. It only
    /// means, that the server acquired your request and began to process it. To analyse the status
    /// of the transaction (for example to verify if it was accepted or rejected) use the
    /// tradeTransactionStatus command with the order number, that came back with the response of
    /// the tradeTransaction command. You can find the example here:
    /// https://developers.xstore.pro/api/tutorials/opening_and_closing_trades2
    async fn trade_transaction(&mut self) -> ResponsePromise;

    /// Description: Please note that this function can be usually replaced by its streaming
    /// equivalent getTradeStatus which is the preferred way of retrieving transaction status data.
    /// Returns current transaction status. At any time of transaction processing client might check
    /// the status of transaction on server side. In order to do that client must provide unique
    /// order taken from tradeTransaction invocation.
    async fn trade_transaction_status(&mut self) -> ResponsePromise;
}


#[async_trait]
pub trait StreamApiClient {
    /// Each streaming command takes as an argument streamSessionId which is sent in response
    /// message for login command performed in main connection. streamSessionId token allows to
    /// identify user in streaming connection. In one streaming connection multiple commands with
    /// different streamSessionId can be invoked. It will cause sending streaming data for multiple
    /// login sessions in one streaming connection. streamSessionId is valid until logout command is
    /// performed on main connection or main connection is disconnected.
    async fn get_balance(&mut self);

    /// Stop receiving balance
    async fn stop_balance(&mut self);

    /// Subscribes for and unsubscribes from API chart candles. The interval of every candle
    /// is 1 minute. A new candle arrives every minute.
    async fn get_candles(&mut self);

    /// Stop receiving candles
    async fn stop_candles(&mut self);

    /// Subscribes for and unsubscribes from 'keep alive' messages. A new 'keep alive' message
    /// is sent by the API every 3 seconds.
    async fn get_keep_alive(&mut self);

    /// Stop receiving keep alives
    async fn stop_keep_alive(&mut self);

    /// Subscribes for and unsubscribes from news.
    async fn get_news(&mut self);

    /// Stop receiving news
    async fn stop_news(&mut self);

    /// Subscribes for and unsubscribes from profits.
    async fn get_profits(&mut self);

    /// Stop receiving news
    async fn stop_profits(&mut self);

    /// Establishes subscription for quotations and allows to obtain the relevant information
    /// in real-time, as soon as it is available in the system. The getTickPrices command can
    /// be invoked many times for the same symbol, but only one subscription for a given symbol
    /// will be created. Please beware that when multiple records are available, the order in which
    /// they are received is not guaranteed.
    async fn get_tick_prices(&mut self);

    /// Stop receiving prices
    async fn stop_tick_prices(&mut self);

    /// Establishes subscription for user trade status data and allows to obtain the relevant
    /// information in real-time, as soon as it is available in the system. Please beware that when
    /// multiple records are available, the order in which they are received is not guaranteed.
    async fn get_trades(&mut self);

    /// Stop receiving trades
    async fn stop_trades(&mut self);

    /// Allows to get status for sent trade requests in real-time, as soon as it is available
    /// in the system. Please beware that when multiple records are available, the order in which
    /// they are received is not guaranteed.
    async fn get_trade_status(&mut self);

    /// Stop receiving trade statues
    async fn stop_trade_status(&mut self);
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
