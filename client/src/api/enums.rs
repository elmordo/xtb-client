use std::fmt;
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Enum representing various types
#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum QuoteId {
    /// fixed
    #[default]
    Fixed = 1,
    /// float
    Float = 2,
    /// depth
    Depth = 3,
    /// cross
    Cross = 4,
}


/// Enum representing different margin modes
#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MarginMode {
    /// Forex
    #[default]
    Forex = 101,
    /// CFD leveraged
    CFDLeveraged = 102,
    /// CFD
    CFD = 103,
}


/// Enum representing different profit modes
#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ProfitMode {
    /// FOREX
    #[default]
    Forex = 5,
    /// CFD
    Cfd = 6,
}


/// Expected impact level of event in calendar
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ImpactLevel {
    /// low
    #[serde(rename = "1")]
    #[default]
    Low = 1,
    /// medium
    #[serde(rename = "2")]
    Medium = 2,
    /// high
    #[serde(rename = "3")]
    High = 3,
}


/// Enum representing different time periods
#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum TimePeriod {
    /// 1 minute
    PeriodM1 = 1,
    /// 5 minutes
    #[default]
    PeriodM5 = 5,
    /// 15 minutes
    PeriodM15 = 15,
    /// 30 minutes
    PeriodM30 = 30,
    /// 60 minutes (1 hour)
    PeriodH1 = 60,
    /// 240 minutes (4 hours)
    PeriodH4 = 240,
    /// 1440 minutes (1 day)
    PeriodD1 = 1440,
    /// 10080 minutes (1 week)
    PeriodW1 = 10080,
    /// 43200 minutes (30 days)
    PeriodMN1 = 43200,
}


/// Enum representing types of trading actions
#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TradingAction {
    /// Buy
    #[default]
    Buy = 0,
    /// Sell
    Sell = 1,
}


/// Enum representing different types of trading actions
#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TradingCommand {
    /// Buy
    #[default]
    Buy = 0,
    /// Sell
    Sell = 1,
    /// Buy limit
    BuyLimit = 2,
    /// Sell limit
    SellLimit = 3,
    /// Buy stop
    BuyStop = 4,
    /// Sell stop
    SellStop = 5,
    /// Read only. Used in getTradesHistory for manager's deposit/withdrawal operations (profit>0 for deposit, profit<0 for withdrawal).
    Balance = 6,
    /// Read only
    Credit = 7,
}

#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DayOfWeek {
    /// Monday
    #[default]
    Monday = 1,
    /// Tuesday
    Tuesday = 2,
    /// Wednesday
    Wednesday = 3,
    /// Thursday
    Thursday = 4,
    /// Friday
    Friday = 5,
    /// Saturday
    Saturday = 6,
    /// Sunday
    Sunday = 7,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TransactionStatus {
    /// Error
    #[default]
    Error = 0,
    /// Pending
    Pending = 1,
    /// The transaction has been executed successfully
    Accepted = 3,
    /// The transaction has been rejected
    Rejected = 4,
}


#[derive(Default, Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TransactionType {
    /// Order open, used for opening orders
    #[default]
    Open = 0,
    /// Order pending, only used in the streaming getTrades command
    Pending = 1,
    /// Order close
    Close = 2,
    /// Order modify, only used in the tradeTransaction command
    Modify = 3,
    /// Order delete, only used in the tradeTransaction command
    Delete = 4,
}

#[derive(Default, Clone, PartialEq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeStatus {
    /// Modified
    #[default]
    Modified,
    /// Deleted
    Deleted,
}


// Implementation of custom Deserializer to allow case insensitive deserialization
impl<'de> Deserialize<'de> for TradeStatus {
    fn deserialize<D>(deserializer: D) -> Result<TradeStatus, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct TradeStatusVisitor;

        impl<'de> serde::de::Visitor<'de> for TradeStatusVisitor {
            type Value = TradeStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing TradeStatus")
            }

            fn visit_str<E>(self, value: &str) -> Result<TradeStatus, E>
                where
                    E: serde::de::Error,
            {
                let lowercase_value = value.to_lowercase();
                match lowercase_value.as_str() {
                    "modified" => Ok(TradeStatus::Modified),
                    "deleted" => Ok(TradeStatus::Deleted),
                    _ => Err(serde::de::Error::unknown_variant(value, &["Modified", "Deleted"])),
                }
            }
        }

        deserializer.deserialize_str(TradeStatusVisitor)
    }
}


#[cfg(test)]
mod tests {
    mod serialize_deserialize {
        use std::fmt::Debug;
        use rstest::rstest;
        use serde::{Deserialize, Serialize};
        use crate::api::enums::{TradeStatus, TransactionStatus, TransactionType, DayOfWeek, TradingCommand, QuoteId, MarginMode, ProfitMode, ImpactLevel, TimePeriod, TradingAction};
        use serde_json::{from_value, to_value, Value};

        #[rstest]
        #[case::QuoteId_Fixed(QuoteId::Fixed, to_value(1).unwrap())]
        #[case::QuoteId_Float(QuoteId::Float, to_value(2).unwrap())]
        #[case::QuoteId_Depth(QuoteId::Depth, to_value(3).unwrap())]
        #[case::QuoteId_Cross(QuoteId::Cross, to_value(4).unwrap())]

        #[case::QuoteId_Forex(MarginMode::Forex, to_value(101).unwrap())]
        #[case::QuoteId_CfdLeveraged(MarginMode::CFDLeveraged, to_value(102).unwrap())]
        #[case::QuoteId_Cfd(MarginMode::CFD, to_value(103).unwrap())]

        #[case::QuoteId_Forex(ProfitMode::Forex, to_value(5).unwrap())]
        #[case::QuoteId_Cfd(ProfitMode::Cfd, to_value(6).unwrap())]

        #[case::ImpactLevel_Low(ImpactLevel::Low, to_value("1").unwrap())]
        #[case::ImpactLevel_Medium(ImpactLevel::Medium, to_value("2").unwrap())]
        #[case::ImpactLevel_High(ImpactLevel::High, to_value("3").unwrap())]

        #[case::TimePeriod_M1(TimePeriod::PeriodM1, to_value(1).unwrap())]
        #[case::TimePeriod_M5(TimePeriod::PeriodM5, to_value(5).unwrap())]
        #[case::TimePeriod_M15(TimePeriod::PeriodM15, to_value(15).unwrap())]
        #[case::TimePeriod_M30(TimePeriod::PeriodM30, to_value(30).unwrap())]
        #[case::TimePeriod_H1(TimePeriod::PeriodH1, to_value(60).unwrap())]
        #[case::TimePeriod_H4(TimePeriod::PeriodH4, to_value(240).unwrap())]
        #[case::TimePeriod_D1(TimePeriod::PeriodD1, to_value(1440).unwrap())]
        #[case::TimePeriod_W1(TimePeriod::PeriodW1, to_value(10080).unwrap())]
        #[case::TimePeriod_MN11(TimePeriod::PeriodMN1, to_value(43200).unwrap())]

        #[case::TradingAction_Buy(TradingAction::Buy, to_value(0).unwrap())]
        #[case::TradingAction_Sell(TradingAction::Sell, to_value(1).unwrap())]

        #[case::TradingCommand_Buy(TradingCommand::Buy, to_value(0).unwrap())]
        #[case::TradingCommand_Sell(TradingCommand::Sell, to_value(1).unwrap())]
        #[case::TradingCommand_BuyLimit(TradingCommand::BuyLimit, to_value(2).unwrap())]
        #[case::TradingCommand_SellLimit(TradingCommand::SellLimit, to_value(3).unwrap())]
        #[case::TradingCommand_BuyStop(TradingCommand::BuyStop, to_value(4).unwrap())]
        #[case::TradingCommand_SellStop(TradingCommand::SellStop, to_value(5).unwrap())]
        #[case::TradingCommand_Balance(TradingCommand::Balance, to_value(6).unwrap())]
        #[case::TradingCommand_Credit(TradingCommand::Credit, to_value(7).unwrap())]

        #[case::DayOfWeek_Monday(DayOfWeek::Monday, to_value(1).unwrap())]
        #[case::DayOfWeek_Tuesday(DayOfWeek::Tuesday, to_value(2).unwrap())]
        #[case::DayOfWeek_Wednesday(DayOfWeek::Wednesday, to_value(3).unwrap())]
        #[case::DayOfWeek_Thursday(DayOfWeek::Thursday, to_value(4).unwrap())]
        #[case::DayOfWeek_Friday(DayOfWeek::Friday, to_value(5).unwrap())]
        #[case::DayOfWeek_Saturday(DayOfWeek::Saturday, to_value(6).unwrap())]
        #[case::DayOfWeek_Sunday(DayOfWeek::Sunday, to_value(7).unwrap())]

        #[case::TransactionType_Open(TransactionType::Open, to_value(0).unwrap())]
        #[case::TransactionType_Pending(TransactionType::Pending, to_value(1).unwrap())]
        #[case::TransactionType_Close(TransactionType::Close, to_value(2).unwrap())]
        #[case::TransactionType_Modify(TransactionType::Modify, to_value(3).unwrap())]
        #[case::TransactionType_Delete(TransactionType::Delete, to_value(4).unwrap())]

        #[case::TransactionStatus_Error(TransactionStatus::Error, to_value(0).unwrap())]
        #[case::TransactionStatus_Pending(TransactionStatus::Pending, to_value(1).unwrap())]
        #[case::TransactionStatus_Accepted(TransactionStatus::Accepted, to_value(3).unwrap())]
        #[case::TransactionStatus_Rejected(TransactionStatus::Rejected, to_value(4).unwrap())]

        #[case::TradeStatus_Rejected(TradeStatus::Modified, to_value("modified").unwrap())]
        #[case::TradeStatus_Deleted(TradeStatus::Deleted, to_value("deleted").unwrap())]
        fn serialize_value<T: Serialize + Debug>(#[case] inp: T, #[case] expected: Value) {
            let serialized = to_value(inp).unwrap();
            assert_eq!(serialized, expected);
        }

        #[rstest]
        #[case::QuoteId_Fixed(QuoteId::Fixed, to_value(1).unwrap())]
        #[case::QuoteId_Float(QuoteId::Float, to_value(2).unwrap())]
        #[case::QuoteId_Depth(QuoteId::Depth, to_value(3).unwrap())]
        #[case::QuoteId_Cross(QuoteId::Cross, to_value(4).unwrap())]

        #[case::QuoteId_Forex(MarginMode::Forex, to_value(101).unwrap())]
        #[case::QuoteId_CfdLeveraged(MarginMode::CFDLeveraged, to_value(102).unwrap())]
        #[case::QuoteId_Cfd(MarginMode::CFD, to_value(103).unwrap())]

        #[case::QuoteId_Forex(ProfitMode::Forex, to_value(5).unwrap())]
        #[case::QuoteId_Cfd(ProfitMode::Cfd, to_value(6).unwrap())]

        #[case::ImpactLevel_Low(ImpactLevel::Low, to_value("1").unwrap())]
        #[case::ImpactLevel_Medium(ImpactLevel::Medium, to_value("2").unwrap())]
        #[case::ImpactLevel_High(ImpactLevel::High, to_value("3").unwrap())]

        #[case::TimePeriod_M1(TimePeriod::PeriodM1, to_value(1).unwrap())]
        #[case::TimePeriod_M5(TimePeriod::PeriodM5, to_value(5).unwrap())]
        #[case::TimePeriod_M15(TimePeriod::PeriodM15, to_value(15).unwrap())]
        #[case::TimePeriod_M30(TimePeriod::PeriodM30, to_value(30).unwrap())]
        #[case::TimePeriod_H1(TimePeriod::PeriodH1, to_value(60).unwrap())]
        #[case::TimePeriod_H4(TimePeriod::PeriodH4, to_value(240).unwrap())]
        #[case::TimePeriod_D1(TimePeriod::PeriodD1, to_value(1440).unwrap())]
        #[case::TimePeriod_W1(TimePeriod::PeriodW1, to_value(10080).unwrap())]
        #[case::TimePeriod_MN11(TimePeriod::PeriodMN1, to_value(43200).unwrap())]

        #[case::TradingAction_1(TradingAction::Buy, to_value(0).unwrap())]
        #[case::TradingAction_1(TradingAction::Sell, to_value(1).unwrap())]

        #[case::TradingCommand_Buy(TradingCommand::Buy, to_value(0).unwrap())]
        #[case::TradingCommand_Sell(TradingCommand::Sell, to_value(1).unwrap())]
        #[case::TradingCommand_BuyLimit(TradingCommand::BuyLimit, to_value(2).unwrap())]
        #[case::TradingCommand_SellLimit(TradingCommand::SellLimit, to_value(3).unwrap())]
        #[case::TradingCommand_BuyStop(TradingCommand::BuyStop, to_value(4).unwrap())]
        #[case::TradingCommand_SellStop(TradingCommand::SellStop, to_value(5).unwrap())]
        #[case::TradingCommand_Balance(TradingCommand::Balance, to_value(6).unwrap())]
        #[case::TradingCommand_Credit(TradingCommand::Credit, to_value(7).unwrap())]

        #[case::DayOfWeek_Monday(DayOfWeek::Monday, to_value(1).unwrap())]
        #[case::DayOfWeek_Tuesday(DayOfWeek::Tuesday, to_value(2).unwrap())]
        #[case::DayOfWeek_Wednesday(DayOfWeek::Wednesday, to_value(3).unwrap())]
        #[case::DayOfWeek_Thursday(DayOfWeek::Thursday, to_value(4).unwrap())]
        #[case::DayOfWeek_Friday(DayOfWeek::Friday, to_value(5).unwrap())]
        #[case::DayOfWeek_Saturday(DayOfWeek::Saturday, to_value(6).unwrap())]
        #[case::DayOfWeek_Sunday(DayOfWeek::Sunday, to_value(7).unwrap())]

        #[case::TransactionType_Open(TransactionType::Open, to_value(0).unwrap())]
        #[case::TransactionType_Pending(TransactionType::Pending, to_value(1).unwrap())]
        #[case::TransactionType_Close(TransactionType::Close, to_value(2).unwrap())]
        #[case::TransactionType_Modify(TransactionType::Modify, to_value(3).unwrap())]
        #[case::TransactionType_Delete(TransactionType::Delete, to_value(4).unwrap())]

        #[case::TransactionStatus_Error(TransactionStatus::Error, to_value(0).unwrap())]
        #[case::TransactionStatus_Pending(TransactionStatus::Pending, to_value(1).unwrap())]
        #[case::TransactionStatus_Accepted(TransactionStatus::Accepted, to_value(3).unwrap())]
        #[case::TransactionStatus_Rejected(TransactionStatus::Rejected, to_value(4).unwrap())]

        #[case::TradeStatus_Rejected(TradeStatus::Modified, to_value("modified").unwrap())]
        #[case::TradeStatus_Rejected(TradeStatus::Modified, to_value("MODIFIED").unwrap())]
        #[case::TradeStatus_Deleted(TradeStatus::Deleted, to_value("deleted").unwrap())]
        #[case::TradeStatus_Deleted(TradeStatus::Deleted, to_value("DELETED").unwrap())]
        fn deserialize_value<T: for<'de> Deserialize<'de> + Debug + PartialEq>(#[case] expected: T, #[case] inp: Value) {
            let deserialized: T = from_value(inp).unwrap();
            assert_eq!(deserialized, expected);
        }
    }
}
