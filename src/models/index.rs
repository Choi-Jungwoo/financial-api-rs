use serde::Deserialize;

use crate::{AShareCode, Adjustment, Thscode, UnixMillis};

use super::{PriceBarItem, TimestampedItems};

/// Index historical prices include a fixed `null` adjustment field.
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
