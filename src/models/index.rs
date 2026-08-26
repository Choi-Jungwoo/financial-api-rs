use serde::Deserialize;

use crate::{AShareCode, Adjustment, Thscode, UnixMillis};

use super::{PriceBarItem, TimestampedItems};

/// 指数历史行情包含固定为 `null` 的复权字段。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IndexHistoricalData {
    pub timestamp: UnixMillis,
    pub adjust: Option<Adjustment>,
    pub item: Vec<PriceBarItem>,
}

pub type IndexCatalogData = TimestampedItems<IndexCatalogItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct IndexCatalogItem {
    pub thscode: Thscode,
    pub name: String,
}

pub type IndexConstituentsData = TimestampedItems<IndexConstituentItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct IndexConstituentItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
}
