#![doc = include_str!("../README.md")]

mod a_share;
mod client;
mod endpoints;
mod error;
mod fund;
mod index;
mod market_dump;
mod meta;
mod models;
mod types;

pub use a_share::{FinancialRange, PriceSnapshotSelection};
pub use client::{ApiKey, Client, ClientBuilder, Response};
pub use endpoints::{EndpointInfo, SUPPORTED_ENDPOINTS};
pub use error::{BusinessError, BusinessErrorKind, Error, ValidationError};
pub use market_dump::{MarketDumpUrl, SecretUrl};
pub use meta::{TickerListRequest, TickerSearchRequest};
pub use models::*;
pub use serde_json::Value as JsonValue;
pub use types::{
    AShareCode, Adjustment, AnomalyTag, AssetType, AuctionPhase, AuctionStage, CompactDate,
    CompanyId, Cursor, DailyInterval, DragonTigerBoard, Exchange, FinancialAbilityKind,
    FinancialIndicatorId, FinancialPeriod, FinancialReport, FiscalPeriod, FundCategoryCode,
    FundNavType, FundRange, FundType, HolderMergeScope, HolderRecordScope, HotListPeriod, IndexTag,
    LimitBreakSortField, LimitDownSortField, LimitUpSortField, ManagerId, ManagerPerformanceRange,
    NaturalDate, OfferingStatus, OptionalInput, Page, PortfolioAssetType, PreciseDecimal,
    RankTrend, ReportType, ShanghaiDateMillis, SortDirection, Thscode, UnixMillis,
};
