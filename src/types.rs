use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::macros::{format_description, offset};
use time::{Date, Time};

use crate::ValidationError;

/// Non-empty search text used for cross-market target lookup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct SearchQuery(String);

impl SearchQuery {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValidationError::new("q", "must not be empty"));
        }
        Ok(Self(value))
    }
}

/// Complete target code including its market suffix.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct Thscode(String);

impl Thscode {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let value = value.as_ref().trim().to_ascii_uppercase();
        if value.contains(',') {
            return Err(ValidationError::new(
                "thscode",
                "must contain exactly one target",
            ));
        }
        let Some((ticker, suffix)) = value.split_once('.') else {
            return Err(ValidationError::new(
                "thscode",
                "must include a market suffix",
            ));
        };
        if ticker.is_empty()
            || suffix.is_empty()
            || suffix.contains('.')
            || !ticker.bytes().all(|byte| byte.is_ascii_alphanumeric())
            || !suffix.bytes().all(|byte| byte.is_ascii_alphanumeric())
        {
            return Err(ValidationError::new(
                "thscode",
                "must use TICKER.SUFFIX with ASCII letters or digits",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Thscode {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for Thscode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Complete target code restricted to the A-share target universe.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct AShareCode(Thscode);

impl AShareCode {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let code = Thscode::new(value)?;
        let (ticker, suffix) = code
            .as_str()
            .split_once('.')
            .expect("validated thscode always contains a dot");
        if ticker.len() != 6
            || !ticker.bytes().all(|byte| byte.is_ascii_digit())
            || !matches!(suffix, "SH" | "SZ" | "BJ")
        {
            return Err(ValidationError::new(
                "thscode",
                "must be a six-digit A-share code ending in SH, SZ, or BJ",
            ));
        }
        Ok(Self(code))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for AShareCode {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for AShareCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl From<AShareCode> for Thscode {
    fn from(value: AShareCode) -> Self {
        value.0
    }
}

/// Non-negative Unix timestamp in milliseconds.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display,
)]
#[serde(transparent)]
pub struct UnixMillis(i64);

impl UnixMillis {
    pub const fn new(value: i64) -> Result<Self, ValidationError> {
        if value < 0 {
            return Err(ValidationError::new(
                "timestamp",
                "must be non-negative milliseconds since the Unix epoch",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for UnixMillis {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i64::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// JSON decimal number whose source digits are preserved without binary-float conversion.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::Display, derive_more::From,
)]
#[serde(transparent)]
#[display("{_0}")]
pub struct PreciseDecimal(serde_json::Number);

impl PreciseDecimal {
    /// Borrow the lossless JSON number representation.
    #[must_use]
    pub const fn as_number(&self) -> &serde_json::Number {
        &self.0
    }
}

/// Validated Gregorian natural day serialized as `YYYY-MM-DD`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::From)]
pub struct NaturalDate(Date);

impl NaturalDate {
    pub fn parse(value: &str) -> Result<Self, ValidationError> {
        if value.len() != 10 {
            return Err(ValidationError::new("date", "must use YYYY-MM-DD format"));
        }
        Date::parse(value, format_description!("[year]-[month]-[day]"))
            .map(Self)
            .map_err(|_| ValidationError::new("date", "must be a valid Gregorian date"))
    }

    pub(crate) fn checked_add_years(self, years: i32) -> Option<Self> {
        let target_year = self.0.year().checked_add(years)?;
        self.0
            .replace_year(target_year)
            .or_else(|_| self.0.replace_day(28)?.replace_year(target_year))
            .ok()
            .map(Self)
    }
}

impl fmt::Display for NaturalDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self
            .0
            .format(format_description!("[year]-[month]-[day]"))
            .map_err(|_| fmt::Error)?;
        formatter.write_str(&value)
    }
}

impl FromStr for NaturalDate {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for NaturalDate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for NaturalDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Validated Gregorian natural day encoded on the wire as `YYYYMMDD`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompactDate(NaturalDate);

impl CompactDate {
    pub fn parse(value: &str) -> Result<Self, ValidationError> {
        if value.len() != 8 {
            return Err(ValidationError::new("date", "must use YYYYMMDD format"));
        }
        Date::parse(value, format_description!("[year][month][day]"))
            .map(|date| Self(NaturalDate(date)))
            .map_err(|_| ValidationError::new("date", "must be a valid Gregorian date"))
    }

    #[must_use]
    pub const fn natural_date(self) -> NaturalDate {
        self.0
    }
}

impl fmt::Display for CompactDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self
            .0
            .0
            .format(format_description!("[year][month][day]"))
            .map_err(|_| fmt::Error)?;
        formatter.write_str(&value)
    }
}

impl FromStr for CompactDate {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for CompactDate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for CompactDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// A natural date represented as its `Asia/Shanghai` midnight Unix milliseconds.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display,
)]
#[serde(transparent)]
pub struct ShanghaiDateMillis(UnixMillis);

impl ShanghaiDateMillis {
    pub fn from_date(date: NaturalDate) -> Result<Self, ValidationError> {
        let milliseconds = date
            .0
            .with_time(Time::MIDNIGHT)
            .assume_offset(offset!(+8))
            .unix_timestamp_nanos()
            / 1_000_000;
        let milliseconds = i64::try_from(milliseconds)
            .map_err(|_| ValidationError::new("date", "is outside the timestamp range"))?;
        UnixMillis::new(milliseconds).map(Self)
    }

    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
}

impl TryFrom<NaturalDate> for ShanghaiDateMillis {
    type Error = ValidationError;

    fn try_from(value: NaturalDate) -> Result<Self, Self::Error> {
        Self::from_date(value)
    }
}

/// Mutually exclusive recent-period or timestamp-range financial query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinancialRange {
    Recent { limit: u8 },
    Between { start: UnixMillis, end: UnixMillis },
}

impl FinancialRange {
    pub const fn recent(limit: u8) -> Result<Self, ValidationError> {
        if limit == 0 || limit > 20 {
            return Err(ValidationError::new("limit", "must be in the range 1..=20"));
        }
        Ok(Self::Recent { limit })
    }

    pub const fn between(start: UnixMillis, end: UnixMillis) -> Result<Self, ValidationError> {
        if end.0 < start.0 {
            return Err(ValidationError::new(
                "end",
                "must not be earlier than start",
            ));
        }
        Ok(Self::Between { start, end })
    }
}

macro_rules! identifier_type {
    ($(#[$meta:meta])* $name:ident, $field:literal) => {
        $(#[$meta])*
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            derive_more::Display,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
                let value = value.into();
                let value = value.trim();
                if value.is_empty() {
                    return Err(ValidationError::new($field, "must not be empty"));
                }
                Ok(Self(value.to_owned()))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = ValidationError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

identifier_type!(
    /// Opaque fund-manager identifier discovered from fund profile data.
    ManagerId,
    "manager_id"
);
identifier_type!(
    /// Opaque fund-company identifier discovered from fund profile data.
    CompanyId,
    "company_id"
);
identifier_type!(
    /// Fund disclosure report type discovered from a report-dates endpoint.
    ReportType,
    "report_type"
);

/// Opaque pagination cursor returned by a cursor-paged endpoint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct Cursor(String);

impl Cursor {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValidationError::new("offset", "must not be empty"));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Cursor {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for Cursor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Financial report identifier in `YYYY-[1-4]` form.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct FinancialReport(String);

impl FinancialReport {
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let bytes = value.as_bytes();
        let valid = bytes.len() == 6
            && bytes[..4].iter().all(u8::is_ascii_digit)
            && bytes[4] == b'-'
            && matches!(bytes[5], b'1'..=b'4');
        if !valid {
            return Err(ValidationError::new("report", "must use YYYY-[1-4] format"));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for FinancialReport {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for FinancialReport {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

// Internal expansion grammar: enum variants map one-to-one to documented wire
// strings; the expansion only adds derives, `as_str`, Display, and Serialize.
macro_rules! wire_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident => $wire:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, derive_more::Display)]
        pub enum $name {
            $($(#[$variant_meta])* #[display($wire)] #[serde(rename = $wire)] $variant),+
        }

        impl $name {
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $wire),+
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }
    };
}

wire_enum! {
    /// Historical price adjustment convention.
    pub enum Adjustment {
        None => "none",
        Forward => "forward",
        Backward => "backward",
    }
}

wire_enum! {
    /// Financial statement report frequency.
    pub enum FinancialPeriod {
        Annual => "annual",
        Quarterly => "quarterly",
    }
}

wire_enum! {
    /// Fiscal period returned by listed-company financial statements.
    pub enum FiscalPeriod {
        FullYear => "FY",
        FirstQuarter => "Q1",
        SecondQuarter => "Q2",
        ThirdQuarter => "Q3",
        FourthQuarter => "Q4",
    }
}

wire_enum! {
    /// Fixed financial-analysis ability group.
    pub enum FinancialAbilityKind {
        Growth => "growth",
        Profitability => "profitability",
        Solvency => "solvency",
        Operation => "operation",
        CashFlow => "cash-flow",
    }
}

wire_enum! {
    /// Financial indicator identifier defined by the upstream contract.
    pub enum FinancialIndicatorId {
        TotalAssetsGrowthRatio => "total_assets_growth_ratio",
        NetProfitYoyGrowthRatio => "net_profit_yoy_growth_ratio",
        OperatingIncomeYoyGrowthRatio => "operating_income_yoy_growth_ratio",
        OperatingProfitYoyGrowthRatio => "operating_profit_yoy_growth_ratio",
        SaleGrossMargin => "sale_gross_margin",
        SaleNetInterestRatio => "sale_net_interest_ratio",
        TotalAssetsNetRatio => "total_assets_net_ratio",
        DeductWeightedAverageReturnOnEquity => "index_deduct_weighted_avg_roe",
        WeightedAverageReturnOnEquity => "index_weighted_avg_roe",
        CurrentRatio => "current_ratio",
        QuickRatio => "quick_ratio",
        AssetsDebtRatio => "assets_debt_ratio",
        CashRatio => "cash_ratio",
        EarnedInterestMultiple => "earned_interest_multiple",
        LongTermDebtEquityRatio => "long_term_debt_equity_ratio",
        TotalAssetsTurnoverRatio => "total_assets_turnover_ratio",
        InventoryTurnoverRatio => "inventory_turnover_ratio",
        CurrentAssetsTurnoverRatio => "current_assets_turnover_ratio",
        ReceiveAccountTurnoverRatio => "receive_account_turnover_ratio",
        CashOperatingIndex => "cash_operating_index",
        OperatingCashFlowNetDivideIncome => "operating_cash_flow_net_divide_income",
        NetProfitCashContent => "net_profit_cash_content",
        OperatingCashNetYoyGrowthRatio => "operating_cash_net_yoy_growth_ratio",
        CashMeetInvestRatio => "cash_meet_invest_ratio",
    }
}

wire_enum! {
    /// Fixed daily K-line interval returned by fund historical prices.
    pub enum DailyInterval {
        OneDay => "1d",
    }
}

wire_enum! {
    /// Tonghuashun index catalog tag.
    pub enum IndexTag {
        Concept => "cn_concept",
        Region => "region",
        Featured => "tszs",
        Industry => "industry",
    }
}

wire_enum! {
    /// Auction observation stage.
    pub enum AuctionStage {
        Live => "live",
        Final => "final",
    }
}

wire_enum! {
    /// A-share anomaly tag.
    pub enum AnomalyTag {
        LimitUp => "LIMIT_UP",
        LimitDown => "LIMIT_DOWN",
        SharpRise => "SHARP_RISE",
        SharpFall => "SHARP_FALL",
        RapidRally => "RAPID_RALLY",
        RapidDecline => "RAPID_DECLINE",
    }
}

wire_enum! {
    /// Ranking aggregation period.
    pub enum HotListPeriod {
        Day => "day",
        Hour => "hour",
    }
}

wire_enum! {
    /// Dragon-tiger list board filter.
    pub enum DragonTigerBoard {
        All => "all",
        Organization => "org",
        HotMoney => "hot_money",
    }
}

wire_enum! {
    /// Sort direction used by paged special-data pools.
    pub enum SortDirection {
        Ascending => "asc",
        Descending => "desc",
    }
}

wire_enum! {
    /// Sort key for the limit-up pool.
    pub enum LimitUpSortField {
        LastPrice => "last_price",
        ConsecutiveDays => "continue_day_cnt",
        SealedAmount => "seal_money",
        LimitUpTime => "limit_up_time",
    }
}

wire_enum! {
    /// Sort key for the limit-down pool.
    pub enum LimitDownSortField {
        LastLimitTime => "last_limit_time",
        FirstLimitTime => "first_limit_time",
        LastPrice => "last_price",
        PriceChangeRatio => "price_change_ratio_pct",
        TurnoverRatio => "turnover_ratio_pct",
    }
}

wire_enum! {
    /// Sort key for the limit-break pool.
    pub enum LimitBreakSortField {
        PriceChangeRatio => "price_change_ratio_pct",
        OpenTimes => "open_times",
        LastPrice => "last_price",
        TurnoverRatio => "turnover_ratio_pct",
        Turnover => "turnover",
    }
}

wire_enum! {
    /// Fund target universe.
    pub enum FundType {
        Otc => "otc",
        Exchange => "exchange",
        Reits => "reits",
    }
}

wire_enum! {
    /// Holder record merge policy.
    pub enum HolderMergeScope {
        All => "all",
        Merged => "merged",
        Separate => "separate",
    }
}

wire_enum! {
    /// Fund NAV history range.
    pub enum FundRange {
        Week => "week",
        Month => "month",
        ThreeMonths => "tmonth",
        HalfYear => "hyear",
        Year => "year",
        TwoYears => "twoyear",
        ThreeYears => "tyear",
        FiveYears => "fyear",
    }
}

wire_enum! {
    /// Fund NAV series selection.
    pub enum FundNavType {
        Unit => "unit",
        Adjusted => "adj",
        UnitAndAdjusted => "unit,adj",
    }
}

wire_enum! {
    /// Fund manager performance range.
    pub enum ManagerPerformanceRange {
        Month => "month",
        ThreeMonths => "tmonth",
        Year => "year",
        CurrentYear => "nowyear",
        SinceInception => "now",
    }
}

wire_enum! {
    /// New fund offering subscription state.
    pub enum OfferingStatus {
        Active => "active",
        Upcoming => "upcoming",
    }
}

/// Validated pagination for special-data pools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Page {
    number: u32,
    size: u16,
}

impl Page {
    pub const fn new(number: u32, size: u16) -> Result<Self, ValidationError> {
        if number == 0 {
            return Err(ValidationError::new("page", "must be at least 1"));
        }
        if size == 0 || size > 200 {
            return Err(ValidationError::new("size", "must be in the range 1..=200"));
        }
        Ok(Self { number, size })
    }

    #[must_use]
    pub const fn number(self) -> u32 {
        self.number
    }

    #[must_use]
    pub const fn size(self) -> u16 {
        self.size
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            number: 1,
            size: 50,
        }
    }
}

pub(crate) fn join_values<T: fmt::Display>(
    field: &'static str,
    values: &[T],
    maximum: Option<usize>,
) -> Result<String, ValidationError> {
    if values.is_empty() {
        return Err(ValidationError::new(field, "must not be empty"));
    }
    if maximum.is_some_and(|maximum| values.len() > maximum) {
        return Err(ValidationError::new(
            field,
            "contains more values than the endpoint allows",
        ));
    }
    Ok(values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(","))
}

wire_enum! {
    /// Exchange filter supported by the metadata API.
    pub enum Exchange {
        Shanghai => "SH",
        Shenzhen => "SZ",
        Beijing => "BJ",
    }
}

wire_enum! {
    /// Normalized leaf asset type.
    pub enum AssetType {
        AShare => "a-share",
        AShareIndex => "a-share-index",
        FundOtc => "fund-otc",
        FundEtf => "fund-etf",
        FundLof => "fund-lof",
    }
}

wire_enum! {
    /// Direction of a hot-list rank change.
    pub enum RankTrend {
        Up => "up",
        Down => "down",
        Flat => "flat",
        Unknown => "unknown",
    }
}

wire_enum! {
    /// Actual disclosure scope returned for a fund-holder record.
    pub enum HolderRecordScope {
        Merged => "merged",
        Separate => "separate",
    }
}

wire_enum! {
    /// Asset class used in a disclosed fund portfolio.
    pub enum PortfolioAssetType {
        Stock => "stock",
        Bond => "bond",
        Fund => "fund",
    }
}
