use std::fmt;
use std::fmt::Write;
use std::str::FromStr;
use thiserror::Error;

/// Rust enum definition for error codes
#[derive(Debug, PartialEq)]
pub enum XtbErrorCode {
    /// Invalid price
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
    // .. SExxx does not appear, so the previous comment covers it
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
    /// Other error - BE20-37 and BE99
    OtherError(u8),
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
            v @ "BE020" | "BE021" | "BE022" | "BE023" | "BE024" | "BE025" | "BE026" | "BE027" | "BE028" | "BE029" | "BE030" | "BE031" | "BE032" | "BE033" | "BE034" | "BE035" | "BE036" | "BE037" | "BE099" => parse_other_error(v),
            v @ _ => Err(XtbErrorCodeError::UnsupportedErrorCode(v.to_owned())),
        }
    }
}


#[derive(Debug, Error)]
pub enum XtbErrorCodeError {
    #[error("The error code '{0}' is not supported by the 'xtb_client' library")]
    UnsupportedErrorCode(String),
}



fn parse_other_error(xtb_err_code: &str) -> Result<XtbErrorCode, XtbErrorCodeError> {
    if !xtb_err_code.starts_with("BE0") || xtb_err_code.len() != 5 {
        Err(XtbErrorCodeError::UnsupportedErrorCode(xtb_err_code.to_owned()))
    } else {
        let digits = &xtb_err_code[3..];
        let c = u8::from_str(digits).map_err(|_| XtbErrorCodeError::UnsupportedErrorCode(xtb_err_code.to_owned()))?;
        Ok(XtbErrorCode::OtherError(c))
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
            XtbErrorCode::OtherError(c) => f.write_str(&format!("OtherError({})", c)),
        }
    }
}
