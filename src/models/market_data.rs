use serde::Deserialize;

use crate::{Thscode, UnixMillis};

/// A 股或指数的行情快照载荷。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PriceSnapshotData {
    pub timestamp: Option<UnixMillis>,
    pub total: u64,
    pub item: Vec<PriceSnapshotItem>,
}

/// 一条最新行情记录。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PriceSnapshotItem {
    pub thscode: Thscode,
    pub ticker: String,
    pub last_price: f64,
    pub price_change: f64,
    pub price_change_ratio_pct: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub prev_price: f64,
    pub volume: f64,
    pub turnover: f64,
}

/// 历史日 K 线载荷。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HistoricalData {
    pub timestamp: UnixMillis,
    pub item: Vec<PriceBarItem>,
}

/// 一条日开高低收量 K 线。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PriceBarItem {
    pub date_ms: UnixMillis,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub volume: f64,
    pub turnover: f64,
}
