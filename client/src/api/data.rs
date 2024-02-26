use std::ops::{Deref, DerefMut};
use std::time::SystemTime;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use crate::api::enums::{ImpactLevel, MarginMode, ProfitMode, QuoteId, TimePeriod, TradeStatus, TradingAction, TradingCommand, TransactionStatus, TransactionType};

/// Structure representing user's login data
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// userId
    pub user_id: String,
    /// password
    pub password: String,
    /// (optional, deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// (optional) application name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
}


/// Only logic struct to keep symmetry.
/// The login command has no response.
#[derive(Default, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct LoginResponse;


/// Only logic struct to keep symmetry.
/// The getAllSymbols command has no request data.
#[derive(Default, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetAllSymbolsRequest;


/// Wrapper for symbols
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct GetAllSymbolsResponse(pub Vec<SymbolRecord>);

impl Deref for GetAllSymbolsResponse {
    type Target = Vec<SymbolRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for GetAllSymbolsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Structure representing details of a financial symbol
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct SymbolRecord {
    /// Ask price in base currency
    pub ask: f32,
    /// Bid price in base currency
    pub bid: f32,
    /// Category name
    pub category_name: String,
    /// Size of 1 lot
    pub contract_size: i64,
    /// Currency
    pub currency: String,
    /// Indicates whether the symbol represents a currency pair
    pub currency_pair: bool,
    /// The currency of calculated profit
    pub currency_profit: String,
    /// Description
    pub description: String,
    /// Null if not applicable
    pub expiration: Option<u64>,
    /// Symbol group name
    pub group_name: String,
    /// The highest price of the day in base currency
    pub high: f32,
    /// Initial margin for 1 lot order, used for profit/margin calculation
    pub initial_margin: i64,
    /// Maximum instant volume multiplied by 100 (in lots)
    pub instant_max_volume: i64,
    /// Symbol leverage
    pub leverage: f32,
    /// Long only
    pub long_only: bool,
    /// Maximum size of trade
    pub lot_max: f32,
    /// Minimum size of trade
    pub lot_min: f32,
    /// A value of minimum step by which the size of trade can be changed (within lotMin - lotMax range)
    pub lot_step: f32,
    /// The lowest price of the day in base currency
    pub low: f32,
    /// Used for profit calculation
    pub margin_hedged: i64,
    /// For margin calculation
    pub margin_hedged_strong: bool,
    /// For margin calculation, null if not applicable
    pub margin_maintenance: Option<i64>,
    /// For margin calculation
    pub margin_mode: MarginMode,
    /// Percentage
    pub percentage: f32,
    /// Number of symbol's pip decimal places
    pub pips_precision: i64,
    /// Number of symbol's price decimal places
    pub precision: i64,
    /// For profit calculation
    pub profit_mode: ProfitMode,
    /// Source of price
    pub quote_id: QuoteId,
    /// Indicates whether short selling is allowed on the instrument
    pub short_selling: bool,
    /// The difference between raw ask and bid prices
    pub spread_raw: f32,
    /// Spread representation
    pub spread_table: f32,
    /// Null if not applicable
    pub starting: Option<u64>,
    /// Appropriate step rule ID from getStepRules command response
    pub step_rule_id: i64,
    /// Minimal distance (in pips) from the current price where the stopLoss/takeProfit can be set
    pub stops_level: i64,
    /// Time when additional swap is accounted for weekend
    #[serde(rename = "swap_rollover3days")]
    pub swap_rollover_3days: i64,
    /// Indicates whether swap value is added to position on end of day
    pub swap_enable: bool,
    /// Swap value for long positions in pips
    pub swap_long: f32,
    /// Swap value for short positions in pips
    pub swap_short: f32,
    /// Type of swap calculated
    pub swap_type: i64,
    /// Symbol name
    pub symbol: String,
    /// Smallest possible price change, used for profit/margin calculation, null if not applicable
    pub tick_size: Option<f32>,
    /// Value of smallest possible price change (in base currency), used for profit/margin calculation, null if not applicable
    pub tick_value: Option<f32>,
    /// Ask & bid tick time
    pub time: u64,
    /// Time in String
    pub time_string: String,
    /// Indicates whether trailing stop (offset) is applicable to the instrument.
    pub trailing_enabled: bool,
    /// Instrument class number
    pub type_: i64,
}


/// The request has no data. This struct exists only to keep symmetry
#[derive(Default, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetCalendarRequest;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetCalendarResponse(pub Vec<CalendarRecord>);


impl Deref for GetCalendarResponse {
    type Target = Vec<CalendarRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GetCalendarResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Structure representing details of a calendar record
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct CalendarRecord {
    /// Two-letter country code
    pub country: String,
    /// Market value (current), empty before time of release of this value (time from "time" record)
    pub current: String,
    /// Forecasted value
    pub forecast: String,
    /// Impact on market
    pub impact: ImpactLevel,
    /// Information period
    pub period: String,
    /// Value from previous information release
    pub previous: String,
    /// Time, when the information will be released (in this time empty "current" value should be changed with exact released value)
    pub time: u64,
    /// Name of the indicator for which values will be released
    pub title: String,
}


/// Get chart information
/// Please note that this function can be usually replaced by its streaming equivalent getCandles
/// which is the preferred way of retrieving current candle data.
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetChartLastRequestRequest {
    info: ChartLastInfoRecord
}


/// Specification and constraints for the requested data
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct ChartLastInfoRecord {
    /// Period code
    pub period: TimePeriod,
    /// Start of chart block (rounded down to the nearest interval and excluding)
    pub start: u64,
    /// Symbol
    pub symbol: String,
}


/// Structure representing details of rate information
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetChartLastRequestResponse {
    /// Number of decimal places
    pub digits: i64,
    /// Array of RATE_INFO_RECORD
    pub rate_infos: Vec<RateInfoRecord>,
}


/// Structure representing details of a rate
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct RateInfoRecord {
    /// Value of close price (shift from open price)
    pub close: f32,
    /// Candle start time in CET/CEST time zone (see Daylight Saving Time, DST)
    pub ctm: u64,
    /// String representation of the 'ctm' field
    pub ctm_string: String,
    /// Highest value in the given period (shift from open price)
    pub high: f32,
    /// Lowest value in the given period (shift from open price)
    pub low: f32,
    /// Open price (in base currency * 10 to the power of digits)
    pub open: f32,
    /// Volume in lots
    pub vol: f32,
}


/// Structure representing chart range details
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetChartRangeRequestRequest {
    /// Chart range information
    pub info: ChartRangeInfoRecord,
}


/// Structure representing chart range information
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct ChartRangeInfoRecord {
    /// End of chart block (rounded down to the nearest interval and excluding)
    pub end: u64,
    /// Period code
    pub period: TimePeriod,
    /// Start of chart block (rounded down to the nearest interval and excluding)
    pub start: u64,
    /// Symbol
    pub symbol: String,
    /// Number of ticks needed, this field is optional
    pub ticks: Option<i64>,
}


/// Response for the `getChartRangeRequest` is same as `getChartLastRequest`
pub type GetChartRangeRequestResponse = GetChartLastRequestResponse;


/// Request for calculation of commission
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionDefRequest {
    /// Symbol
    pub symbol: String,
    /// Volume
    pub volume: f32,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
struct GetCommissionDefResponse {
    /// Calculated commission in account currency, could be null if not applicable
    commission: Option<f32>,
    /// Rate of exchange between account currency and instrument base currency, could be null if not applicable
    rate_of_exchange: Option<f32>,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetCurrentUserDataRequest;


/// Structure representing account details
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentUserDataResponse {
    /// Unit the account is assigned to
    pub company_unit: i64,
    /// Account currency
    pub currency: String,
    /// Group
    pub group: String,
    /// Indicates whether this account is an IB account
    pub ib_account: bool,
    /// This field should not be used. It is inactive and its value is always 1
    pub leverage: i64,
    /// The factor used for margin calculations. The actual value of leverage can be calculated by dividing this value by 100
    pub leverage_multiplier: f32,
    /// SpreadType, null if not applicable
    pub spread_type: Option<String>,
    /// Indicates whether this account is enabled to use trailing stop
    pub trailing_stop: bool,
}


/// Structure representing IB's history block
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetIbsHistoryRequest {
    /// End of IBs history block
    pub end: u64,
    /// Start of IBs history block
    pub start: u64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetIbsHistoryResponse(Vec<IBRecord>);


impl Deref for GetIbsHistoryResponse {
    type Target = Vec<IBRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for GetIbsHistoryResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Structure representing IB data
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct IBRecord {
    /// IB close price or null if not allowed to view
    pub close_price: Option<f32>,
    /// IB user login or null if not allowed to view
    pub login: Option<String>,
    /// IB nominal or null if not allowed to view
    pub nominal: Option<f32>,
    /// IB open price or null if not allowed to view
    pub open_price: Option<f32>,
    /// Operation code or null if not allowed to view
    pub side: Option<TradingAction>,
    /// IB user surname or null if not allowed to view
    pub surname: Option<String>,
    /// Symbol or null if not allowed to view
    pub symbol: Option<String>,
    /// Time the record was created or null if not allowed to view
    pub timestamp: Option<u64>,
    /// Volume in lots or null if not allowed to view
    pub volume: Option<f32>,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetMarginLevelRequest;


/// Structure representing account financial information
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetMarginLevelResponse {
    /// Balance in account currency
    pub balance: f32,
    /// credit
    pub credit: f32,
    /// User currency
    pub currency: String,
    /// Sum of balance and all profits in account currency
    pub equity: f32,
    /// Margin requirements in account currency
    pub margin: f32,
    /// Free margin in account currency
    #[serde(rename = "margin_free")]
    pub margin_free: f32,
    /// Margin level percentage
    #[serde(rename = "margin_level")]
    pub margin_level: f32,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetMarginTradeRequest {
    /// Symbol
    pub symbol: String,
    /// volume
    pub volume: f32,
}


/// Structure representing margin information
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetMarginTradeResponse {
    /// Calculated margin in account currency
    pub margin: f32,
}


/// Structure representing a specific Time span
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetNewsRequest {
    /// End time. 0 indicates the current time for simplicity.
    pub end: u64,
    /// Start time.
    pub start: u64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetNewsResponse(Vec<NewsBodyRecord>);

impl Deref for GetNewsResponse {
    type Target = Vec<NewsBodyRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GetNewsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Structure representing a news item
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct NewsBodyRecord {
    /// News body content
    pub body: String,
    /// Length of the news body content
    #[serde(rename = "bodylen")]
    pub body_len: usize,
    /// The key of the news item
    pub key: String,
    /// Time when the news was posted, represented as a UNIX timestamp
    pub time: u64,
    /// Time when the news was posted, presented in string format
    pub time_string: String,
    /// Title of the news item
    pub title: String,
}

/// Structure representing the order with operation code, price and volume
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetProfitCalculationRequest {
    /// Theoretical close price of order
    pub close_price: f32,
    /// Operation code
    pub cmd: TradingCommand,
    /// Theoretical open price of order
    pub open_price: f32,
    /// Symbol
    pub symbol: String,
    /// Volume
    pub volume: f32,
}


/// Structure representing profit information
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetProfitCalculationResponse {
    /// Profit in account currency
    pub profit: f32,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetServerTimeRequest;


/// Structure representing time information
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetServerTimeResponse {
    /// Time represented as a UNIX timestamp
    pub time: u64,
    /// Time described in form set on server (local time of server)
    pub time_string: String,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetStepRulesRequest;

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetStepRulesResponse(Vec<StepRuleRecord>);


impl Deref for GetStepRulesResponse {
    type Target = Vec<StepRuleRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GetStepRulesResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Structure representing a step rule
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StepRuleRecord {
    /// The ID of the step rule
    pub id: u32,
    /// The name of the step rule
    pub name: String,
    /// Array of `STEP_RECORD`
    // `STEP_RECORD` needs to be replaced with the actual type
    pub steps: Vec<StepRecord>,
}


/// Structure representing a record step
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StepRecord {
    /// Lower border of the volume range
    pub from_value: f32,
    /// LotStep value in the given volume range
    pub step: f32,
}


/// Structure representing a symbol
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetSymbolRequest {
    /// Symbol for the record
    pub symbol: String,
}


pub type GetSymbolResponse = SymbolRecord;


/// Structure representing a level
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetTickPricesRequest {
    /// Price level
    pub level: u32,
    /// Array of symbol names
    pub symbols: Vec<String>,
    /// The time from which the most recent tick should be looked for
    pub timestamp: u64,
}


/// Structure representing market price data
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetTickPricesResponse {
    /// Ask price in base currency
    pub ask: f32,
    /// Number of available lots to buy at given price or None if not applicable
    pub ask_volume: Option<u32>,
    /// Bid price in base currency
    pub bid: f32,
    /// Number of available lots to sell at given price or None if not applicable
    pub bid_volume: Option<u32>,
    /// The highest price of the day in base currency
    pub high: f32,
    /// Price level
    pub level: u32,
    /// The lowest price of the day in base currency
    pub low: f32,
    /// The difference between raw ask and bid prices
    pub spread_raw: f32,
    /// Spread representation
    pub spread_table: f32,
    /// Symbol
    pub symbol: String,
    /// Timestamp in UNIX time
    pub timestamp: u64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetTradeRecordsRequest {
    /// Array of orders (position numbers)
    pub orders: Vec<u32>,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetTradeRecordsResponse(Vec<TradeRecord>);


impl Deref for GetTradeRecordsResponse {
    type Target = Vec<TradeRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for GetTradeRecordsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Structure representing a trade order
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct TradeRecord {
    /// Close price in base currency
    #[serde(rename = "close_price")]
    pub close_price: f32,
    /// Null if order is not closed
    #[serde(rename = "close_time")]
    pub close_time: Option<u64>,
    /// Null if order is not closed
    #[serde(rename="close_timeString")]
    pub close_time_string: Option<String>,
    /// Closed
    pub closed: bool,
    /// Operation code
    pub cmd: TradingCommand,
    /// Comment
    pub comment: String,
    /// Commission in account currency, null if not applicable
    pub commission: Option<f32>,
    /// The value the customer may provide in order to retrieve it later.
    pub custom_comment: String,
    /// Number of decimal places
    pub digits: u32,
    /// Null if order is not closed
    pub expiration: Option<u64>,
    /// Null if order is not closed
    pub expiration_string: Option<String>,
    /// Margin rate
    pub margin_rate: f32,
    /// Trailing offset
    pub offset: u32,
    /// Open price in base currency
    #[serde(rename = "open_price")]
    pub open_price: f32,
    /// Open time
    #[serde(rename = "open_time")]
    pub open_time: u64,
    /// Open time string
    #[serde(rename = "open_timeString")]
    pub open_time_string: String,
    /// Order number for opened transaction
    pub order: u32,
    /// Order number for closed transaction
    pub order2: u32,
    /// Order number common both for opened and closed transaction
    pub position: u32,
    /// Profit in account currency
    pub profit: f32,
    /// Zero if stop loss is not set (in base currency)
    pub sl: f32,
    /// Order swaps in account currency
    pub storage: f32,
    /// Symbol name or null for deposit/withdrawal operations
    pub symbol: Option<String>,
    /// Timestamp
    pub timestamp: u64,
    /// Zero if take profit is not set (in base currency)
    pub tp: f32,
    /// Volume in lots
    pub volume: f32,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetTradesRequest {
    /// Whether only opened trades will be returned
    pub opened_only: bool,
}


/// List of trade records
pub type GetTradesResponse = GetTradeRecordsResponse;


/// Structure representing a time interval
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetTradesHistoryRequest {
    /// End time, equals to current time if set to 0
    pub end: u64,
    /// Start time, refers to the last month interval if set to 0
    pub start: u64,
}


/// List of trade records
pub type GetTradesHistoryResponse = GetTradeRecordsResponse;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetTradingHoursRequest {
    /// Array of symbol names
    pub symbols: Vec<String>,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetTradingHoursResponse(Vec<TradingHoursRecord>);


impl Deref for GetTradingHoursResponse {
    type Target = Vec<TradingHoursRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GetTradingHoursResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct TradingHoursRecord {
    /// Array of QUOTES_RECORD
    pub quotes: Vec<HoursRecord>,
    /// Symbol
    pub symbol: String,
    /// Array of TRADING_RECORD
    pub trading: Vec<HoursRecord>,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct HoursRecord {
    /// Day of the week
    pub day: u8,
    /// Start time in ms from 00:00 CET / CEST time zone
    pub from_t: u64,
    /// End time in ms from 00:00 CET / CEST time zone
    pub to_t: u64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetVersionRequest;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct GetVersionResponse {
    /// Current API version
    pub version: String,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PingRequest;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PingResponse;


/// Structure embedding a TRADE_TRANS_INFO instance
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct TradeTransactionRequest {
    /// The embedded TRADE_TRANS_INFO
    pub trade_trans_info: TradeTransInfo,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct TradeTransInfo {
    /// Operation code
    pub cmd: TradingCommand,
    /// The value the customer may provide in order to retrieve it later.
    pub custom_comment: String,
    /// Pending order expiration time
    pub expiration: u64,
    /// Trailing offset
    pub offset: i32,
    /// 0 or position number for closing/modifications
    pub order: i32,
    /// Trade price
    pub price: f64,
    /// Stop loss
    pub sl: f64,
    /// Trade symbol
    pub symbol: String,
    /// Take profit
    pub tp: f64,
    /// Trade transaction type
    pub type_: TransactionType,
    /// Trade volume
    pub volume: f64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct TradeTransactionResponse {
    /// order
    pub order: i32,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct TradeTransactionStatusRequest {
    /// order
    pub order: i32,
}


/// Structure representing the response
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct TradeTransactionStatusResponse {
    /// Price in base currency
    pub ask: f64,
    /// Price in base currency
    pub bid: f64,
    /// The value the customer may provide in order to retrieve it later
    pub custom_comment: String,
    /// Can be null
    pub message: Option<String>,
    /// Unique order number
    pub order: i32,
    /// Request status code
    pub request_status: TransactionStatus,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetBalanceSubscribe;

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetBalanceUnsubscribe;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetBalanceData {
    /// Balance in account currency
    pub balance: f64,
    /// Credit in account currency
    pub credit: f64,
    /// Sum of balance and all profits in account currency
    pub equity: f64,
    /// Margin requirements
    pub margin: f64,
    /// Free margin
    pub margin_free: f64,
    /// Margin level percentage
    pub margin_level: f64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetCandlesSubscribe {
    /// Symbol
    pub symbol: String,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetCandlesUnsubscribe {
    /// Symbol
    pub symbol: String,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetCandlesData {
    /// Close price in base currency
    pub close: f64,
    /// Candle start time in CET time zone (Central European Time)
    pub ctm: u64,
    /// String representation of the ctm field
    pub ctm_string: String,
    /// Highest value in the given period in base currency
    pub high: f64,
    /// Lowest value in the given period in base currency
    pub low: f64,
    /// Open price in base currency
    pub open: f64,
    /// Source of price
    pub quote_id: QuoteId,
    /// Symbol
    pub symbol: String,
    /// Volume in lots
    pub vol: f64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetKeepAliveSubscribe;

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetKeepAliveUnsubscribe;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetKeepAliveData {
    /// Current timestamp
    pub timestamp: u64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetNewsSubscribe;

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetNewsUnsubscribe;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetNewsData {
    /// Body content of the news article
    pub body: String,
    /// Unique identifier for the news article
    pub key: String,
    /// Time of the news article
    pub time: u64,
    /// Title of the news article
    pub title: String,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetProfitSubscribe;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetProfitUnsubscribe;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetProfitData {
    /// Order number
    pub order: i32,
    /// Transaction ID
    pub order2: i32,
    /// Position number
    pub position: i32,
    /// Profit in account currency
    pub profit: f64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetTickPricesSubscribe {
    /// Financial instrument symbol
    pub symbol: String,
    /// Minimal interval in milliseconds between any two consecutive updates. It is optional.
    pub min_arrival_time: Option<u64>,
    /// Specifies the maximum level of the quote that the user is interested in. It is optional.
    pub max_level: Option<u64>,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetTickPricesUnsubscribe {
    /// Financial instrument symbol
    pub symbol: String,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetTickPricesData {
    /// Ask price in base currency
    pub ask: f64,
    /// Number of available lots to buy at given price
    pub ask_volume: Option<i32>,
    /// Bid price in base currency
    pub bid: f64,
    /// Number of available lots to sell at given price
    pub bid_volume: Option<i32>,
    /// The highest price of the day in base currency
    pub high: f64,
    /// Price level
    pub level: i32,
    /// The lowest price of the day in base currency
    pub low: f64,
    /// Source of price
    pub quote_id: QuoteId,
    /// The difference between raw ask and bid prices
    pub spread_raw: f64,
    /// Spread representation
    pub spread_table: f64,
    /// Financial instrument symbol
    pub symbol: String,
    /// Time when the information was updated
    pub timestamp: u64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetTradesSubscribe;

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetTradesUnsubscribe;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetTradesData {
    /// Close price in base currency
    #[serde(rename = "close_price")]
    pub close_price: f64,
    /// Close time, null if order is not closed
    #[serde(rename = "close_time")]
    pub close_time: Option<u64>,
    /// Is the order closed
    pub closed: bool,
    /// Operation code
    pub cmd: TradingCommand,
    /// Comment
    pub comment: String,
    /// Commission in account currency, null if not applicable
    pub commission: Option<f64>,
    /// Custom comment
    pub custom_comment: String,
    /// Number of decimal places
    pub digits: i32,
    /// Expiration time, null if order is not closed
    pub expiration: Option<u64>,
    /// Margin rate
    #[serde(rename = "margin_rate")]
    pub margin_rate: f64,
    /// Trailing offset
    pub offset: i32,
    /// Open price in base currency
    #[serde(rename = "open_price")]
    pub open_price: f64,
    /// Open time
    #[serde(rename = "open_time")]
    pub open_time: u64,
    /// Order number for opened transaction
    pub order: i32,
    /// Transaction id
    pub order2: i32,
    /// Position number (if type is 0 and 2) or transaction parameter (if type is 1)
    pub position: i32,
    /// Profit, null unless the trade is closed (type=2) or opened (type=0)
    pub profit: Option<f64>,
    /// Stop loss amount, zero if not set (in base currency)
    pub sl: f64,
    /// Trade state, should be used for detecting pending order's cancellation
    pub state: TradeStatus,
    /// Storage
    pub storage: f64,
    /// Financial instrument symbol
    pub symbol: String,
    /// Take profit amount, zero if not set (in base currency)
    pub tp: f64,
    /// Type
    pub type_: TransactionType,
    /// Volume in lots
    pub volume: f64,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetTradeStatusSubscribe;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamGetTradeStatusUnsubscribe;


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, Setters)]
#[setters(into, strip_option, prefix = "with_")]
#[serde(rename_all = "camelCase")]
pub struct StreamGetTradeStatusData {
    /// The value the customer may provide in order to retrieve it later
    pub custom_comment: String,
    /// Message, can be null
    pub message: Option<String>,
    /// Unique order number
    pub order: i32,
    /// Price in base currency
    pub price: f64,
    /// Request status code
    pub request_status: TransactionStatus,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StreamPingSubscribe;


#[cfg(test)]
mod tests {
    use serde_json::Value;

    fn assert_all_keys(value: &Value, keys: Vec<&str>) {
        assert!(value.is_object());
        let Value::Object(mapping) = value else { unreachable!() };
        let extra_keys: Vec<_> = mapping.keys().filter(|k| !keys.contains(&k.as_str())).map(|k| k.to_owned()).collect();
        let missing_keys: Vec<_> = keys.into_iter().filter(|k| !mapping.contains_key(k.to_string().as_str())).map(|k| k.to_owned()).collect();

        if !extra_keys.is_empty() || !missing_keys.is_empty() {
            panic!("Keys in value does not match to expected keys.\nExtra keys: {:?}\nMissing keys: {:?}", extra_keys, missing_keys);
        }
    }

    mod serialize_deserialize {
        use std::fmt::Debug;
        use std::ops::Deref;
        use rstest::rstest;
        use serde::{Deserialize, Serialize};
        use serde_json::{from_str, from_value, to_string, to_value};
        use crate::api::data::{StreamPingSubscribe, StreamGetTradeStatusSubscribe, StreamGetTradeStatusUnsubscribe, StreamGetTradesData, StreamGetTradesSubscribe, StreamGetTradesUnsubscribe, StreamGetTickPricesSubscribe, StreamGetTickPricesUnsubscribe, StreamGetTickPricesData, StreamGetProfitData, StreamGetProfitSubscribe, StreamGetProfitUnsubscribe, StreamGetKeepAliveData, StreamGetNewsSubscribe, StreamGetNewsData, StreamGetNewsUnsubscribe, StreamGetKeepAliveSubscribe, StreamGetKeepAliveUnsubscribe, StreamGetCandlesSubscribe, StreamGetCandlesUnsubscribe, StreamGetBalanceData, StreamGetCandlesData, StreamGetBalanceSubscribe, StreamGetBalanceUnsubscribe, TradeTransactionStatusResponse, TradeTransactionStatusRequest, TradeTransactionResponse, TradeTransInfo, TradeTransactionRequest, GetVersionResponse, PingRequest, PingResponse, GetVersionRequest, TradingHoursRecord, HoursRecord, GetTradingHoursResponse, GetTradingHoursRequest, GetTradesHistoryRequest, GetTradesRequest, GetTradeRecordsResponse, TradeRecord, GetTradeRecordsRequest, GetTickPricesResponse, GetTickPricesRequest, GetSymbolRequest, GetStepRulesResponse, StepRuleRecord, StepRecord, GetStepRulesRequest, GetServerTimeRequest, GetServerTimeResponse, GetProfitCalculationRequest, GetProfitCalculationResponse, GetNewsRequest, GetNewsResponse, NewsBodyRecord, GetMarginTradeRequest, GetMarginTradeResponse, GetMarginLevelRequest, GetMarginLevelResponse, IBRecord, GetIbsHistoryResponse, GetCurrentUserDataResponse, GetCurrentUserDataRequest, GetCommissionDefRequest, GetCommissionDefResponse, GetChartRangeRequestRequest, ChartRangeInfoRecord, GetChartLastRequestResponse, RateInfoRecord, ChartLastInfoRecord, GetChartLastRequestRequest, GetCalendarResponse, CalendarRecord, GetAllSymbolsRequest, GetAllSymbolsResponse, LoginRequest, LoginResponse, SymbolRecord, GetCalendarRequest, StreamGetTradeStatusData};
        use crate::api::data::tests::assert_all_keys;

        #[rstest]
        #[case::LoginRequest_1(LoginRequest::default(), vec ! ["userId", "password"])]
        #[case::LoginRequest_2(LoginRequest::default().with_app_id("foo").with_app_name("bar"), vec ! ["userId", "password", "appId", "appName"])]
        #[case::SymbolRecord_1(SymbolRecord::default(), vec ! ["ask", "bid", "categoryName", "contractSize", "currency", "currencyPair", "currencyProfit", "description", "expiration", "groupName", "high", "initialMargin", "instantMaxVolume", "leverage", "longOnly", "lotMax", "lotMin", "lotStep", "low", "marginHedged", "marginHedgedStrong", "marginMaintenance", "marginMode", "percentage", "pipsPrecision", "precision", "profitMode", "quoteId", "shortSelling", "spreadRaw", "spreadTable", "starting", "stepRuleId", "stopsLevel", "swap_rollover3days", "swapEnable", "swapLong", "swapShort", "swapType", "symbol", "tickSize", "tickValue", "time", "timeString", "trailingEnabled", "type"])]
        #[case::CalendarRecord_1(CalendarRecord::default(), vec!["country", "current", "forecast", "impact", "period", "previous", "time", "title"])]
        #[case::GetChartLastRequestRequest_1(GetChartLastRequestRequest::default(), vec!["info"])]
        #[case::ChartLastInfoRecord_1(ChartLastInfoRecord::default(), vec!["period", "start", "symbol"])]
        #[case::GetChartLastRequestResponse_1(GetChartLastRequestResponse::default(), vec!["digits", "rateInfos"])]
        #[case::RateInfoRecord_1(RateInfoRecord::default(), vec!["close", "ctm", "ctmString", "high", "low", "open", "vol"])]
        #[case::GetChartRangeRequestRequest_1(GetChartRangeRequestRequest::default(), vec!["info"])]
        #[case::ChartRangeInfoRecord_1(ChartRangeInfoRecord::default(), vec!["end", "period", "start", "symbol", "ticks"])]
        #[case::GetCommissionDefRequest_1(GetCommissionDefRequest::default(), vec!["symbol", "volume"])]
        #[case::GetCommissionDefResponse_1(GetCommissionDefResponse::default(), vec!["commission", "rateOfExchange"])]
        #[case::GetCurrentUserDataResponse_1(GetCurrentUserDataResponse::default(), vec!["companyUnit", "currency", "group", "ibAccount", "leverage", "leverageMultiplier", "spreadType", "trailingStop"])]
        #[case::IBRecord_1(IBRecord::default(), vec!["closePrice", "login", "nominal", "openPrice", "side", "surname", "symbol", "timestamp", "volume"])]
        #[case::GetMarginLevelResponse_1(GetMarginLevelResponse::default(), vec!["balance", "credit", "currency", "equity", "margin", "margin_free", "margin_level"])]
        #[case::GetMarginTradeRequest_1(GetMarginTradeRequest::default(), vec!["symbol", "volume"])]
        #[case::GetMarginTradeResponse_1(GetMarginTradeResponse::default(), vec!["margin"])]
        #[case::GetNewsRequest_1(GetNewsRequest::default(), vec!["end", "start"])]
        #[case::NewsBodyRecord_1(NewsBodyRecord::default(), vec!["body", "bodylen", "key", "time", "timeString", "title"])]
        #[case::GetProfitCalculationRequest_1(GetProfitCalculationRequest::default(), vec!["closePrice", "cmd", "openPrice", "symbol", "volume"])]
        #[case::GetProfitCalculationResponse_1(GetProfitCalculationResponse::default(), vec!["profit"])]
        #[case::GetServerTimeResponse_1(GetServerTimeResponse::default(), vec!["time", "timeString"])]
        #[case::StepRuleRecord_1(StepRuleRecord::default(), vec!["id", "name", "steps"])]
        #[case::StepRecord_1(StepRecord::default(), vec!["fromValue", "step"])]
        #[case::GetSymbolRequest_1(GetSymbolRequest::default(), vec!["symbol"])]
        #[case::GetTickPricesRequest_1(GetTickPricesRequest::default(), vec!["level", "symbols", "timestamp"])]
        #[case::GetTickPricesResponse_1(GetTickPricesResponse::default(), vec!["ask", "askVolume", "bid", "bidVolume", "high", "level", "low", "spreadRaw", "spreadTable", "symbol", "timestamp"])]
        #[case::GetTradeRecordsRequest_1(GetTradeRecordsRequest::default(), vec!["orders"])]
        #[case::TradeRecord_1(TradeRecord::default(), vec!["close_price", "close_time", "close_timeString", "closed", "cmd", "comment", "commission", "customComment", "digits", "expiration", "expirationString", "marginRate", "offset", "open_price", "open_time", "open_timeString", "order", "order2", "position", "profit", "sl", "storage", "symbol", "timestamp", "tp", "volume"])]
        #[case::GetTradesRequest_1(GetTradesRequest::default(), vec!["openedOnly"])]
        #[case::GetTradesHistoryRequest_1(GetTradesHistoryRequest::default(), vec!["end", "start"])]
        #[case::GetTradingHoursRequest_1(GetTradingHoursRequest::default(), vec!["symbols"])]
        #[case::TradingHoursRecord_1(TradingHoursRecord::default(), vec!["quotes", "symbol", "trading"])]
        #[case::HoursRecord_1(HoursRecord::default(), vec!["day", "fromT", "toT"])]
        #[case::GetVersionResponse_1(GetVersionResponse::default(), vec!["version"])]
        #[case::TradeTransactionRequest_1(TradeTransactionRequest::default(), vec!["tradeTransInfo"])]
        #[case::TradeTransInfo_1(TradeTransInfo::default(), vec!["cmd", "customComment", "expiration", "offset", "order", "price", "sl", "symbol", "tp", "type", "volume"])]
        #[case::TradeTransactionResponse_1(TradeTransactionResponse::default(), vec!["order"])]
        #[case::TradeTransactionStatusRequest_1(TradeTransactionStatusRequest::default(), vec!["order"])]
        #[case::TradeTransactionStatusResponse_1(TradeTransactionStatusResponse::default(), vec!["ask", "bid", "customComment", "message", "order", "requestStatus"])]
        #[case::StreamGetBalanceData_1(StreamGetBalanceData::default(), vec!["balance", "credit", "equity", "margin", "marginFree", "marginLevel"])]
        #[case::StreamGetCandlesData_1(StreamGetCandlesData::default(), vec!["close", "ctm", "ctmString", "high", "low", "open", "quoteId", "symbol", "vol"])]
        #[case::StreamGetCandlesSubscribe_1(StreamGetCandlesSubscribe::default(), vec!["symbol"])]
        #[case::StreamGetCandlesUnsubscribe_1(StreamGetCandlesUnsubscribe::default(), vec!["symbol"])]
        #[case::StreamGetKeepAliveData_1(StreamGetKeepAliveData::default(), vec!["timestamp"])]
        #[case::StreamGetNewsData_1(StreamGetNewsData::default(), vec!["body", "key", "time", "title"])]
        #[case::StreamGetProfitData_1(StreamGetProfitData::default(), vec!["order", "order2", "position", "profit"])]
        #[case::StreamGetTickPricesSubscribe_1(StreamGetTickPricesSubscribe::default(), vec!["symbol", "minArrivalTime", "maxLevel"])]
        #[case::StreamGetTickPricesUnsubscribe_1(StreamGetTickPricesUnsubscribe::default(), vec!["symbol"])]
        #[case::StreamGetTickPricesData_1(StreamGetTickPricesData::default(), vec!["ask", "askVolume", "bid", "bidVolume", "high", "level", "low", "quoteId", "spreadRaw", "spreadTable", "symbol", "timestamp"])]
        #[case::StreamGetTradesData_1(StreamGetTradesData::default(), vec!["close_price", "close_time", "closed", "cmd", "comment", "commission", "customComment", "digits", "expiration", "margin_rate", "offset", "open_price", "open_time", "order", "order2", "position", "profit", "sl", "state", "storage", "symbol", "tp", "type", "volume"])]
        #[case::StreamGetTradeStatusData_1(StreamGetTradeStatusData::default(), vec!["customComment", "message", "order", "price", "requestStatus"])]
        fn serialize_deserialize_payload_struct<T: Serialize + Clone + Default + Debug + PartialEq + for<'de> Deserialize<'de>>(#[case] original: T, #[case] keys: Vec<&str>) {
            let serialized_value = to_value(original.clone()).unwrap();
            assert_all_keys(&serialized_value, keys);
            let new_value = from_value(serialized_value).unwrap();
            assert_eq!(original, new_value);
        }

        #[rstest]
        #[case::LoginResponse_1(LoginResponse)]
        #[case::GetAllSymbolsRequest_1(GetAllSymbolsRequest)]
        #[case::GetCalendarRequest_1(GetCalendarRequest)]
        #[case::GetCalendarRequest_1(GetCurrentUserDataRequest)]
        #[case::GetMarginLevelRequest_1(GetMarginLevelRequest)]
        #[case::GetServerTimeRequest_1(GetServerTimeRequest)]
        #[case::GetStepRulesRequest_1(GetStepRulesRequest)]
        #[case::GetVersionRequest_1(GetVersionRequest)]
        #[case::PingRequest_1(PingRequest)]
        #[case::PingResponse_1(PingResponse)]
        #[case::StreamGetBalanceSubscribe_1(StreamGetBalanceSubscribe)]
        #[case::StreamGetBalanceUnsubscribe_1(StreamGetBalanceUnsubscribe)]
        #[case::StreamGetKeepAliveSubscribe_1(StreamGetKeepAliveSubscribe)]
        #[case::StreamGetKeepAliveUnsubscribe_1(StreamGetKeepAliveUnsubscribe)]
        #[case::StreamGetNewsSubscribe_1(StreamGetNewsSubscribe)]
        #[case::StreamGetNewsUnsubscribe_1(StreamGetNewsUnsubscribe)]
        #[case::StreamGetProfitSubscribe_1(StreamGetProfitSubscribe)]
        #[case::StreamGetProfitUnsubscribe_1(StreamGetProfitUnsubscribe)]
        #[case::StreamGetTradesSubscribe_1(StreamGetTradesSubscribe)]
        #[case::StreamGetTradesUnsubscribe_1(StreamGetTradesUnsubscribe)]
        #[case::StreamGetTradeStatusSubscribe_1(StreamGetTradeStatusSubscribe)]
        #[case::StreamGetTradeStatusUnsubscribe_1(StreamGetTradeStatusUnsubscribe)]
        #[case::StreamPingSubscribe_1(StreamPingSubscribe)]
        fn serialize_deserialize_logic_struct<T: Serialize + Clone + Default + Debug + PartialEq + for<'de> Deserialize<'de>>(#[case] original: T) {
            let serialized_value = to_value(original.clone()).unwrap();
            assert!(serialized_value.is_null());
            let new_value = from_value(serialized_value).unwrap();
            assert_eq!(original, new_value);
        }

        #[rstest]
        #[case::GetAllSymbolsResponse_1(GetAllSymbolsResponse::default())]
        #[case::GetCalendarResponse_1(GetCalendarResponse::default())]
        #[case::GetIbsHistoryResponse_1(GetIbsHistoryResponse::default())]
        #[case::GetNewsResponse_1(GetNewsResponse::default())]
        #[case::GetStepRulesResponse_1(GetStepRulesResponse::default())]
        #[case::GetTradeRecordsResponse_1(GetTradeRecordsResponse::default())]
        #[case::GetTradingHoursResponse_1(GetTradingHoursResponse::default())]
        fn deserialize_empty_array_of_records<T: Serialize + for<'de> Deserialize<'de> + PartialEq + Debug>(#[case] reference: T) {
            let response: T = from_str("[]").unwrap();
            let serialized_reference = to_string(&reference).unwrap();
            assert_eq!(response, reference);
            assert_eq!(&serialized_reference, "[]");
        }

        #[rstest]
        #[case::GetAllSymbolsResponse_1(GetAllSymbolsResponse::default())]
        #[case::GetCalendarResponse_1(GetCalendarResponse::default())]
        #[case::GetIbsHistoryResponse_1(GetIbsHistoryResponse::default())]
        #[case::GetNewsResponse_1(GetNewsResponse::default())]
        #[case::GetStepRulesResponse_1(GetStepRulesResponse::default())]
        #[case::GetTradeRecordsResponse_1(GetTradeRecordsResponse::default())]
        #[case::GetTradingHoursResponse_1(GetTradingHoursResponse::default())]
        fn deserialize_array_of_records_with_one_record<T, I>(#[case] inp: T)
        where
            T: Serialize + for<'de> Deserialize<'de> + PartialEq + Debug + Deref<Target = Vec<I>>,
            I: Default + PartialEq + Debug + Serialize + for<'de> Deserialize<'de>
        {
            let ref_val = SymbolRecord::default();
            let source_data = format!("[{}]", to_string(&ref_val).unwrap());
            let response: GetAllSymbolsResponse = from_str(&source_data).unwrap();

            assert_eq!(response.len(), 1);
            assert_eq!(response[0], ref_val);
        }
    }
}
