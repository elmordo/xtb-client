use std::collections::HashMap;
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};
use thiserror::Error;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{debug, error};
use url::Url;

use crate::{BasicMessageStream, BasicXtbConnection, BasicXtbStreamConnection, DataMessageFilter, MessageStream, ResponsePromise, XtbConnection, BasicXtbConnectionError, XtbStreamConnection, BasicXtbStreamConnectionError};
use crate::message_processing::ProcessedMessage;
use crate::schema::{COMMAND_GET_ALL_SYMBOLS, COMMAND_GET_CALENDAR, COMMAND_GET_CHART_LAST_REQUEST, COMMAND_GET_CHART_RANGE_REQUEST, COMMAND_GET_COMMISSION_DEF, COMMAND_GET_CURRENT_USER_DATA, COMMAND_GET_IBS_HISTORY, COMMAND_GET_MARGIN_LEVEL, COMMAND_GET_MARGIN_TRADE, COMMAND_GET_NEWS, COMMAND_GET_PROFIT_CALCULATION, COMMAND_GET_SERVER_TIME, COMMAND_GET_STEP_RULES, COMMAND_GET_SYMBOL, COMMAND_GET_TICK_PRICES, COMMAND_GET_TRADE_RECORDS, COMMAND_GET_TRADES, COMMAND_GET_TRADES_HISTORY, COMMAND_GET_TRADING_HOURS, COMMAND_GET_VERSION, COMMAND_LOGIN, COMMAND_PING, COMMAND_TRADE_TRANSACTION, COMMAND_TRADE_TRANSACTION_STATUS, ErrorResponse, GetAllSymbolsRequest, GetAllSymbolsResponse, GetCalendarRequest, GetCalendarResponse, GetChartLastRequestRequest, GetChartLastRequestResponse, GetChartRangeRequestRequest, GetChartRangeRequestResponse, GetCommissionDefRequest, GetCommissionDefResponse, GetCurrentUserDataRequest, GetCurrentUserDataResponse, GetIbsHistoryRequest, GetIbsHistoryResponse, GetMarginLevelRequest, GetMarginLevelResponse, GetMarginTradeRequest, GetMarginTradeResponse, GetNewsRequest, GetNewsResponse, GetProfitCalculationRequest, GetProfitCalculationResponse, GetServerTimeRequest, GetServerTimeResponse, GetStepRulesRequest, GetStepRulesResponse, GetSymbolRequest, GetSymbolResponse, GetTickPricesRequest, GetTickPricesResponse, GetTradeRecordsRequest, GetTradeRecordsResponse, GetTradesHistoryRequest, GetTradesHistoryResponse, GetTradesRequest, GetTradesResponse, GetTradingHoursRequest, GetTradingHoursResponse, GetVersionRequest, GetVersionResponse, LoginRequest, PingRequest, STREAM_BALANCE, STREAM_CANDLES, STREAM_BALANCE_SUBSCRIBE, STREAM_CANDLES_SUBSCRIBE, STREAM_KEEP_ALIVE_SUBSCRIBE, STREAM_NEWS_SUBSCRIBE, STREAM_PROFITS_SUBSCRIBE, STREAM_TICK_PRICES_SUBSCRIBE, STREAM_TRADE_STATUS_SUBSCRIBE, STREAM_TRADES_SUBSCRIBE, STREAM_KEEP_ALIVE, STREAM_NEWS, STREAM_PING, STREAM_PROFITS, STREAM_BALANCE_UNSUBSCRIBE, STREAM_CANDLES_UNSUBSCRIBE, STREAM_KEEP_ALIVE_UNSUBSCRIBE, STREAM_NEWS_UNSUBSCRIBE, STREAM_PROFITS_UNSUBSCRIBE, STREAM_TICK_PRICES_UNSUBSCRIBE, STREAM_TRADE_STATUS_UNSUBSCRIBE, STREAM_TRADES_UNSUBSCRIBE, STREAM_TICK_PRICES, STREAM_TRADE_STATUS, STREAM_TRADES, StreamDataMessage, StreamGetBalanceData, StreamGetBalanceSubscribe, StreamGetBalanceUnsubscribe, StreamGetCandlesData, StreamGetCandlesSubscribe, StreamGetCandlesUnsubscribe, StreamGetKeepAliveData, StreamGetKeepAliveSubscribe, StreamGetKeepAliveUnsubscribe, StreamGetNewsData, StreamGetNewsSubscribe, StreamGetNewsUnsubscribe, StreamGetProfitData, StreamGetProfitSubscribe, StreamGetProfitUnsubscribe, StreamGetTickPricesData, StreamGetTickPricesSubscribe, StreamGetTickPricesUnsubscribe, StreamGetTradesData, StreamGetTradesSubscribe, StreamGetTradeStatusData, StreamGetTradeStatusSubscribe, StreamGetTradeStatusUnsubscribe, StreamGetTradesUnsubscribe, StreamPingSubscribe, TradeTransactionRequest, TradeTransactionResponse, TradeTransactionStatusRequest, TradeTransactionStatusResponse};


/// Builder for `XtbClient`.
///
/// Configuration can be set with this class and when configuration step is finished, the `XtbClient`
/// can be created by the `build()` method.
///
/// Configurable fields are:
///
/// * `api_url` - url of the request/response API server.
/// * `stream_api_url` - url of the stream API server.
/// * `app_id` - application identifier (deprecated by the official API documentation)
/// * `app_name` - application name (deprecated by the official API documentation)
/// * `ping_period` - interval between ping commands. Default interval is 30s.
///
/// The required configuration values are `api_url` and `stream_api_url`. Other values are optional.
///
/// Official documentation says, the `ping_interval` should be less than 10 minutes. But real world
/// observation shows that the maximal interval must be less than 1 minute.
#[derive(Default, Setters)]
#[setters(into, prefix = "with_", strip_option)]
pub struct XtbClientBuilder {
    /// Url of the request/response API server
    api_url: Option<String>,
    /// Url of the stream API server
    stream_api_url: Option<String>,
    /// Identifier of the application. (Deprecated by the official api)
    app_id: Option<String>,
    /// Name of the application (deprecated by the official api)
    app_name: Option<String>,
    /// Interval between pings. Shouldn't be greater than 1 minute.
    ping_period: Option<u64>,
}


const DEFAULT_PING_INTERVAL_S: u64 = 30;

const DEFAULT_XTB_REAL: &'static str = "wss://ws.xtb.com/real";
const DEFAULT_XTB_REAL_STREAM: &'static str = "wss://ws.xtb.com/realStream";
const DEFAULT_XTB_DEMO: &'static str = "wss://ws.xtb.com/demo";
const DEFAULT_XTB_DEMO_STREAM: &'static str = "wss://ws.xtb.com/demoStream";


impl XtbClientBuilder {
    /// Create new builder using custom server urls.
    ///
    /// The resulting builder instance can be instantly built into the `XtbClient`
    pub fn new(api_url: &str, stream_api_url: &str) -> Self {
        Self {
            api_url: Some(api_url.to_string()),
            stream_api_url: Some(stream_api_url.to_string()),
            app_id: None,
            app_name: None,
            ping_period: None,
        }
    }

    /// Create new builder without any configuration.
    ///
    /// The `api_url` and the `stream_api_url` must be set at least.
    pub fn new_bare() -> Self {
        return Self {
            api_url: None,
            stream_api_url: None,
            app_id: None,
            app_name: None,
            ping_period: None,
        }
    }

    /// Shorthand for `XtbClientBuilder::new("wss://ws.xtb.com/real", "wss://ws.xtb.com/realStream")`
    pub fn new_real() -> Self {
        Self::new(DEFAULT_XTB_REAL, DEFAULT_XTB_REAL_STREAM)
    }


    /// Shorthand for `XtbClientBuilder::new("wss://ws.xtb.com/demo", "wss://ws.xtb.com/demoStream")`
    pub fn new_demo() -> Self {
        Self::new(DEFAULT_XTB_DEMO, DEFAULT_XTB_DEMO_STREAM)
    }

    /// Consume the builder instance and create instance of the `XtbClient`.
    ///
    /// The login is performed in this step.
    ///
    /// # Returns
    ///
    /// * `Ok(XtbClient)` - connected to the servers. The credentials given as arguments are
    /// used to log in an user.
    /// * `Err(XtbClintBuilderError)` - cannot connect to the servers.
    ///
    /// Common reasons of an errors are:
    ///
    /// * Incorrect credentials.
    /// * Malformed API servers urls.
    /// * Missing required configuration value.
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
            ProcessedMessage::ErrorResponse(msg) => return Err(XtbClientBuilderError::LoginFailed { user_id: user_id.to_string(), extra_info: format!("{:?}", msg) }),
            ProcessedMessage::Response(response) => response.stream_session_id.unwrap(),
        };

        let stream_connection = BasicXtbStreamConnection::new(stream_api_url, stream_session_id).await.map_err(|err| XtbClientBuilderError::CannotMakeStreamConnection(err))?;

        Ok(XtbClient::new(connection, stream_connection, self.ping_period.unwrap_or(DEFAULT_PING_INTERVAL_S)))
    }

    /// Convert string into an `Url` instance. This method is also used for validation of url presence.
    ///
    /// # Returns
    ///
    /// * `Ok(Url)` with correctly parsed url.
    /// * `Err(XtbClientBuilderError::RequiredFieldMissing)` if argument is `None`
    /// * `Err(XtbClientBuilderError::InvalidUrl)` if the url is malformed (cannot be parsed).
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
    CannotMakeConnection(BasicXtbConnectionError),
    #[error("Cannot connect to stream server")]
    CannotMakeStreamConnection(BasicXtbStreamConnectionError),
    #[error("Login failed for user: {user_id} ({extra_info:?})")]
    LoginFailed { user_id: String, extra_info: String },
    #[error("Something gets horribly wrong: {0}")]
    UnexpectedError(String),
}


/// Declaration of the Request/response API interface.
#[async_trait]
pub trait CommandApi {
    /// Error returned from methods when command failed
    type Error;

    /// Returns array of all symbols available for the user.
    async fn get_all_symbols(&mut self, request: GetAllSymbolsRequest) -> Result<GetAllSymbolsResponse, Self::Error>;

    /// Returns calendar with market events.
    async fn get_calendar(&mut self, request: GetCalendarRequest) -> Result<GetCalendarResponse, Self::Error>;

    /// Please note that this function can be usually replaced by its streaming equivalent
    /// getCandles which is the preferred way of retrieving current candle data. Returns chart info,
    /// from start date to the current time. If the chosen period of CHART_LAST_INFO_RECORD is
    /// greater than 1 minute, the last candle returned by the API can change until the end of the
    /// period (the candle is being automatically updated every minute).
    ///
    /// Limitations: there are limitations in charts data availability. Detailed ranges for charts
    /// data, what can be accessed with specific period, are as follows:
    ///
    /// * PERIOD_M1 --- <0-1) month, i.e. one-month time
    /// * PERIOD_M30 --- <1-7) month, six months time
    /// * PERIOD_H4 --- <7-13) month, six months time
    /// * PERIOD_D1 --- 13 month, and earlier on
    ///
    /// Note, that specific PERIOD_ is the lowest (i.e. the most detailed) period, accessible
    /// in listed range. For instance, in months range <1-7) you can access periods: PERIOD_M30,
    /// PERIOD_H1, PERIOD_H4, PERIOD_D1, PERIOD_W1, PERIOD_MN1. Specific data ranges availability
    /// is guaranteed, however those ranges may be wider, e.g.: PERIOD_M1 may be accessible
    /// for 1.5 months back from now, where 1.0 months is guaranteed.
    ///
    /// Example scenario:
    ///
    /// * request charts of 5 minutes period, for 3 months time span, back from now;
    /// * response: you are guaranteed to get 1 month of 5 minutes charts; because, 5 minutes period
    /// charts are not accessible 2 months and 3 months back from now.
    async fn get_chart_last_request(&mut self, request: GetChartLastRequestRequest) -> Result<GetChartLastRequestResponse, Self::Error>;

    /// Please note that this function can be usually replaced by its streaming equivalent
    /// getCandles which is the preferred way of retrieving current candle data. Returns chart info
    /// with data between given start and end dates.
    ///
    /// Limitations: there are limitations in charts data availability. Detailed ranges for charts
    /// data, what can be accessed with specific period, are as follows:
    ///
    /// * PERIOD_M1 --- <0-1) month, i.e. one month time
    /// * PERIOD_M30 --- <1-7) month, six months time
    /// * PERIOD_H4 --- <7-13) month, six months time
    /// * PERIOD_D1 --- 13 month, and earlier on
    ///
    /// Note, that specific PERIOD_ is the lowest (i.e. the most detailed) period, accessible
    /// in listed range. For instance, in months range <1-7) you can access periods: PERIOD_M30,
    /// PERIOD_H1, PERIOD_H4, PERIOD_D1, PERIOD_W1, PERIOD_MN1. Specific data ranges availability
    /// is guaranteed, however those ranges may be wider, e.g.: PERIOD_M1 may be accessible
    /// for 1.5 months back from now, where 1.0 months is guaranteed.
    async fn get_chart_range_request(&mut self, request: GetChartRangeRequestRequest) -> Result<GetChartRangeRequestResponse, Self::Error>;

    /// Returns calculation of commission and rate of exchange. The value is calculated as expected
    /// value, and therefore might not be perfectly accurate.
    async fn get_commission_def(&mut self, request: GetCommissionDefRequest) -> Result<GetCommissionDefResponse, Self::Error>;

    /// Returns information about account currency, and account leverage.
    async fn get_current_user_data(&mut self, request: GetCurrentUserDataRequest) -> Result<GetCurrentUserDataResponse, Self::Error>;

    /// Returns IBs data from the given time range.
    async fn get_ibs_history(&mut self, request: GetIbsHistoryRequest) -> Result<GetIbsHistoryResponse, Self::Error>;

    /// Please note that this function can be usually replaced by its streaming equivalent
    /// getBalance which is the preferred way of retrieving account indicators. Returns various
    /// account indicators.
    async fn get_margin_level(&mut self, request: GetMarginLevelRequest) -> Result<GetMarginLevelResponse, Self::Error>;

    /// Returns expected margin for given instrument and volume. The value is calculated as expected
    /// margin value, and therefore might not be perfectly accurate.
    async fn get_margin_trade(&mut self, request: GetMarginTradeRequest) -> Result<GetMarginTradeResponse, Self::Error>;

    /// Please note that this function can be usually replaced by its streaming equivalent getNews
    /// which is the preferred way of retrieving news data. Returns news from trading server which
    /// were sent within specified period of time.
    async fn get_news(&mut self, request: GetNewsRequest) -> Result<GetNewsResponse, Self::Error>;

    /// Calculates estimated profit for given deal data Should be used for calculator-like apps
    /// only. Profit for opened transactions should be taken from server, due to higher precision of
    /// server calculation.
    async fn get_profit_calculation(&mut self, request: GetProfitCalculationRequest) -> Result<GetProfitCalculationResponse, Self::Error>;

    /// Returns current time on trading server.
    async fn get_server_time(&mut self, request: GetServerTimeRequest) -> Result<GetServerTimeResponse, Self::Error>;

    /// Returns a list of step rules for DMAs.
    async fn get_step_rules(&mut self, request: GetStepRulesRequest) -> Result<GetStepRulesResponse, Self::Error>;

    /// Returns information about symbol available for the user.
    async fn get_symbol(&mut self, request: GetSymbolRequest) -> Result<GetSymbolResponse, Self::Error>;

    /// Please note that this function can be usually replaced by its streaming equivalent
    /// getTickPrices which is the preferred way of retrieving ticks data. Returns array of current
    /// quotations for given symbols, only quotations that changed from given timestamp are
    /// returned. New timestamp obtained from output will be used as an argument of the next call
    /// of this command.
    async fn get_tick_prices(&mut self, request: GetTickPricesRequest) -> Result<GetTickPricesResponse, Self::Error>;

    /// Returns array of trades listed in orders argument.
    async fn get_trade_records(&mut self, request: GetTradeRecordsRequest) -> Result<GetTradeRecordsResponse, Self::Error>;

    /// Please note that this function can be usually replaced by its streaming equivalent getTrades
    /// which is the preferred way of retrieving trades data. Returns array of user's trades.
    async fn get_trades(&mut self, request: GetTradesRequest) -> Result<GetTradesResponse, Self::Error>;

    /// Please note that this function can be usually replaced by its streaming equivalent getTrades
    /// which is the preferred way of retrieving trades data. Returns array of user's trades which
    /// were closed within specified period of time.
    async fn get_trades_history(&mut self, request: GetTradesHistoryRequest) -> Result<GetTradesHistoryResponse, Self::Error>;

    /// Returns quotes and trading times.
    async fn get_trading_hours(&mut self, request: GetTradingHoursRequest) -> Result<GetTradingHoursResponse, Self::Error>;

    /// Returns the current API version.
    async fn get_version(&mut self, request: GetVersionRequest) -> Result<GetVersionResponse, Self::Error>;

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
    async fn trade_transaction(&mut self, request: TradeTransactionRequest) -> Result<TradeTransactionResponse, Self::Error>;

    /// Description: Please note that this function can be usually replaced by its streaming
    /// equivalent getTradeStatus which is the preferred way of retrieving transaction status data.
    /// Returns current transaction status. At any time of transaction processing client might check
    /// the status of transaction on server side. In order to do that client must provide unique
    /// order taken from tradeTransaction invocation.
    async fn trade_transaction_status(&mut self, request: TradeTransactionStatusRequest) -> Result<TradeTransactionStatusResponse, Self::Error>;
}


/// Declaration of the stream API interface
#[async_trait]
pub trait StreamApi {
    /// Error returned from the client when something went wrong
    type Error;

    type Stream<T: Send + Sync + for<'de> Deserialize<'de>>;

    /// Each streaming command takes as an argument streamSessionId which is sent in response
    /// message for login command performed in main connection. streamSessionId token allows to
    /// identify user in streaming connection. In one streaming connection multiple commands with
    /// different streamSessionId can be invoked. It will cause sending streaming data for multiple
    /// login sessions in one streaming connection. streamSessionId is valid until logout command is
    /// performed on main connection or main connection is disconnected.
    async fn subscribe_balance(&mut self, arguments: StreamGetBalanceSubscribe) -> Result<Self::Stream<StreamGetBalanceData>, Self::Error>;

    /// Subscribes for and unsubscribes from API chart candles. The interval of every candle
    /// is 1 minute. A new candle arrives every minute.
    async fn subscribe_candles(&mut self, arguments: StreamGetCandlesSubscribe) -> Result<Self::Stream<StreamGetCandlesData>, Self::Error>;

    /// Subscribes for and unsubscribes from 'keep alive' messages. A new 'keep alive' message
    /// is sent by the API every 3 seconds.
    async fn subscribe_keep_alive(&mut self, arguments: StreamGetKeepAliveSubscribe) -> Result<Self::Stream<StreamGetKeepAliveData>, Self::Error>;

    /// Subscribes for and unsubscribes from news.
    async fn subscribe_news(&mut self, arguments: StreamGetNewsSubscribe) -> Result<Self::Stream<StreamGetNewsData>, Self::Error>;

    /// Subscribes for and unsubscribes from profits.
    async fn subscribe_profits(&mut self, arguments: StreamGetProfitSubscribe) -> Result<Self::Stream<StreamGetProfitData>, Self::Error>;

    /// Establishes subscription for quotations and allows to obtain the relevant information
    /// in real-time, as soon as it is available in the system. The getTickPrices command can
    /// be invoked many times for the same symbol, but only one subscription for a given symbol
    /// will be created. Please beware that when multiple records are available, the order in which
    /// they are received is not guaranteed.
    async fn subscribe_tick_prices(&mut self, arguments: StreamGetTickPricesSubscribe) -> Result<Self::Stream<StreamGetTickPricesData>, Self::Error>;

    /// Establishes subscription for user trade status data and allows to obtain the relevant
    /// information in real-time, as soon as it is available in the system. Please beware that when
    /// multiple records are available, the order in which they are received is not guaranteed.
    async fn subscribe_trades(&mut self, arguments: StreamGetTradesSubscribe) -> Result<Self::Stream<StreamGetTradesData>, Self::Error>;

    /// Allows to get status for sent trade requests in real-time, as soon as it is available
    /// in the system. Please beware that when multiple records are available, the order in which
    /// they are received is not guaranteed.
    async fn subscribe_trade_status(&mut self, arguments: StreamGetTradeStatusSubscribe) -> Result<Self::Stream<StreamGetTradeStatusData>, Self::Error>;
}


/// Implementor of the API traits.
///
/// This struct is designed to be an interface between user (application) and XTB API servers.
///
/// The `XtbClient` is responsible for sending and receiving pings and logout when instance is dropped.
pub struct XtbClient {
    /// Connection to the request/response server
    connection: Arc<Mutex<BasicXtbConnection>>,
    /// Connection to the stream server
    stream_manager: StreamManager,
    /// handle of the request/response server ping worker
    ping_join_handle: JoinHandle<()>,
    /// handle of the stream server ping worker
    stream_ping_join_handle: JoinHandle<()>,
}


impl XtbClient {
    /// Create builder for the `XtbClient`.
    ///
    /// When builder is configured, the `build()` method can be called with credentials passed to the
    /// method arguments. This call create new `XtbClient` instance.
    pub fn builder() -> XtbClientBuilder {
        XtbClientBuilder::default()
    }

    /// Create new instance of the `XtbClient`. it expects connected and logged connections to the
    /// request/response and the stream server.
    ///
    /// # Note
    ///
    /// The login is performed by the builder because the stream server implementation needs to know
    /// a stream session id which is provided by the `login` command.
    pub fn new(connection: BasicXtbConnection, stream_connection: BasicXtbStreamConnection, ping_period: u64) -> Self {
        let connection = Arc::new(Mutex::new(connection));

        let ping_join_handle = spawn_ping(connection.clone(), ping_period);

        let stream_manager = StreamManager::new(stream_connection);
        let stream_ping_join_handle = spawn_stream_ping(stream_manager.clone(), ping_period);

        Self {
            connection,
            stream_manager,
            ping_join_handle,
            stream_ping_join_handle,
        }
    }

    /// Send command to the server and wait for response.
    ///
    /// If command does not return any response, create default one with type of `RESP`.
    async fn send_and_wait_or_default<REQ, RESP>(&mut self, command: &str, request: REQ) -> Result<RESP, XtbClientError>
        where
            REQ: Serialize,
            RESP: for<'de> Deserialize<'de> + Default {
        self.send_and_wait(command, request).await.map(|val| val.unwrap_or_default())
    }

    /// Send the command and wait for a response.
    async fn send_and_wait<REQ, RESP>(&mut self, command: &str, request: REQ) -> Result<Option<RESP>, XtbClientError>
        where
            REQ: Serialize,
            RESP: for<'de> Deserialize<'de>
    {
        let promise = self.send(command, request).await?;
        let response = promise.await.map_err(|err| {
            error!("Unexpected error: {:?}", err);
            XtbClientError::UnexpectedError
        })?;
        match response {
            ProcessedMessage::Response(response) => {
                match response.return_data {
                    Some(data) => from_value(data).map_err(|err| XtbClientError::DeserializationFailed(err)).map(|v| Some(v)),
                    None => Ok(None)
                }
            }
            ProcessedMessage::ErrorResponse(err) => Err(XtbClientError::CommandFailed(err)),
        }
    }

    /// Send a command request to the server and return `Ok(ResponsePromise)` o
    async fn send<A>(&mut self, command: &str, request: A) -> Result<ResponsePromise, XtbClientError>
        where
            A: Serialize
    {
        let mut conn = self.connection.lock().await;
        let payload = Self::convert_data_to_value(request)?;
        conn.send_command(command, Some(payload)).await.map_err(|err| {
            match err {
                BasicXtbConnectionError::SerializationError(err) => XtbClientError::SerializationFailed(err),
                BasicXtbConnectionError::CannotSendRequest(err) => XtbClientError::CannotSendCommand(err),
                _ => XtbClientError::UnexpectedError,
            }
        })
    }

    /// Serialize payload data into value.
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` when data was serialized successfully.
    /// * `Err(XtbClientError::SerializationFailed)` - data cannot be serialized.
    fn convert_data_to_value<T: Serialize>(data: T) -> Result<Value, XtbClientError> {
        to_value(data).map_err(|err| XtbClientError::SerializationFailed(err))
    }

    /// Send stream command to the stream API server.
    ///
    /// # Parameters
    ///
    /// * `subscribe_command` - command name of the subscribe command (e.g. `getCandles`)
    /// * `subscribe_arguments` - arguments for the subscribe command
    /// * `unsubscribe_command` - command name of the unsubscribe command (e.g. `stopCandles`)
    /// * `unsubscribe_arguments` - arguments for the unsubscribe command
    /// * `data_command` - command name in data messages (e.g. `candle`)
    ///
    /// # Returns
    ///
    /// * `Ok<DataStream<T>>` - data stream with filter set to messages related to sent command
    /// * `Err<XtbClientError>` - unable to send command
    async fn send_simple_stream_command<T, SA, UA>(
        &mut self,
        subscribe_command: &str,
        subscribe_arguments: SA,
        unsubscribe_command: &str,
        unsubscribe_arguments: UA,
        data_command: &str,
    ) -> Result<DataStream<T>, XtbClientError>
        where
            T: for<'de> Deserialize<'de> + Send + Sync,
            SA: Serialize,
            UA: Serialize,
    {
        let unsubscribe_arguments = Self::convert_data_to_value(unsubscribe_arguments)?;
        let filter = DataMessageFilter::Command(data_command.to_owned());
        let subscribe_arguments = Self::convert_data_to_value(subscribe_arguments)?;
        self.stream_manager.subscribe(subscribe_command, Some(subscribe_arguments), unsubscribe_command, Some(unsubscribe_arguments), data_command, filter).await
    }

    /// Send stream command to the stream API server and add filter by the `symbol` field to the
    /// data stream.
    ///
    /// # Parameters
    ///
    /// * `subscribe_command` - command name of the subscribe command (e.g. `getCandles`)
    /// * `subscribe_arguments` - arguments for the subscribe command
    /// * `unsubscribe_command` - command name of the unsubscribe command (e.g. `stopCandles`)
    /// * `unsubscribe_arguments` - arguments for the unsubscribe command
    /// * `data_command` - command name in data messages (e.g. `candle`)
    ///
    /// # Returns
    ///
    /// * `Ok<DataStream<T>>` - data stream with filter set to messages related to sent command
    /// * `Err<XtbClientError>` - unable to send command
    async fn send_symbol_scoped_stream_command<T, SA, UA>(
        &mut self,
        subscribe_command: &str,
        subscribe_arguments: SA,
        unsubscribe_command: &str,
        unsubscribe_arguments: UA,
        data_command: &str,
        symbol: &str,
    ) -> Result<DataStream<T>, XtbClientError>
        where
            T: for<'de> Deserialize<'de> + Send + Sync,
            SA: Serialize,
            UA: Serialize,
    {
        let unsubscribe_arguments = Self::convert_data_to_value(unsubscribe_arguments)?;
        let subscribe_arguments = Self::convert_data_to_value(subscribe_arguments)?;
        let subscription_key = format!("{}.{}", data_command, symbol);

        let filter = DataMessageFilter::All(vec![
            DataMessageFilter::Command(data_command.to_owned()),
            DataMessageFilter::FieldValue { name: "symbol".to_owned(), value: Value::String(symbol.to_owned()) },
        ]);
        self.stream_manager.subscribe(subscribe_command, Some(subscribe_arguments), unsubscribe_command, Some(unsubscribe_arguments), &subscription_key, filter).await
    }
}


impl Drop for XtbClient {
    fn drop(&mut self) {
        self.ping_join_handle.abort();
        self.stream_ping_join_handle.abort();
    }
}


#[async_trait]
impl CommandApi for XtbClient {
    type Error = XtbClientError;

    async fn get_all_symbols(&mut self, request: GetAllSymbolsRequest) -> Result<GetAllSymbolsResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_ALL_SYMBOLS, request).await
    }

    async fn get_calendar(&mut self, request: GetCalendarRequest) -> Result<GetCalendarResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_CALENDAR, request).await
    }

    async fn get_chart_last_request(&mut self, request: GetChartLastRequestRequest) -> Result<GetChartLastRequestResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_CHART_LAST_REQUEST, request).await
    }

    async fn get_chart_range_request(&mut self, request: GetChartRangeRequestRequest) -> Result<GetChartRangeRequestResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_CHART_RANGE_REQUEST, request).await
    }

    async fn get_commission_def(&mut self, request: GetCommissionDefRequest) -> Result<GetCommissionDefResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_COMMISSION_DEF, request).await
    }

    async fn get_current_user_data(&mut self, request: GetCurrentUserDataRequest) -> Result<GetCurrentUserDataResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_CURRENT_USER_DATA, request).await
    }

    async fn get_ibs_history(&mut self, request: GetIbsHistoryRequest) -> Result<GetIbsHistoryResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_IBS_HISTORY, request).await
    }

    async fn get_margin_level(&mut self, request: GetMarginLevelRequest) -> Result<GetMarginLevelResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_MARGIN_LEVEL, request).await
    }

    async fn get_margin_trade(&mut self, request: GetMarginTradeRequest) -> Result<GetMarginTradeResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_MARGIN_TRADE, request).await
    }

    async fn get_news(&mut self, request: GetNewsRequest) -> Result<GetNewsResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_NEWS, request).await
    }

    async fn get_profit_calculation(&mut self, request: GetProfitCalculationRequest) -> Result<GetProfitCalculationResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_PROFIT_CALCULATION, request).await
    }

    async fn get_server_time(&mut self, request: GetServerTimeRequest) -> Result<GetServerTimeResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_SERVER_TIME, request).await
    }

    async fn get_step_rules(&mut self, request: GetStepRulesRequest) -> Result<GetStepRulesResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_STEP_RULES, request).await
    }

    async fn get_symbol(&mut self, request: GetSymbolRequest) -> Result<GetSymbolResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_SYMBOL, request).await
    }

    async fn get_tick_prices(&mut self, request: GetTickPricesRequest) -> Result<GetTickPricesResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_TICK_PRICES, request).await
    }

    async fn get_trade_records(&mut self, request: GetTradeRecordsRequest) -> Result<GetTradeRecordsResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_TRADE_RECORDS, request).await
    }

    async fn get_trades(&mut self, request: GetTradesRequest) -> Result<GetTradesResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_TRADES, request).await
    }

    async fn get_trades_history(&mut self, request: GetTradesHistoryRequest) -> Result<GetTradesHistoryResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_TRADES_HISTORY, request).await
    }

    async fn get_trading_hours(&mut self, request: GetTradingHoursRequest) -> Result<GetTradingHoursResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_TRADING_HOURS, request).await
    }

    async fn get_version(&mut self, request: GetVersionRequest) -> Result<GetVersionResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_GET_VERSION, request).await
    }

    async fn trade_transaction(&mut self, request: TradeTransactionRequest) -> Result<TradeTransactionResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_TRADE_TRANSACTION, request).await
    }

    async fn trade_transaction_status(&mut self, request: TradeTransactionStatusRequest) -> Result<TradeTransactionStatusResponse, Self::Error> {
        self.send_and_wait_or_default(COMMAND_TRADE_TRANSACTION_STATUS, request).await
    }
}


#[async_trait]
impl StreamApi for XtbClient {
    type Error = XtbClientError;

    type Stream<T: Send + Sync + for<'de> Deserialize<'de>> = DataStream<T>;

    async fn subscribe_balance(&mut self, arguments: StreamGetBalanceSubscribe) -> Result<Self::Stream<StreamGetBalanceData>, Self::Error> {
        let stop_arguments = Self::convert_data_to_value(StreamGetBalanceUnsubscribe::default())?;
        self.send_simple_stream_command(STREAM_BALANCE_SUBSCRIBE, arguments, STREAM_BALANCE_UNSUBSCRIBE, stop_arguments, STREAM_BALANCE).await
    }

    async fn subscribe_candles(&mut self, arguments: StreamGetCandlesSubscribe) -> Result<Self::Stream<StreamGetCandlesData>, Self::Error> {
        let stop_arguments = Self::convert_data_to_value(StreamGetCandlesUnsubscribe::default().with_symbol(&arguments.symbol))?;
        let symbol = arguments.symbol.clone();
        self.send_symbol_scoped_stream_command(STREAM_CANDLES_SUBSCRIBE, arguments, STREAM_CANDLES_UNSUBSCRIBE, stop_arguments, STREAM_CANDLES, &symbol).await
    }

    async fn subscribe_keep_alive(&mut self, arguments: StreamGetKeepAliveSubscribe) -> Result<Self::Stream<StreamGetKeepAliveData>, Self::Error> {
        let stop_arguments = Self::convert_data_to_value(StreamGetKeepAliveUnsubscribe::default())?;
        self.send_simple_stream_command(STREAM_KEEP_ALIVE_SUBSCRIBE, arguments, STREAM_KEEP_ALIVE_UNSUBSCRIBE, stop_arguments, STREAM_KEEP_ALIVE).await
    }

    async fn subscribe_news(&mut self, arguments: StreamGetNewsSubscribe) -> Result<Self::Stream<StreamGetNewsData>, Self::Error> {
        let stop_arguments = Self::convert_data_to_value(StreamGetNewsUnsubscribe::default())?;
        self.send_simple_stream_command(STREAM_NEWS_SUBSCRIBE, arguments, STREAM_NEWS_UNSUBSCRIBE, stop_arguments, STREAM_NEWS).await
    }

    async fn subscribe_profits(&mut self, arguments: StreamGetProfitSubscribe) -> Result<Self::Stream<StreamGetProfitData>, Self::Error> {
        let stop_arguments = Self::convert_data_to_value(StreamGetProfitUnsubscribe::default())?;
        self.send_simple_stream_command(STREAM_PROFITS_SUBSCRIBE, arguments, STREAM_PROFITS_UNSUBSCRIBE, stop_arguments, STREAM_PROFITS).await
    }

    async fn subscribe_tick_prices(&mut self, arguments: StreamGetTickPricesSubscribe) -> Result<Self::Stream<StreamGetTickPricesData>, Self::Error> {
        let stop_arguments = Self::convert_data_to_value(StreamGetTickPricesUnsubscribe::default().with_symbol(&arguments.symbol))?;
        let symbol = arguments.symbol.clone();
        self.send_symbol_scoped_stream_command(STREAM_TICK_PRICES_SUBSCRIBE, arguments, STREAM_TICK_PRICES_UNSUBSCRIBE, stop_arguments, STREAM_TICK_PRICES, &symbol).await
    }

    async fn subscribe_trades(&mut self, arguments: StreamGetTradesSubscribe) -> Result<Self::Stream<StreamGetTradesData>, Self::Error> {
        let stop_arguments = Self::convert_data_to_value(StreamGetTradesUnsubscribe::default())?;
        self.send_simple_stream_command(STREAM_TRADES_SUBSCRIBE, arguments, STREAM_TRADES_UNSUBSCRIBE, stop_arguments, STREAM_TRADES).await
    }

    async fn subscribe_trade_status(&mut self, arguments: StreamGetTradeStatusSubscribe) -> Result<Self::Stream<StreamGetTradeStatusData>, Self::Error> {
        let stop_arguments = Self::convert_data_to_value(StreamGetTradeStatusUnsubscribe::default())?;
        self.send_simple_stream_command(STREAM_TRADE_STATUS_SUBSCRIBE, arguments, STREAM_TRADE_STATUS_UNSUBSCRIBE, stop_arguments, STREAM_TRADE_STATUS).await
    }
}


#[derive(Debug, Error)]
pub enum XtbClientError {
    #[error("Cannot serialize arguments")]
    SerializationFailed(serde_json::Error),
    #[error("Cannot send command to server")]
    CannotSendCommand(tokio_tungstenite::tungstenite::Error),
    #[error("Cannot send stream command")]
    CannotSendStreamCommand(BasicXtbStreamConnectionError),
    #[error("Unexpected error.")]
    UnexpectedError,
    #[error("Cannot deserialize data")]
    DeserializationFailed(serde_json::Error),
    #[error("Command failed and an error response was returned")]
    CommandFailed(ErrorResponse),
}


/// Shared inner state of the `StreamManager`
#[derive(Debug)]
struct StreamManagerState {
    /// The stream connection
    connection: BasicXtbStreamConnection,
    /// subscription counter
    subscriptions: HashMap<String, usize>,
}


impl StreamManagerState {
    /// Create new instance of the struct
    pub fn new(connection: BasicXtbStreamConnection) -> Self {
        Self {
            connection,
            subscriptions: HashMap::new(),
        }
    }
}


/// Manage stream subscriptions across application. All instances cloned from same origin share
/// its internal state.
#[derive(Clone, Debug)]
struct StreamManager {
    /// The inner state shared between instances of the `StreamManager`
    state: Arc<Mutex<StreamManagerState>>,
}


impl StreamManager {
    /// Create new instance of the `StreamManager` struct.
    pub fn new(connection: BasicXtbStreamConnection) -> Self {
        let state = Arc::new(Mutex::new(StreamManagerState::new(connection)));
        Self {
            state
        }
    }

    /// Subscribe for a stream from the stream API server.
    ///
    /// # Parameters
    ///
    /// * `subscribe_command` - command name of the subscribe command (e.g. `getCandles`)
    /// * `subscribe_arguments` - arguments for the subscribe command
    /// * `unsubscribe_command` - command name of the unsubscribe command (e.g. `stopCandles`)
    /// * `unsubscribe_arguments` - arguments for the unsubscribe command
    /// * `subscription_key` - key used to track number of subscribers of the data stream
    /// * `filter` - the filter predicate for message routing.
    ///
    /// # Returns
    ///
    /// * `Ok<DataStream<T>>` - data stream with filter set to messages related to sent command
    /// * `Err<XtbClientError>` - unable to send command
    pub async fn subscribe<T: for<'de> Deserialize<'de> + Send + Sync>(
        &mut self,
        subscribe_command: &str,
        subscribe_arguments: Option<Value>,
        unsubscribe_command: &str,
        unsubscribe_arguments: Option<Value>,
        subscription_key: &str,
        filter: DataMessageFilter,
    ) -> Result<DataStream<T>, XtbClientError> {
        let mut state = self.state.lock().await;
        let stream = state.connection.make_message_stream(filter).await;
        state.connection.subscribe(subscribe_command, subscribe_arguments).await.map_err(|err| XtbClientError::CannotSendStreamCommand(err))?;
        *state.subscriptions.entry(subscription_key.to_owned()).or_default() += 1;
        Ok(DataStream::new(stream, self.clone(), subscription_key.to_owned(), unsubscribe_command.to_owned(), unsubscribe_arguments))
    }

    /// Unsubscribe from a stream.
    ///
    /// # Parameters
    ///
    /// * `subscription_key` - the subscription key where number of subscribers is tracked
    /// * `command` - an unsubscribe command to be sent to the server
    /// * `arguments` - the unsubscribe command arguments
    ///
    /// # Returns
    ///
    /// * `Ok(())` - success
    /// * `Err(XtbClientError::CannotSendStreamCommand)` - fail
    pub async fn unsubscribe(&mut self, subscription_key: &str, command: &str, arguments: Option<Value>) -> Result<(), XtbClientError> {
        let mut state = self.state.lock().await;
        let mut entry = state.subscriptions.entry(subscription_key.to_owned()).or_default();
        if *entry > 0 {
            *entry -= 1;
        }
        if *entry == 0 {
            state.connection.unsubscribe(command, arguments).await.map_err(|err| XtbClientError::CannotSendStreamCommand(err))?;
        }
        Ok(())
    }
}


/// Stream of messages delivered to a consumer.
///
/// The message data is deserialized and typed to data type related to a command.
pub struct DataStream<T>
    where
        T: for<'de> Deserialize<'de> + Send + Sync
{
    /// The message stream with raw messages
    message_stream: BasicMessageStream,
    /// The stream manager used to unsubscribe from a stream when struct is dropped
    stream_manager: StreamManager,
    /// Internal subscription key for subscriber tracking
    subscription_key: String,
    /// Unsubscribe command to be sent to the XTB server
    unsubscribe_command: String,
    /// Unsubscribe command arguments
    unsubscribe_arguments: Option<Value>,
    /// Data type returned to a consumer
    type_: PhantomData<T>,
}

impl<T> DataStream<T>
    where
        T: for<'de> Deserialize<'de> + Send + Sync
{
    /// Create new instance of the stream.
    fn new(message_stream: BasicMessageStream, stream_manager: StreamManager, subscription_key: String, unsubscribe_command: String, unsubscribe_arguments: Option<Value>) -> Self {
        Self {
            message_stream,
            stream_manager,
            subscription_key,
            unsubscribe_command,
            unsubscribe_arguments,
            type_: PhantomData::<T>,
        }
    }

    /// Wait and get next message from the stream.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(T))` - next message in stream.
    /// * `Ok(None)` - there is no message left
    /// * `Err(DataStreamError)` - message was recived but cannot be processed. A next message can be ok.
    pub async fn next(&mut self) -> Result<Option<T>, DataStreamError> {
        let message = self.message_stream.next().await;
        match message {
            Some(msg) => Self::process_message(msg).map(|r| Some(r)),
            None => Ok(None),
        }
    }

    /// Deserialize serialized data representation to actual type `T`.
    fn process_message(msg: StreamDataMessage) -> Result<T, DataStreamError> {
        from_value(msg.data).map_err(|err| DataStreamError::CannotDeserializeValue(err))
    }
}

impl<T> Drop for DataStream<T>
    where
        T: for<'de> Deserialize<'de> + Send + Sync
{
    fn drop(&mut self) {
        let mut manager = self.stream_manager.clone();
        let unsubscribe_command = self.unsubscribe_command.clone();
        let unsubscribe_arguments = self.unsubscribe_arguments.take();
        let subscription_key = self.subscription_key.clone();
        spawn(async move {
            let result = manager.unsubscribe(&subscription_key, &unsubscribe_command, unsubscribe_arguments.clone()).await;
            match result {
                Err(err) => error!("Cannot unsubscribe command '{unsubscribe_command}' ({unsubscribe_arguments:?}). The subscription key was: '{subscription_key}'. The error was: {err:?}"),
                _ => (),
            };
        });
    }
}

#[derive(Debug, Error)]
pub enum DataStreamError {
    #[error("Cannot deserialize value: {0}")]
    CannotDeserializeValue(serde_json::Error)
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
                    Err(err) => error!("Cannot await the ping response #{}: {:?}", idx, err)
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
fn spawn_stream_ping(stream_manager: StreamManager, ping_secs: u64) -> JoinHandle<()> {
    let ping_value = to_value(StreamPingSubscribe::default()).expect("Cannot serialize the stream ping message");
    spawn(async move {
        let mut idx = 1u64;
        loop {
            {
                debug!("Sending ping #{} to stream connection", idx);
                let mut inner_state = stream_manager.state.lock().await;
                match inner_state.connection.subscribe(STREAM_PING, Some(ping_value.clone())).await {
                    Ok(_) => (),
                    Err(err) => error!("Cannot send ping #{}: {:?}", idx, err)
                }
            }
            idx += 1;
            sleep(Duration::from_secs(ping_secs)).await;
        }
    })
}
