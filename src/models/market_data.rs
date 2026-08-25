use serde::Deserialize;

use crate::{Thscode, UnixMillis};

/// A-share or index price snapshot payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PriceSnapshotData {
    pub timestamp: Option<UnixMillis>,
    pub total: u64,
    pub item: Vec<PriceSnapshotItem>,
}

/// One latest-price record.
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

/// Historical daily price payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HistoricalData {
    pub timestamp: UnixMillis,
    pub item: Vec<PriceBarItem>,
}

/// One daily OHLCV bar.
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
