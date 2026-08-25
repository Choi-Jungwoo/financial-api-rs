use serde::Deserialize;

use crate::{AssetType, Exchange, Thscode};

use super::TimestampedItems;

/// One normalized target-code record.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TickerItem {
    pub thscode: Thscode,
    pub ticker: String,
    pub name: String,
    pub exchange: Option<Exchange>,
    pub asset_type: AssetType,
    pub currency: String,
}

pub type TickerData = TimestampedItems<TickerItem>;
