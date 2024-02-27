use std::marker::PhantomData;
use rstest::rstest;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, to_value, Value};

use crate::api::data::{StreamGetTradeStatusData, StreamGetTradesData, StreamGetTickPricesData, StreamGetTickPricesSubscribe, StreamGetTickPricesUnsubscribe, StreamGetProfitData, StreamGetNewsData, StreamGetKeepAliveData, StreamGetCandlesSubscribe, StreamGetCandlesUnsubscribe, StreamGetCandlesData, StreamGetBalanceData, TradeTransactionStatusResponse, TradeTransactionStatusRequest, TradeTransactionResponse, TradeTransactionRequest, TradeTransInfo, GetVersionResponse, TradingHoursRecord, HoursRecord, GetTradingHoursRequest, GetTradesHistoryRequest, GetTradesHistoryResponse, GetTradesRequest, GetTradesResponse, GetTradeRecordsResponse, TradeRecord, GetTradeRecordsRequest, GetTickPricesResponse, TickRecord, GetTickPricesRequest, GetSymbolResponse, GetStepRulesResponse, StepRuleRecord, StepRecord, GetServerTimeResponse, GetProfitCalculationResponse, GetProfitCalculationRequest, GetNewsResponse, NewsBodyRecord, GetNewsRequest, GetMarginTradeResponse, GetMarginTradeRequest, GetMarginLevelResponse, IBRecord, GetIbsHistoryResponse, GetIbsHistoryRequest, GetCurrentUserDataResponse, GetCommissionDefResponse, GetCommissionDefRequest, GetChartRangeRequestResponse, GetChartRangeRequestRequest, ChartRangeInfoRecord, GetChartLastRequestResponse, RateInfoRecord, ChartLastInfoRecord, GetChartLastRequestRequest, LoginRequest, SymbolRecord, GetAllSymbolsResponse, GetCalendarResponse, CalendarRecord};

#[rstest]
#[case::LoginRequest_Full(TEST_LOGIN_REQUEST_FULL, Converter::<LoginRequest>::default())]
#[case::LoginRequest_Mandatory(TEST_LOGIN_REQUEST_MANDATORY, Converter::<LoginRequest>::default())]
#[case::SymbolRecord(TEST_SYMBOL_RECORD, Converter::<SymbolRecord>::default())]
#[case::GetAllSymbolsResponse(TEST_GET_ALL_SYMBOLS_RESPONSE, Converter::<GetAllSymbolsResponse>::default())]
#[case::CalendarRecord(TEST_CALENDAR_RECORD, Converter::<CalendarRecord>::default())]
#[case::GetCalendarResponse(TEST_GET_CALENDAR_RESPONSE, Converter::<GetCalendarResponse>::default())]
#[case::ChartLastInfoRecord(TEST_CHART_LAST_INFO_RECORD, Converter::<ChartLastInfoRecord>::default())]
#[case::GetChartLastRequestRequest(TEST_GET_CHART_LAST_REQUEST_REQUEST, Converter::<GetChartLastRequestRequest>::default())]
#[case::RateInfoRecord(TEST_RATE_INFO_RECORD, Converter::<RateInfoRecord>::default())]
#[case::GetChartLastRequestResponse(TEST_GET_CHART_LAST_REQUEST_RESPONSE, Converter::<GetChartLastRequestResponse>::default())]
#[case::ChartRangeInfoRecord(TEST_CHART_RANGE_INFO_RECORD, Converter::<ChartRangeInfoRecord>::default())]
#[case::GetChartRangeRequestRequest(TEST_GET_CHART_RANGE_REQUEST, Converter::<GetChartRangeRequestRequest>::default())]
// same as the `GetChartLastRequestResponse`
#[case::GetChartRangeRequestResponse(TEST_GET_CHART_LAST_REQUEST_RESPONSE, Converter::<GetChartRangeRequestResponse>::default())]
#[case::GetCommissionDefRequest(TEST_GET_COMMISSION_DEF_REQUEST, Converter::<GetCommissionDefRequest>::default())]
#[case::GetCommissionDefResponse(TEST_GET_COMMISSION_DEF_RESPONSE, Converter::<GetCommissionDefResponse>::default())]
#[case::GetCurrentUserDataResponse(TEST_GET_CURRENT_USER_DATA_RESPONSE, Converter::<GetCurrentUserDataResponse>::default())]
#[case::GetIbsHistoryRequest(TEST_GET_IBS_HISTORY_REQUEST, Converter::<GetIbsHistoryRequest>::default())]
#[case::IBRecord(TEST_IB_RECORD, Converter::<IBRecord>::default())]
#[case::GetIbsHistoryResponse(TEST_GET_IBS_HISTORY_RESPONSE, Converter::<GetIbsHistoryResponse>::default())]
#[case::GetMarginLevelResponse(TEST_GET_MARGIN_LEVEL_RESPONSE, Converter::<GetMarginLevelResponse>::default())]
#[case::GetMarginTradeRequest(TEST_GET_MARGIN_TRADE_REQUEST, Converter::<GetMarginTradeRequest>::default())]
#[case::GetMarginTradeResponse(TEST_GET_MARGIN_TRADE_RESPONE, Converter::<GetMarginTradeResponse>::default())]
#[case::GetNewsRequest(TEST_GET_NEWS_REQUEST, Converter::<GetNewsRequest>::default())]
#[case::NewsBodyRecord(TEST_NEWS_TOPIC_RECORD, Converter::<NewsBodyRecord>::default())]
#[case::GetNewsResponse(TEST_GET_NEWS_RESPONSE, Converter::<GetNewsResponse>::default())]
#[case::GetProfitCalculationRequest(TEST_GET_PROFIT_CALCULATION_REQUEST, Converter::<GetProfitCalculationRequest>::default())]
#[case::GetProfitCalculationResponse(TEST_GET_PROFIT_CALCULATION_RESPONSE, Converter::<GetProfitCalculationResponse>::default())]
#[case::GetServerTimeResponse(TEST_GET_SERVER_TIME_RESPONSE, Converter::<GetServerTimeResponse>::default())]
#[case::StepRecord(TEST_STEP_RECORD, Converter::<StepRecord>::default())]
#[case::StepRuleRecord(TEST_STEP_RULE_RECORD, Converter::<StepRuleRecord>::default())]
#[case::GetStepRulesResponse(TEST_GET_STEP_RULES_RESPONSE, Converter::<GetStepRulesResponse>::default())]
// same as SymbolRecord
#[case::GetSymbolResponse(TEST_SYMBOL_RECORD, Converter::<GetSymbolResponse>::default())]
#[case::GetTickPricesRequest(TEST_GET_TICK_PRICES_REQUEST, Converter::<GetTickPricesRequest>::default())]
#[case::TickRecord(TEST_TICK_RECORD, Converter::<TickRecord>::default())]
#[case::GetTickPricesResponse(TEST_GET_TICK_PRICES_RESPONSE, Converter::<GetTickPricesResponse>::default())]
#[case::GetTradeRecordsRequest(TEST_GET_TRADE_RECORDS_REQUEST, Converter::<GetTradeRecordsRequest>::default())]
#[case::TradeRecord(TEST_TRADE_RECORD, Converter::<TradeRecord>::default())]
#[case::GetTradeRecordsResponse(TEST_GET_TRADE_RECORDS_RESPONSE, Converter::<GetTradeRecordsResponse>::default())]
#[case::GetTradesRequest(TEST_GET_TRADES_REQUEST, Converter::<GetTradesRequest>::default())]
// same as GetTradeRecordsResponse
#[case::GetTradesResponse(TEST_GET_TRADE_RECORDS_RESPONSE, Converter::<GetTradesResponse>::default())]
#[case::GetTradesHistoryRequest(TEST_GET_TRADES_HISTORY_REQUEST, Converter::<GetTradesHistoryRequest>::default())]
// same as GetTradeRecordsResponse
#[case::GetTradesHistoryResponse(TEST_GET_TRADE_RECORDS_RESPONSE, Converter::<GetTradesHistoryResponse>::default())]
#[case::GetTradingHoursRequest(TEST_GET_TRADING_HOURS_REQUEST, Converter::<GetTradingHoursRequest>::default())]
#[case::HoursRecord(TEST_HOURS_RECORD, Converter::<HoursRecord>::default())]
#[case::TradingHoursRecord(TEST_TRADING_HOURS_RECORD, Converter::<TradingHoursRecord>::default())]
#[case::GetVersionResponse(TEST_GET_VERSION_RESPONSE, Converter::<GetVersionResponse>::default())]
#[case::TradeTransInfo(TEST_TRADE_TRANS_INFO, Converter::<TradeTransInfo>::default())]
#[case::TradeTransactionRequest(TEST_TRADE_TRANSACTION_REQUEST, Converter::<TradeTransactionRequest>::default())]
#[case::TradeTransactionResponse(TEST_TRADE_TRANSACTION_RESPONSE, Converter::<TradeTransactionResponse>::default())]
#[case::TradeTransactionStatusRequest(TEST_TRADE_TRANSACTION_STATUS_REQUEST, Converter::<TradeTransactionStatusRequest>::default())]
#[case::TradeTransactionStatusResponse(TEST_TRADE_TRANSACTION_STATUS_RESPONSE, Converter::<TradeTransactionStatusResponse>::default())]
#[case::StreamGetBalanceData(TEST_STREAMING_GET_BALANCE_DATA, Converter::<StreamGetBalanceData>::default())]
#[case::StreamGetCandlesSubscribe(TEST_STREAM_GET_CANDLES_SUBSCRIBE, Converter::<StreamGetCandlesSubscribe>::default())]
#[case::StreamGetCandlesUnsubscribe(TEST_STREAM_GET_CANDLES_UNSUBSCRIBE, Converter::<StreamGetCandlesUnsubscribe>::default())]
#[case::StreamGetCandlesData(TEST_STREAM_GET_CANDLES_DATA, Converter::<StreamGetCandlesData>::default())]
#[case::StreamGetKeepAliveData(TEST_STREAM_GET_KEEP_ALIVE_DATA, Converter::<StreamGetKeepAliveData>::default())]
#[case::StreamGetNewsData(TEST_STREAM_GET_NEWS_DATA, Converter::<StreamGetNewsData>::default())]
#[case::StreamGetProfitData(TEST_STREAM_GET_PROFITS_DATA, Converter::<StreamGetProfitData>::default())]
#[case::StreamGetTickPricesSubscribe(TEST_STREAM_GET_TICK_PRICES_SUBSCRIBE, Converter::<StreamGetTickPricesSubscribe>::default())]
#[case::StreamGetTickPricesUnsubscribe(TEST_STREAM_GET_TICK_PRICES_UNSUBSCRIBE, Converter::<StreamGetTickPricesUnsubscribe>::default())]
#[case::StreamGetTickPricesData(TEST_STREAM_GET_TICK_PRICES_DATA, Converter::<StreamGetTickPricesData>::default())]
#[case::StreamGetTradesData(TEST_STREAM_GET_TRADES_DATA, Converter::<StreamGetTradesData>::default())]
#[case::StreamGetTradeStatusData(TEST_STREAM_GET_TRADE_STATUS_DATA, Converter::<StreamGetTradeStatusData>::default())]
fn test_payload_serde<T: Default + Serialize + for<'de> Deserialize<'de>>(#[case] input: &str, #[case] converter: Converter<T>) {
    let reference_value: Value = from_str(input).unwrap();
    let converted = converter.convert(reference_value.clone());

    assert_eq!(converted, reference_value);
}

#[derive(Default)]
struct Converter<T: Default + Serialize + for<'de> Deserialize<'de>> {
    _t: PhantomData<T>
}

impl<T: Default + Serialize + for<'de> Deserialize<'de>> Converter<T> {
    fn convert(&self, value: Value) -> Value {
        let obj: T = from_value(value).unwrap();
        to_value(obj).unwrap()
    }
}


const TEST_LOGIN_REQUEST_FULL: &'static str = r#"
{
    "userId": "1000",
    "password": "PASSWORD",
    "appId": "test",
    "appName": "test"
}
"#;

const TEST_LOGIN_REQUEST_MANDATORY: &'static str = r#"
{
    "userId": "1000",
    "password": "PASSWORD"
}
"#;


// Note - added pipsPrecision field to payload from official documentation
// This field was missing there and I think it is typo.
// If it causes errors when this lib will be used to real server communication, fix must be done
const TEST_SYMBOL_RECORD: &'static str = r#"
{
	"ask": 4000.0,
	"bid": 4000.0,
	"categoryName": "Forex",
	"contractSize": 100000,
	"currency": "USD",
	"currencyPair": true,
	"currencyProfit": "SEK",
	"description": "USD/PLN",
	"expiration": null,
	"groupName": "Minor",
	"high": 4000.0,
	"initialMargin": 0,
	"instantMaxVolume": 0,
	"leverage": 1.5,
	"longOnly": false,
	"lotMax": 10.0,
	"lotMin": 0.1,
	"lotStep": 0.1,
	"low": 3500.0,
	"marginHedged": 0,
	"marginHedgedStrong": false,
	"marginMaintenance": null,
	"marginMode": 101,
	"percentage": 100.0,
	"pipsPrecision": 2,
	"precision": 2,
	"profitMode": 5,
	"quoteId": 1,
	"shortSelling": true,
	"spreadRaw": 0.000003,
	"spreadTable": 0.00042,
	"starting": null,
	"stepRuleId": 1,
	"stopsLevel": 0,
	"swap_rollover3days": 0,
	"swapEnable": true,
	"swapLong": -2.55929,
	"swapShort": 0.131,
	"swapType": 0,
	"symbol": "USDPLN",
	"tickSize": 1.0,
	"tickValue": 1.0,
	"time": 1272446136891,
	"timeString": "Thu May 23 12:23:44 EDT 2013",
	"trailingEnabled": true,
	"type": 21
}
"#;


// Note - added pipsPrecision field to payload from official documentation
// This field was missing there and I think it is typo.
// If it causes errors when this lib will be used to real server communication, fix must be done
const TEST_GET_ALL_SYMBOLS_RESPONSE: &'static str = r#"
[{
	"ask": 4000.0,
	"bid": 4000.0,
	"categoryName": "Forex",
	"contractSize": 100000,
	"currency": "USD",
	"currencyPair": true,
	"currencyProfit": "SEK",
	"description": "USD/PLN",
	"expiration": null,
	"groupName": "Minor",
	"high": 4000.0,
	"initialMargin": 0,
	"instantMaxVolume": 0,
	"leverage": 1.5,
	"longOnly": false,
	"lotMax": 10.0,
	"lotMin": 0.1,
	"lotStep": 0.1,
	"low": 3500.0,
	"marginHedged": 0,
	"marginHedgedStrong": false,
	"marginMaintenance": null,
	"marginMode": 101,
	"percentage": 100.0,
	"pipsPrecision": 2,
	"precision": 2,
	"profitMode": 5,
	"quoteId": 1,
	"shortSelling": true,
	"spreadRaw": 0.000003,
	"spreadTable": 0.00042,
	"starting": null,
	"stepRuleId": 1,
	"stopsLevel": 0,
	"swap_rollover3days": 0,
	"swapEnable": true,
	"swapLong": -2.55929,
	"swapShort": 0.131,
	"swapType": 0,
	"symbol": "USDPLN",
	"tickSize": 1.0,
	"tickValue": 1.0,
	"time": 1272446136891,
	"timeString": "Thu May 23 12:23:44 EDT 2013",
	"trailingEnabled": true,
	"type": 21
}]
"#;


const TEST_CALENDAR_RECORD: &'static str = r#"
{
	"country": "CA",
	"current": "",
	"forecast": "",
	"impact": "3",
	"period": "(FEB)",
	"previous": "58.3",
	"time": 1374846900000,
	"title": "Ivey Purchasing Managers Index"
}
"#;


const TEST_GET_CALENDAR_RESPONSE: &'static str = r#"
[{
	"country": "CA",
	"current": "",
	"forecast": "",
	"impact": "3",
	"period": "(FEB)",
	"previous": "58.3",
	"time": 1374846900000,
	"title": "Ivey Purchasing Managers Index"
}]
"#;


const TEST_CHART_LAST_INFO_RECORD: &'static str = r#"
{
	"period": 5,
	"start": 1262944112000,
	"symbol": "PKN.PL"
}
"#;


const TEST_GET_CHART_LAST_REQUEST_REQUEST: &'static str = r#"
{
    "info": {
        "period": 5,
        "start": 1262944112000,
        "symbol": "PKN.PL"
    }
}
"#;


const TEST_RATE_INFO_RECORD: &'static str = r#"
{
	"close": 1.0,
	"ctm": 1389362640000,
	"ctmString": "Jan 10, 2014 3:04:00 PM",
	"high": 6.0,
	"low": 0.0,
	"open": 41848.0,
	"vol": 0.0
}
"#;


const TEST_GET_CHART_LAST_REQUEST_RESPONSE: &'static str = r#"
{
    "digits": 4,
    "rateInfos": [{
        "close": 1.0,
        "ctm": 1389362640000,
        "ctmString": "Jan 10, 2014 3:04:00 PM",
        "high": 6.0,
        "low": 0.0,
        "open": 41848.0,
        "vol": 0.0
    }]
}
"#;


const TEST_CHART_RANGE_INFO_RECORD: &'static str = r#"
{
	"end": 1262944412000,
	"period": 5,
	"start": 1262944112000,
	"symbol": "PKN.PL",
	"ticks": 0
}
"#;


const TEST_GET_CHART_RANGE_REQUEST: &'static str = r#"
{
    "info": {
        "end": 1262944412000,
        "period": 5,
        "start": 1262944112000,
        "symbol": "PKN.PL",
        "ticks": 0
    }
}
"#;


const TEST_GET_COMMISSION_DEF_REQUEST: &'static str = r#"
{
    "symbol": "T.US",
    "volume": 1.0
}
"#;


const TEST_GET_COMMISSION_DEF_RESPONSE: &'static str = r#"
{
    "commission": 0.51,
    "rateOfExchange": 0.1609
}
"#;


const TEST_GET_CURRENT_USER_DATA_RESPONSE: &'static str = r#"
{
    "companyUnit": 8,
    "currency": "PLN",
    "group": "demoPLeurSTANDARD200",
    "ibAccount": false,
    "leverage": 1,
    "leverageMultiplier": 0.25,
    "spreadType": "FLOAT",
    "trailingStop": false
}
"#;


const TEST_GET_IBS_HISTORY_REQUEST: &'static str = r#"
{
    "end": 1395053810991,
    "start": 1394449010991
}
"#;


const TEST_IB_RECORD: &'static str = r#"
{
	"closePrice": 1.39302,
	"login": "12345",
	"nominal": 6.00,
	"openPrice": 1.39376,
	"side": 0,
	"surname": "IB_Client_1",
	"symbol": "EURUSD",
	"timestamp": 1395755870000,
	"volume": 1.0
}
"#;


const TEST_GET_IBS_HISTORY_RESPONSE: &'static str = r#"
[{
	"closePrice": 1.39302,
	"login": "12345",
	"nominal": 6.00,
	"openPrice": 1.39376,
	"side": 0,
	"surname": "IB_Client_1",
	"symbol": "EURUSD",
	"timestamp": 1395755870000,
	"volume": 1.0
}]
"#;


const TEST_GET_MARGIN_LEVEL_RESPONSE: &'static str = r#"
{
    "balance": 995800269.43,
    "credit": 1000.00,
    "currency": "PLN",
    "equity": 995985397.56,
    "margin": 572634.43,
    "margin_free": 995227635.00,
    "margin_level": 173930.41
}
"#;


const TEST_GET_MARGIN_TRADE_REQUEST: &'static str = r#"
{
    "symbol": "EURPLN",
    "volume": 1.0
}
"#;


const TEST_GET_MARGIN_TRADE_RESPONE: &'static str = r#"
{
    "margin": 4399.350
}
"#;


const TEST_GET_NEWS_REQUEST: &'static str = r#"
{
    "end": 0,
    "start": 1275993488000
}
"#;


const TEST_NEWS_TOPIC_RECORD: &'static str = r#"
{
	"body": "<html>...</html>",
	"bodylen": 110,
	"key": "1f6da766abd29927aa854823f0105c23",
	"time": 1262944112000,
	"timeString": "May 17, 2013 4:30:00 PM",
	"title": "Breaking trend"
}
"#;


const TEST_GET_NEWS_RESPONSE: &'static str = r#"
[{
	"body": "<html>...</html>",
	"bodylen": 110,
	"key": "1f6da766abd29927aa854823f0105c23",
	"time": 1262944112000,
	"timeString": "May 17, 2013 4:30:00 PM",
	"title": "Breaking trend"
}]
"#;


const TEST_GET_PROFIT_CALCULATION_REQUEST: &'static str = r#"
{
    "closePrice": 1.3000,
    "cmd": 0,
    "openPrice": 1.2233,
    "symbol": "EURPLN",
    "volume": 1.0
}
"#;


const TEST_GET_PROFIT_CALCULATION_RESPONSE: &'static str = r#"
{
    "profit": 714.303
}
"#;


const TEST_GET_SERVER_TIME_RESPONSE: &'static str = r#"
{
    "time": 1392211379731,
    "timeString": "Feb 12, 2014 2:22:59 PM"
}
"#;


const TEST_STEP_RECORD: &'static str = r#"
{
	"fromValue": 0.1,
	"step": 0.0025
}
"#;


const TEST_STEP_RULE_RECORD: &'static str = r#"
{
	"id": 1,
	"name": "Forex",
	"steps": [{
        "fromValue": 0.1,
        "step": 0.0025
    }]
}
"#;


const TEST_GET_STEP_RULES_RESPONSE: &'static str = r#"
[{
	"id": 1,
	"name": "Forex",
	"steps": [{
        "fromValue": 0.1,
        "step": 0.0025
    }]
}]
"#;


const TEST_GET_TICK_PRICES_REQUEST: &'static str = r#"
{
    "level": 0,
    "symbols": ["EURPLN", "AGO.PL"],
    "timestamp": 1262944112000
}
"#;


const TEST_TICK_RECORD: &'static str = r#"
{
	"ask": 4000.0,
	"askVolume": 15000,
	"bid": 4000.0,
	"bidVolume": 16000,
	"high": 4000.0,
	"level": 0,
	"low": 3500.0,
	"spreadRaw": 0.000003,
	"spreadTable": 0.00042,
	"symbol": "KOMB.CZ",
	"timestamp": 1272529161605
}
"#;


const TEST_GET_TICK_PRICES_RESPONSE: &'static str = r#"
{
    "quotations": [{
        "ask": 4000.0,
        "askVolume": 15000,
        "bid": 4000.0,
        "bidVolume": 16000,
        "high": 4000.0,
        "level": 0,
        "low": 3500.0,
        "spreadRaw": 0.000003,
        "spreadTable": 0.00042,
        "symbol": "KOMB.CZ",
        "timestamp": 1272529161605
    }]
}
"#;


const TEST_GET_TRADE_RECORDS_REQUEST: &'static str = r#"
{
    "orders": [7489839, 7489841]
}
"#;


const TEST_TRADE_RECORD: &'static str = r#"
{
	"close_price": 1.3256,
	"close_time": null,
	"close_timeString": null,
	"closed": false,
	"cmd": 0,
	"comment": "Web Trader",
	"commission": 0.0,
	"customComment": "Some text",
	"digits": 4,
	"expiration": null,
	"expirationString": null,
	"margin_rate": 0.0,
	"offset": 0,
	"open_price": 1.4,
	"open_time": 1272380927000,
	"open_timeString": "Fri Jan 11 10:03:36 CET 2013",
	"order": 7497776,
	"order2": 1234567,
	"position": 1234567,
	"profit": -2196.44,
	"sl": 0.0,
	"storage": -4.46,
	"symbol": "EURUSD",
	"timestamp": 1272540251000,
	"tp": 0.0,
	"volume": 0.10
}
"#;


const TEST_GET_TRADE_RECORDS_RESPONSE: &'static str = r#"
[{
	"close_price": 1.3256,
	"close_time": null,
	"close_timeString": null,
	"closed": false,
	"cmd": 0,
	"comment": "Web Trader",
	"commission": 0.0,
	"customComment": "Some text",
	"digits": 4,
	"expiration": null,
	"expirationString": null,
	"margin_rate": 0.0,
	"offset": 0,
	"open_price": 1.4,
	"open_time": 1272380927000,
	"open_timeString": "Fri Jan 11 10:03:36 CET 2013",
	"order": 7497776,
	"order2": 1234567,
	"position": 1234567,
	"profit": -2196.44,
	"sl": 0.0,
	"storage": -4.46,
	"symbol": "EURUSD",
	"timestamp": 1272540251000,
	"tp": 0.0,
	"volume": 0.10
}]
"#;


const TEST_GET_TRADES_REQUEST: &'static str = r#"
{
    "openedOnly": true
}
"#;


const TEST_GET_TRADES_HISTORY_REQUEST: &'static str = r#"
{
    "end": 0,
    "start": 1275993488000
}
"#;


const TEST_GET_TRADING_HOURS_REQUEST: &'static str = r#"
{
    "symbols": ["EURPLN", "AGO.PL"]
}
"#;


const TEST_HOURS_RECORD: &'static str = r#"
{
	"day": 2,
	"fromT": 63000000,
	"toT": 63300000
}
"#;


const TEST_TRADING_HOURS_RECORD: &'static str = r#"
{
	"quotes": [{
        "day": 2,
        "fromT": 63000000,
        "toT": 63300000
    }],
	"symbol": "USDPLN",
	"trading": [{
        "day": 2,
        "fromT": 63000000,
        "toT": 63300000
    }]
}
"#;


const TEST_GET_TRADING_HOURS_RESPONSE: &'static str = r#"
[{
	"quotes": [{
        "day": 2,
        "fromT": 63000000,
        "toT": 63300000
    }],
	"symbol": "USDPLN",
	"trading": [{
        "day": 2,
        "fromT": 63000000,
        "toT": 63300000
    }]
}]
"#;


const TEST_GET_VERSION_RESPONSE: &'static str = r#"
{
    "version": "2.4.15"
}
"#;


const TEST_TRADE_TRANS_INFO: &'static str = r#"
{
	"cmd": 2,
	"customComment": "Some text",
	"expiration": 1462006335000,
	"offset": 0,
	"order": 82188055,
	"price": 1.12,
	"sl": 0.0,
	"symbol": "EURUSD",
	"tp": 0.0,
	"type": 0,
	"volume": 5.0
}
"#;


const TEST_TRADE_TRANSACTION_REQUEST: &'static str = r#"
{
    "tradeTransInfo": {
        "cmd": 2,
        "customComment": "Some text",
        "expiration": 1462006335000,
        "offset": 0,
        "order": 82188055,
        "price": 1.12,
        "sl": 0.0,
        "symbol": "EURUSD",
        "tp": 0.0,
        "type": 0,
        "volume": 5.0
    }
}
"#;


const TEST_TRADE_TRANSACTION_RESPONSE: &'static str = r#"
{
    "order": 43
}
"#;


const TEST_TRADE_TRANSACTION_STATUS_REQUEST: &'static str = r#"
{
    "order": 43
}
"#;


const TEST_TRADE_TRANSACTION_STATUS_RESPONSE: &'static str = r#"
{
    "ask": 1.392,
    "bid": 1.392,
    "customComment": "Some text",
    "message": null,
    "order": 43,
    "requestStatus": 3
}
"#;


const TEST_STREAMING_GET_BALANCE_DATA: &'static str = r#"
{
	"balance": 995800269.43,
	"credit": 1000.00,
	"equity": 995985397.56,
	"margin": 572634.43,
	"marginFree": 995227635.00,
	"marginLevel": 173930.41
}
"#;


const TEST_STREAM_GET_CANDLES_SUBSCRIBE: &'static str = r#"
{
	"symbol": "EURUSD"
}
"#;


const TEST_STREAM_GET_CANDLES_UNSUBSCRIBE: &'static str = r#"
{
	"symbol": "EURUSD"
}
"#;


const TEST_STREAM_GET_CANDLES_DATA: &'static str = r#"
{
	"close": 4.1849,
	"ctm": 1378369375000,
	"ctmString": "Sep 05, 2013 10:22:55 AM",
	"high": 4.1854,
	"low": 4.1848,
	"open": 4.1848,
	"quoteId": 2,
	"symbol": "EURUSD",
	"vol": 0.0
}
"#;


const TEST_STREAM_GET_KEEP_ALIVE_DATA: &'static str = r#"
{
	"timestamp": 1362944112000
}
"#;


const TEST_STREAM_GET_NEWS_DATA: &'static str = r#"
{
	"body": "<html>...</html>",
	"key": "1f6da766abd29927aa854823f0105c23",
	"time": 1262944112000,
	"title": "Breaking trend"
}
"#;


const TEST_STREAM_GET_PROFITS_DATA: &'static str = r#"
{
	"order": 7497776,
	"order2": 7497777,
	"position": 7497776,
	"profit": 7076.52
}
"#;


const TEST_STREAM_GET_TICK_PRICES_SUBSCRIBE: &'static str = r#"
{
	"symbol": "EURUSD",
	"minArrivalTime": 5000,
	"maxLevel": 2
}
"#;


const TEST_STREAM_GET_TICK_PRICES_UNSUBSCRIBE: &'static str = r#"
{
	"symbol": "EURUSD"
}
"#;


const TEST_STREAM_GET_TICK_PRICES_DATA: &'static str = r#"
{
	"ask": 4000.0,
	"askVolume": 15000,
	"bid": 4000.0,
	"bidVolume": 16000,
	"high": 4000.0,
	"level": 0,
	"low": 3500.0,
	"quoteId": 1,
	"spreadRaw": 0.000003,
	"spreadTable": 0.00042,
	"symbol": "KOMB.CZ",
	"timestamp": 1272529161605
}
"#;


const TEST_STREAM_GET_TRADES_DATA: &'static str = r#"
{
	"close_price": 1.3256,
	"close_time": null,
	"closed": false,
	"cmd": 0,
	"comment": "Web Trader",
	"commission": 0.0,
	"customComment": "Some text",
	"digits": 4,
	"expiration": null,
	"margin_rate": 3.9149000,
	"offset": 0,
	"open_price": 1.4,
	"open_time": 1272380927000,
	"order": 7497776,
	"order2": 1234567,
	"position": 1234567,
	"profit": 68.392,
	"sl": 0.0,
	"state": "Modified",
	"storage": -4.46,
	"symbol": "EURUSD",
	"tp": 0.0,
	"type": 0,
	"volume": 0.10
}
"#;


const TEST_STREAM_GET_TRADE_STATUS_DATA: &'static str = r#"
{
	"customComment": "Some text",
	"message": null,
	"order": 43,
	"price": 1.392,
	"requestStatus": 3
}
"#;
