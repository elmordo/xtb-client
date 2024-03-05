use std::fmt;
use std::str::FromStr;

use serde_with::DeserializeFromStr;
use thiserror::Error;

/// Rust enum definition for error codes
#[derive(Debug, Clone, PartialEq, DeserializeFromStr, Default)]
pub enum XtbErrorCode {
    /// Invalid price
    #[default]
    BE001,
    /// Invalid StopLoss or TakeProfit
    BE002,
    /// Invalid volume
    BE003,
    /// Login disabled
    BE004,
    /// userPasswordCheck: Invalid login or password.
    BE005,
    /// Market for instrument is closed
    BE006,
    /// Mismatched parameters
    BE007,
    /// Modification is denied
    BE008,
    /// Not enough money on account to perform trade
    BE009,
    /// Off quotes
    BE010,
    /// Opposite positions prohibited
    BE011,
    /// Short positions prohibited
    BE012,
    /// Price has changed
    BE013,
    /// Request too frequent
    BE014,
    /// Too many trade requests
    BE016,
    /// Too many trade requests
    BE017,
    /// Trading on instrument disabled
    BE018,
    /// Trading timeout
    BE019,
    /// Symbol does not exist for given account
    BE094,
    /// Account cannot trade on given symbol
    BE095,
    /// Pending order cannot be closed. Pending order must be deleted
    BE096,
    /// Cannot close already closed order
    BE097,
    /// No such transaction
    BE098,
    /// Unknown instrument symbol
    BE101,
    /// Unknown transaction type
    BE102,
    /// User is not logged
    BE103,
    /// Method does not exist
    BE104,
    /// Incorrect period given
    BE105,
    /// Missing data
    BE106,
    /// Incorrect command format
    BE110,
    /// Symbol does not exist
    BE115,
    /// Symbol does not exist
    BE116,
    /// Invalid token
    BE117,
    /// User already logged
    BE118,
    /// Session timed out.
    BE200,
    /// Invalid parameters
    EX000,
    /// Internal error, in case of such error, please contact support
    EX001,
    /// Internal error, in case of such error, please contact support
    EX002,
    /// Internal error, in case of such error, please contact support
    BE000,
    /// Internal error, request timed out
    EX003,
    /// Login credentials are incorrect or this login is not allowed to use an application with this appId
    EX004,
    /// Internal error, system overloaded
    EX005,
    /// No access
    EX006,
    /// userPasswordCheck: Invalid login or password. This login/password is disabled for 10 minutes (the specific login and password pair is blocked after an unsuccessful login attempt).
    EX007,
    /// You have reached the connection limit. For details see the Connection validation section.
    EX008,
    /// Data limit potentially exceeded. Please narrow your request range. The potential data size is calculated by: (end_time - start_time) / interval. The limit is 50 000 candles
    EX009,
    /// Your login is on the black list, perhaps due to previous misuse. For details please contact support.
    EX010,
    /// You are not allowed to execute this command. For details please contact support.
    EX011,
    /// BE20-37 and BE99 - Other error
    OtherError(u8),
    /// SExxx - Internal server error
    InternalServerError(u16)
}

impl FromStr for XtbErrorCode {
    type Err = XtbErrorCodeError;

    fn from_str(s: &str) -> Result<XtbErrorCode, Self::Err> {
        match s {
            "BE001" => Ok(XtbErrorCode::BE001),
            "BE002" => Ok(XtbErrorCode::BE002),
            "BE003" => Ok(XtbErrorCode::BE003),
            "BE004" => Ok(XtbErrorCode::BE004),
            "BE005" => Ok(XtbErrorCode::BE005),
            "BE006" => Ok(XtbErrorCode::BE006),
            "BE007" => Ok(XtbErrorCode::BE007),
            "BE008" => Ok(XtbErrorCode::BE008),
            "BE009" => Ok(XtbErrorCode::BE009),
            "BE010" => Ok(XtbErrorCode::BE010),
            "BE011" => Ok(XtbErrorCode::BE011),
            "BE012" => Ok(XtbErrorCode::BE012),
            "BE013" => Ok(XtbErrorCode::BE013),
            "BE014" => Ok(XtbErrorCode::BE014),
            "BE016" => Ok(XtbErrorCode::BE016),
            "BE017" => Ok(XtbErrorCode::BE017),
            "BE018" => Ok(XtbErrorCode::BE018),
            "BE019" => Ok(XtbErrorCode::BE019),
            "BE094" => Ok(XtbErrorCode::BE094),
            "BE095" => Ok(XtbErrorCode::BE095),
            "BE096" => Ok(XtbErrorCode::BE096),
            "BE097" => Ok(XtbErrorCode::BE097),
            "BE098" => Ok(XtbErrorCode::BE098),
            "BE101" => Ok(XtbErrorCode::BE101),
            "BE102" => Ok(XtbErrorCode::BE102),
            "BE103" => Ok(XtbErrorCode::BE103),
            "BE104" => Ok(XtbErrorCode::BE104),
            "BE105" => Ok(XtbErrorCode::BE105),
            "BE106" => Ok(XtbErrorCode::BE106),
            "BE110" => Ok(XtbErrorCode::BE110),
            "BE115" => Ok(XtbErrorCode::BE115),
            "BE116" => Ok(XtbErrorCode::BE116),
            "BE117" => Ok(XtbErrorCode::BE117),
            "BE118" => Ok(XtbErrorCode::BE118),
            "BE200" => Ok(XtbErrorCode::BE200),
            "EX000" => Ok(XtbErrorCode::EX000),
            "EX001" => Ok(XtbErrorCode::EX001),
            "EX002" => Ok(XtbErrorCode::EX002),
            "BE000" => Ok(XtbErrorCode::BE000),
            "EX003" => Ok(XtbErrorCode::EX003),
            "EX004" => Ok(XtbErrorCode::EX004),
            "EX005" => Ok(XtbErrorCode::EX005),
            "EX006" => Ok(XtbErrorCode::EX006),
            "EX007" => Ok(XtbErrorCode::EX007),
            "EX008" => Ok(XtbErrorCode::EX008),
            "EX009" => Ok(XtbErrorCode::EX009),
            "EX010" => Ok(XtbErrorCode::EX010),
            "EX011" => Ok(XtbErrorCode::EX011),
            v @ ("BE020" | "BE021" | "BE022" | "BE023" | "BE024" | "BE025" | "BE026" | "BE027" | "BE028" | "BE029" | "BE030" | "BE031" | "BE032" | "BE033" | "BE034" | "BE035" | "BE036" | "BE037" | "BE099") => parse_other_error(v),
            v if v.starts_with("SE") => parse_se_error(v),
            v => Err(XtbErrorCodeError::UnsupportedErrorCode(v.to_owned())),
        }
    }
}


#[derive(Debug, PartialEq, Error)]
pub enum XtbErrorCodeError {
    #[error("The error code '{0}' is not supported by the 'xtb_client' library")]
    UnsupportedErrorCode(String),
}



fn parse_other_error(err_str: &str) -> Result<XtbErrorCode, XtbErrorCodeError> {
    if !err_str.starts_with("BE0") || err_str.len() != 5 {
        Err(XtbErrorCodeError::UnsupportedErrorCode(err_str.to_owned()))
    } else {
        let digits = &err_str[3..];
        let c = u8::from_str(digits).map_err(|_| XtbErrorCodeError::UnsupportedErrorCode(err_str.to_owned()))?;
        Ok(XtbErrorCode::OtherError(c))
    }
}


fn parse_se_error(err_str: &str) -> Result<XtbErrorCode, XtbErrorCodeError> {
    if !err_str.starts_with("SE") || err_str.len() != 5 {
        Err(XtbErrorCodeError::UnsupportedErrorCode(err_str.to_owned()))
    } else {
        let digits = &err_str[2..];
        let c = u16::from_str(digits).map_err(|_| XtbErrorCodeError::UnsupportedErrorCode(err_str.to_owned()))?;
        Ok(XtbErrorCode::InternalServerError(c))
    }
}


impl fmt::Display for XtbErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XtbErrorCode::BE001 => f.write_str("BE001"),
            XtbErrorCode::BE002 => f.write_str("BE002"),
            XtbErrorCode::BE003 => f.write_str("BE003"),
            XtbErrorCode::BE004 => f.write_str("BE004"),
            XtbErrorCode::BE005 => f.write_str("BE005"),
            XtbErrorCode::BE006 => f.write_str("BE006"),
            XtbErrorCode::BE007 => f.write_str("BE007"),
            XtbErrorCode::BE008 => f.write_str("BE008"),
            XtbErrorCode::BE009 => f.write_str("BE009"),
            XtbErrorCode::BE010 => f.write_str("BE010"),
            XtbErrorCode::BE011 => f.write_str("BE011"),
            XtbErrorCode::BE012 => f.write_str("BE012"),
            XtbErrorCode::BE013 => f.write_str("BE013"),
            XtbErrorCode::BE014 => f.write_str("BE014"),
            XtbErrorCode::BE016 => f.write_str("BE016"),
            XtbErrorCode::BE017 => f.write_str("BE017"),
            XtbErrorCode::BE018 => f.write_str("BE018"),
            XtbErrorCode::BE019 => f.write_str("BE019"),
            XtbErrorCode::BE094 => f.write_str("BE094"),
            XtbErrorCode::BE095 => f.write_str("BE095"),
            XtbErrorCode::BE096 => f.write_str("BE096"),
            XtbErrorCode::BE097 => f.write_str("BE097"),
            XtbErrorCode::BE098 => f.write_str("BE098"),
            XtbErrorCode::BE101 => f.write_str("BE101"),
            XtbErrorCode::BE102 => f.write_str("BE102"),
            XtbErrorCode::BE103 => f.write_str("BE103"),
            XtbErrorCode::BE104 => f.write_str("BE104"),
            XtbErrorCode::BE105 => f.write_str("BE105"),
            XtbErrorCode::BE106 => f.write_str("BE106"),
            XtbErrorCode::BE110 => f.write_str("BE110"),
            XtbErrorCode::BE115 => f.write_str("BE115"),
            XtbErrorCode::BE116 => f.write_str("BE116"),
            XtbErrorCode::BE117 => f.write_str("BE117"),
            XtbErrorCode::BE118 => f.write_str("BE118"),
            XtbErrorCode::BE200 => f.write_str("BE200"),
            XtbErrorCode::EX000 => f.write_str("EX000"),
            XtbErrorCode::EX001 => f.write_str("EX001"),
            XtbErrorCode::EX002 => f.write_str("EX002"),
            XtbErrorCode::BE000 => f.write_str("BE000"),
            XtbErrorCode::EX003 => f.write_str("EX003"),
            XtbErrorCode::EX004 => f.write_str("EX004"),
            XtbErrorCode::EX005 => f.write_str("EX005"),
            XtbErrorCode::EX006 => f.write_str("EX006"),
            XtbErrorCode::EX007 => f.write_str("EX007"),
            XtbErrorCode::EX008 => f.write_str("EX008"),
            XtbErrorCode::EX009 => f.write_str("EX009"),
            XtbErrorCode::EX010 => f.write_str("EX010"),
            XtbErrorCode::EX011 => f.write_str("EX011"),
            XtbErrorCode::OtherError(c) => f.write_str(&format!("BE{:03}", c)),
            XtbErrorCode::InternalServerError(c) => f.write_str(&format!("SE{:03}", c)),
        }
    }
}


#[cfg(test)]
mod tests {
    mod other_error_parser {
        use rstest::rstest;

        use crate::api::api_errors::{parse_other_error, XtbErrorCode, XtbErrorCodeError};

        #[rstest]
        #[case("BE020", 20)]
        #[case("BE021", 21)]
        #[case("BE022", 22)]
        #[case("BE023", 23)]
        #[case("BE024", 24)]
        #[case("BE025", 25)]
        #[case("BE026", 26)]
        #[case("BE027", 27)]
        #[case("BE028", 28)]
        #[case("BE029", 29)]
        #[case("BE030", 30)]
        #[case("BE031", 31)]
        #[case("BE032", 32)]
        #[case("BE033", 33)]
        #[case("BE034", 34)]
        #[case("BE035", 35)]
        #[case("BE036", 36)]
        #[case("BE037", 37)]
        #[case("BE099", 99)]
        fn parse_valid_value(#[case] input: &str, #[case] expected_code: u8) {
            assert_eq!(parse_other_error(input), Ok(XtbErrorCode::OtherError(expected_code)));
        }

        #[rstest]
        #[case("BE100")]
        #[case("BE120")]
        #[case("BEA20")]
        #[case("BE030A")]
        #[case("BE0300")]
        fn parse_invalid_value(#[case] input: &str) {
            assert_eq!(parse_other_error(input), Err(XtbErrorCodeError::UnsupportedErrorCode(input.to_owned())));
        }
    }

    mod se_error_parser {
        use rstest::rstest;

        use crate::api::api_errors::{parse_se_error, XtbErrorCode, XtbErrorCodeError};

        #[rstest]
        #[case("SE000", 0)]
        #[case("SE099", 99)]
        #[case("SE100", 100)]
        #[case("SE555", 555)]
        #[case("SE999", 999)]
        fn parse_valid_value(#[case] input: &str, #[case] expected_code: u16) {
            assert_eq!(parse_se_error(input), Ok(XtbErrorCode::InternalServerError(expected_code)));
        }

        #[rstest]
        #[case("SEE000")]
        #[case("SEABC")]
        #[case("SE0000")]
        fn parse_invalid_value(#[case] input: &str) {
            assert_eq!(parse_se_error(input), Err(XtbErrorCodeError::UnsupportedErrorCode(input.to_owned())));
        }
    }

    mod xtb_error_operations {
        use std::str::FromStr;

        use rstest::rstest;
        use rstest_reuse::{self, *};
        use serde_json::from_str;

        use crate::api::api_errors::XtbErrorCode;

        #[template]
        #[rstest]
        #[case("BE001", XtbErrorCode::BE001)]
        #[case("BE002", XtbErrorCode::BE002)]
        #[case("BE003", XtbErrorCode::BE003)]
        #[case("BE004", XtbErrorCode::BE004)]
        #[case("BE005", XtbErrorCode::BE005)]
        #[case("BE006", XtbErrorCode::BE006)]
        #[case("BE007", XtbErrorCode::BE007)]
        #[case("BE008", XtbErrorCode::BE008)]
        #[case("BE009", XtbErrorCode::BE009)]
        #[case("BE010", XtbErrorCode::BE010)]
        #[case("BE011", XtbErrorCode::BE011)]
        #[case("BE012", XtbErrorCode::BE012)]
        #[case("BE013", XtbErrorCode::BE013)]
        #[case("BE014", XtbErrorCode::BE014)]
        #[case("BE016", XtbErrorCode::BE016)]
        #[case("BE017", XtbErrorCode::BE017)]
        #[case("BE018", XtbErrorCode::BE018)]
        #[case("BE019", XtbErrorCode::BE019)]
        #[case("BE094", XtbErrorCode::BE094)]
        #[case("BE095", XtbErrorCode::BE095)]
        #[case("BE096", XtbErrorCode::BE096)]
        #[case("BE097", XtbErrorCode::BE097)]
        #[case("BE098", XtbErrorCode::BE098)]
        #[case("BE101", XtbErrorCode::BE101)]
        #[case("BE102", XtbErrorCode::BE102)]
        #[case("BE103", XtbErrorCode::BE103)]
        #[case("BE104", XtbErrorCode::BE104)]
        #[case("BE105", XtbErrorCode::BE105)]
        #[case("BE106", XtbErrorCode::BE106)]
        #[case("BE110", XtbErrorCode::BE110)]
        #[case("BE115", XtbErrorCode::BE115)]
        #[case("BE116", XtbErrorCode::BE116)]
        #[case("BE117", XtbErrorCode::BE117)]
        #[case("BE118", XtbErrorCode::BE118)]
        #[case("BE200", XtbErrorCode::BE200)]
        #[case("EX000", XtbErrorCode::EX000)]
        #[case("EX001", XtbErrorCode::EX001)]
        #[case("EX002", XtbErrorCode::EX002)]
        #[case("BE000", XtbErrorCode::BE000)]
        #[case("EX003", XtbErrorCode::EX003)]
        #[case("EX004", XtbErrorCode::EX004)]
        #[case("EX005", XtbErrorCode::EX005)]
        #[case("EX006", XtbErrorCode::EX006)]
        #[case("EX007", XtbErrorCode::EX007)]
        #[case("EX008", XtbErrorCode::EX008)]
        #[case("EX009", XtbErrorCode::EX009)]
        #[case("EX010", XtbErrorCode::EX010)]
        #[case("EX011", XtbErrorCode::EX011)]

        #[case("BE020", XtbErrorCode::OtherError(20u8))]
        #[case("BE021", XtbErrorCode::OtherError(21u8))]
        #[case("BE022", XtbErrorCode::OtherError(22u8))]
        #[case("BE023", XtbErrorCode::OtherError(23u8))]
        #[case("BE024", XtbErrorCode::OtherError(24u8))]
        #[case("BE025", XtbErrorCode::OtherError(25u8))]
        #[case("BE026", XtbErrorCode::OtherError(26u8))]
        #[case("BE027", XtbErrorCode::OtherError(27u8))]
        #[case("BE028", XtbErrorCode::OtherError(28u8))]
        #[case("BE029", XtbErrorCode::OtherError(29u8))]
        #[case("BE030", XtbErrorCode::OtherError(30u8))]
        #[case("BE031", XtbErrorCode::OtherError(31u8))]
        #[case("BE032", XtbErrorCode::OtherError(32u8))]
        #[case("BE033", XtbErrorCode::OtherError(33u8))]
        #[case("BE034", XtbErrorCode::OtherError(34u8))]
        #[case("BE035", XtbErrorCode::OtherError(35u8))]
        #[case("BE036", XtbErrorCode::OtherError(36u8))]
        #[case("BE037", XtbErrorCode::OtherError(37u8))]
        #[case("BE099", XtbErrorCode::OtherError(99u8))]

        #[case("SE000", XtbErrorCode::InternalServerError(0u16))]
        #[case("SE099", XtbErrorCode::InternalServerError(99u16))]
        #[case("SE100", XtbErrorCode::InternalServerError(100u16))]
        #[case("SE555", XtbErrorCode::InternalServerError(555u16))]
        #[case("SE999", XtbErrorCode::InternalServerError(999u16))]
        fn variant_test_template(#[case] input: &str, #[case] variant: XtbErrorCode) {}

        #[apply(variant_test_template)]
        fn parse_from_str(#[case] input: &str, #[case] variant: XtbErrorCode) {
            assert_eq!(XtbErrorCode::from_str(input), Ok(variant))
        }

        #[apply(variant_test_template)]
        fn convert_to_str(#[case] input: &str, #[case] variant: XtbErrorCode) {
            assert_eq!(format!("{}", variant), input.to_owned())
        }

        #[apply(variant_test_template)]
        fn deserialize(#[case] input: &str, #[case] variant: XtbErrorCode) {
            let deserialized: XtbErrorCode = from_str(&format!("\"{}\"", input)).unwrap();
            assert_eq!(deserialized, variant)
        }
    }
}
