use std::marker::PhantomData;
use rstest::rstest;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, to_value, Value};

use crate::api::data::{LoginRequest, SymbolRecord};

#[rstest]
#[case::LoginRequest_Full(TEST_LOGIN_REQUEST_FULL, Converter::<LoginRequest>::default())]
#[case::LoginRequest_Mandatory(TEST_LOGIN_REQUEST_MANDATORY, Converter::<LoginRequest>::default())]
#[case::SymbolRecord(TEST_SYMBOL_RECORD, Converter::<SymbolRecord>::default())]
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
