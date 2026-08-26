use serde::Deserialize;

use crate::{AssetType, Exchange, Thscode};

use super::TimestampedItems;

/// 一条规范化标的代码记录。
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
