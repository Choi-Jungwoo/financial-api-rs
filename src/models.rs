use serde::Deserialize;

use crate::{
    AShareCode, Adjustment, AssetType, AuctionStage, CompactDate, CompanyId, Cursor, DailyInterval,
    DragonTigerBoard, FinancialAbilityKind, FinancialIndicatorId, FinancialPeriod, FinancialReport,
    FiscalPeriod, FundType, HolderRecordScope, ManagerId, NaturalDate, PortfolioAssetType,
    PreciseDecimal, RankTrend, ReportType, Thscode, UnixMillis,
};

/// A timestamped endpoint payload whose business records are stored in `item`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TimestampedItems<T> {
    pub timestamp: UnixMillis,
    pub item: Vec<T>,
}

/// One normalized target-code record.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TickerItem {
    pub thscode: Thscode,
    pub ticker: String,
    pub name: String,
    pub exchange: Option<crate::Exchange>,
    pub asset_type: AssetType,
    pub currency: String,
}

pub type TickerData = TimestampedItems<TickerItem>;

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

/// Index historical prices include a fixed `null` adjustment field.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IndexHistoricalData {
    pub timestamp: UnixMillis,
    pub adjust: Option<Adjustment>,
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

/// Corporate-action events for one A-share target.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AdjustmentFactorsData {
    pub thscode: AShareCode,
    pub ticker: String,
    pub item: Vec<AdjustmentFactorItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AdjustmentFactorItem {
    pub ticker: String,
    pub ex_date_ms: UnixMillis,
    pub dividend_per_share: f64,
    pub per_share_bonus: f64,
}

/// Fields shared by the three listed-company financial statements.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FinancialStatementMeta {
    pub thscode: AShareCode,
    pub ticker: String,
    pub period: FinancialPeriod,
    pub fiscal_year: i32,
    pub fiscal_period: FiscalPeriod,
    pub report_date_ms: UnixMillis,
    pub period_end_ms: UnixMillis,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IncomeStatementItem {
    #[serde(flatten)]
    pub meta: FinancialStatementMeta,
    pub operating_income: Option<f64>,
    pub operating_costs: Option<f64>,
    pub operating_expenses: Option<f64>,
    pub sales_fee: Option<f64>,
    pub manage_fee: Option<f64>,
    pub research_and_development_expenses: Option<f64>,
    pub operating_profit: Option<f64>,
    pub interest_expenses: Option<f64>,
    pub profit_total: Option<f64>,
    pub income_tax_expense: Option<f64>,
    pub net_profit: Option<f64>,
    pub parent_holder_net_profit: Option<f64>,
    pub basic_eps: Option<f64>,
}

pub type IncomeStatementsData = TimestampedItems<IncomeStatementItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BalanceSheetItem {
    #[serde(flatten)]
    pub meta: FinancialStatementMeta,
    pub assets_total: Option<f64>,
    pub total_current_assets: Option<f64>,
    pub non_current_nets_total: Option<f64>,
    pub cash: Option<f64>,
    pub accounts_receivable: Option<f64>,
    pub total_debt: Option<f64>,
    pub holder_equity_total: Option<f64>,
}

pub type BalanceSheetsData = TimestampedItems<BalanceSheetItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CashFlowStatementItem {
    #[serde(flatten)]
    pub meta: FinancialStatementMeta,
    pub act_cash_flow_net: Option<f64>,
    pub invest_cash_flow_net: Option<f64>,
    pub financing_cash_flow_net: Option<f64>,
    pub pay_fixed_assets_etc_cash: Option<f64>,
    pub pay_dividends_profits_interest_cash: Option<f64>,
    pub cash_equivalents_net_addition: Option<f64>,
}

pub type CashFlowStatementsData = TimestampedItems<CashFlowStatementItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FinancialIndicatorsData {
    pub thscode: AShareCode,
    pub report: FinancialReport,
    pub abilities: Vec<FinancialAbility>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FinancialAbility {
    pub ability: FinancialAbilityKind,
    pub indicators: Vec<FinancialIndicator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FinancialIndicator {
    pub index_id: FinancialIndicatorId,
    pub value: Option<String>,
}

pub type TradingDaysData = TimestampedItems<TradingDayItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TradingDayItem {
    pub date_ms: UnixMillis,
    pub date: CompactDate,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AuctionSnapshotData {
    pub timestamp: UnixMillis,
    pub auction_phase: AuctionStage,
    pub data_status: String,
    pub total: u64,
    pub item: Vec<AuctionSnapshotItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AuctionSnapshotItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    pub auction_price: Option<f64>,
    pub auction_pct: Option<f64>,
    pub auction_volume: Option<f64>,
    pub auction_amount: Option<f64>,
    pub auction_unmatched: Option<f64>,
    pub auction_turnover_pct: Option<f64>,
    pub auction_yesterday_ratio_pct: Option<f64>,
    pub auction_volume_ratio: Option<f64>,
    pub pre_close_price: Option<f64>,
    pub open_price: Option<f64>,
    pub last_price: Option<f64>,
    pub float_market_cap: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AuctionBenchmarkData {
    pub timestamp: UnixMillis,
    pub date: NaturalDate,
    pub date_ms: UnixMillis,
    pub item: Vec<AuctionBenchmarkItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AuctionBenchmarkItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    pub auction_pct: Option<f64>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ValuationsData {
    pub timestamp: Option<UnixMillis>,
    pub total: u64,
    pub item: Vec<ValuationItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ValuationItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: Option<String>,
    pub pe_ttm: Option<PreciseDecimal>,
    pub pe_mrq: Option<PreciseDecimal>,
    pub pb_mrq: Option<PreciseDecimal>,
    pub ps_ttm: Option<PreciseDecimal>,
    pub pcf_ttm: Option<PreciseDecimal>,
}

pub type AnomalyData = TimestampedItems<AnomalyItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AnomalyItem {
    pub stock_name: String,
    pub analysis_content: String,
    pub keyword_list: Vec<String>,
    pub thscode: AShareCode,
    pub tag_name: String,
}

pub type HotStockData = TimestampedItems<HotStockItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HotStockItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    pub rank: u64,
    pub heat: String,
    pub rank_change: Option<i64>,
    pub rank_trend: RankTrend,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HotStockHistoryData {
    pub date: NaturalDate,
    pub date_ms: UnixMillis,
    pub item: Vec<HotStockHistoryItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HotStockHistoryItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    pub rank: u64,
}

pub type HotStockTrendData = TimestampedItems<HotStockTrendItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HotStockTrendItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub date: NaturalDate,
    pub date_ms: UnixMillis,
    pub rank: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DragonTigerData {
    pub timestamp: UnixMillis,
    pub board_type: DragonTigerBoard,
    pub trade_date: NaturalDate,
    pub count: u64,
    pub stock_count: u64,
    pub stock_items: Vec<DragonTigerStockItem>,
    pub hot_money_items: Vec<HotMoneyItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ConceptItem {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DragonTigerStockItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    #[serde(default)]
    pub concept_list: Vec<ConceptItem>,
    pub change: Option<f64>,
    pub net_value: Option<f64>,
    pub net_rate: Option<f64>,
    pub hot_rank: Option<u64>,
    pub buy_value: Option<f64>,
    pub sell_value: Option<f64>,
    pub limit_reason: Option<String>,
    pub range_days: Option<u64>,
    pub org_net_value: Option<f64>,
    pub org_net_rate: Option<f64>,
    pub org_buy_num: Option<u64>,
    pub org_sell_num: Option<u64>,
    pub amount: Option<f64>,
    pub hot_money_net_value: Option<f64>,
    pub hot_money_net_rate: Option<f64>,
    pub hot_money_item_net_value: Option<f64>,
    pub hot_money_item_net_rate: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HotMoneyItem {
    pub name: String,
    pub buying: f64,
    pub rows: Vec<DragonTigerStockItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Pagination {
    pub total: u64,
    pub pages: u64,
    pub size: u16,
    pub page: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PoolData<T> {
    pub timestamp: UnixMillis,
    pub pagination: Pagination,
    pub item: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LimitUpItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    pub is_st: bool,
    pub is_new: bool,
    pub last_price: f64,
    pub price_change_ratio_pct: f64,
    pub limit_up_time: String,
    pub limit_up_reason: Option<String>,
    pub continue_day_text: String,
    pub continue_day_cnt: u64,
    pub seal_money: f64,
    pub max_seal_money: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LimitDownItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    pub last_price: f64,
    pub price_change_ratio_pct: f64,
    pub first_limit_time: String,
    pub last_limit_time: String,
    pub turnover_ratio_pct: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LimitBreakItem {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    pub last_price: f64,
    pub price_change_ratio_pct: f64,
    pub open_times: u64,
    pub turnover_ratio_pct: f64,
    pub turnover: f64,
}

pub type LimitUpData = PoolData<LimitUpItem>;
pub type LimitDownData = PoolData<LimitDownItem>;
pub type LimitBreakData = PoolData<LimitBreakItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LadderData {
    pub timestamp: UnixMillis,
    pub window: LadderWindow,
    pub item: Vec<LadderDay>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LadderWindow {
    pub length: u64,
    pub date_list: Vec<CompactDate>,
    pub board_caps: LadderBoardCaps,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LadderBoardCaps {
    pub two_board: u64,
    pub three_board: u64,
    pub four_board: u64,
    pub five_board: u64,
    pub six_board: u64,
    pub seven_over: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LadderDay {
    pub date: CompactDate,
    pub boards: LadderBoards,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LadderBoards {
    pub two_board: Vec<LadderStock>,
    pub three_board: Vec<LadderStock>,
    pub four_board: Vec<LadderStock>,
    pub five_board: Vec<LadderStock>,
    pub six_board: Vec<LadderStock>,
    pub seven_over: Vec<LadderStock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LadderStock {
    pub thscode: AShareCode,
    pub ticker: String,
    pub name: String,
    pub board_num: u64,
    pub seal_nextday: Option<bool>,
    pub sign_level: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundProfileItem {
    pub thscode: Thscode,
    pub ticker: String,
    pub fund_name: Option<String>,
    pub estab_date: Option<UnixMillis>,
    pub company_id: Option<CompanyId>,
    pub mgmt_name: Option<String>,
    pub manager_name: Option<String>,
    pub fund_scale: Option<f64>,
    pub unit_nav: Option<f64>,
    #[serde(default)]
    pub manager_info: Vec<FundManagerRef>,
    #[serde(default)]
    pub trade_rule: Vec<FundTradeRule>,
    #[serde(default)]
    pub rate_info: Vec<FundRateInfo>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundManagerRef {
    pub manager_id: ManagerId,
    pub manager_name: String,
    pub tenure_return_pct: Option<f64>,
    pub tenure_days: Option<u64>,
    pub start_date_ms: Option<UnixMillis>,
    pub end_date_ms: Option<UnixMillis>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundTradeRule {
    pub title: Option<String>,
    pub display_time: Option<String>,
    pub time_ms: Option<UnixMillis>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundRateInfo {
    pub rate_type: Option<String>,
    pub charge_mode: Option<String>,
    pub condition: Option<String>,
    pub standard_rate: Option<f64>,
    pub discounted_rate: Option<f64>,
}

pub type FundProfileData = TimestampedItems<FundProfileItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundHoldingsData {
    pub timestamp: UnixMillis,
    pub item: Vec<FundHoldingItem>,
    pub total_stock_ratio_pct: Option<f64>,
    pub total_bond_ratio_pct: Option<f64>,
    pub total_fund_ratio_pct: Option<f64>,
    pub turnover_rate_pct: Option<f64>,
    pub stock_ratio_pct: Option<f64>,
    pub main_industry: Option<String>,
    pub concentration_ratio: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundHoldingItem {
    pub thscode: Option<Thscode>,
    pub ticker: Option<String>,
    pub stock_name: Option<String>,
    pub hold_ratio: Option<f64>,
    pub asset_type: Option<PortfolioAssetType>,
    pub position_capital: Option<f64>,
    pub position_count: Option<f64>,
    pub security_market_value_rate_pct: Option<f64>,
    pub period_increase_rate_pct: Option<f64>,
    pub investment_rank: Option<u64>,
    pub start_date_ms: Option<UnixMillis>,
    pub end_date_ms: Option<UnixMillis>,
    pub publish_date_ms: Option<UnixMillis>,
    pub modify_time_ms: Option<UnixMillis>,
}

pub type FundNavData = TimestampedItems<FundNavItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundNavItem {
    pub nav_date: UnixMillis,
    pub unit_nav: Option<f64>,
    pub adj_nav: Option<f64>,
}

pub type FundReturnsData = TimestampedItems<FundReturnsItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundReturnsItem {
    pub return_week: Option<f64>,
    pub return_month: Option<f64>,
    pub return_tmonth: Option<f64>,
    pub return_hyear: Option<f64>,
    pub return_year: Option<f64>,
    pub return_twoyear: Option<f64>,
    pub return_tyear: Option<f64>,
    pub return_fyear: Option<f64>,
    pub return_nowyear: Option<f64>,
    pub return_now: Option<f64>,
    pub peer_average_week: Option<f64>,
    pub peer_average_month: Option<f64>,
    pub peer_average_tmonth: Option<f64>,
    pub peer_average_hyear: Option<f64>,
    pub peer_average_year: Option<f64>,
    pub peer_average_twoyear: Option<f64>,
    pub peer_average_tyear: Option<f64>,
    pub peer_average_fyear: Option<f64>,
    pub rank_week: Option<u64>,
    pub rank_month: Option<u64>,
    pub rank_tmonth: Option<u64>,
    pub rank_hyear: Option<u64>,
    pub rank_year: Option<u64>,
    pub rank_twoyear: Option<u64>,
    pub rank_tyear: Option<u64>,
    pub rank_fyear: Option<u64>,
    pub rank_total_week: Option<u64>,
    pub rank_total_month: Option<u64>,
    pub rank_total_tmonth: Option<u64>,
    pub rank_total_hyear: Option<u64>,
    pub rank_total_year: Option<u64>,
    pub rank_total_twoyear: Option<u64>,
    pub rank_total_tyear: Option<u64>,
    pub rank_total_fyear: Option<u64>,
}

pub type FundHoldersData = TimestampedItems<FundHolderItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundHolderItem {
    pub merge_scope: HolderRecordScope,
    pub report_date_ms: UnixMillis,
    pub ins_position: Option<f64>,
    pub holder_amount: Option<u64>,
    pub avg_holder_share: Option<f64>,
    pub psnl_rate: Option<f64>,
    pub mgmt_staff_hold_rate: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundTopHoldersData {
    pub timestamp: UnixMillis,
    pub limit: u8,
    pub item: Vec<FundTopHolderItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundTopHolderItem {
    pub holder_id: Option<String>,
    pub holder_code: Option<String>,
    pub holder_name: Option<String>,
    pub holder_type: Option<String>,
    pub rank: Option<u64>,
    pub hold_share: Option<f64>,
    pub hold_rate_pct: Option<f64>,
    pub report_date_ms: Option<UnixMillis>,
    pub publish_date_ms: Option<UnixMillis>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundDividendsData {
    pub timestamp: UnixMillis,
    pub dividend_count: Option<u64>,
    pub dividend_total: Option<f64>,
    pub item: Vec<FundDividendItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundDividendItem {
    pub per_ten_cash_before_tax: Option<f64>,
    pub per_ten_cash_after_tax: Option<f64>,
    pub progress: Option<String>,
    pub publish_date_ms: Option<UnixMillis>,
    pub registration_date_ms: Option<UnixMillis>,
    pub ex_dividend_date_ms: Option<UnixMillis>,
    pub payment_date_ms: Option<UnixMillis>,
    pub reinvestment_date_ms: Option<UnixMillis>,
    pub profit_base_date_ms: Option<UnixMillis>,
    pub in_dividend_date_ms: Option<UnixMillis>,
}

pub type FundDiagnosticsData = TimestampedItems<FundDiagnosticsItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundDiagnosticsItem {
    pub thscode: Thscode,
    pub ticker: String,
    pub fund_type: FundType,
    pub peer_code: String,
    pub dimensions: serde_json::Value,
    pub peer_dimensions: serde_json::Value,
    pub probabilities: serde_json::Value,
    pub ranges: serde_json::Value,
    pub resilience: serde_json::Value,
    pub peer_resilience: serde_json::Value,
}

pub type FundFinancialIndicatorsData = TimestampedItems<FundFinancialIndicatorsItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundFinancialIndicatorsItem {
    pub start_date_ms: Option<UnixMillis>,
    pub end_date_ms: Option<UnixMillis>,
    pub publish_date_ms: Option<UnixMillis>,
    pub distribution_profit: Option<f64>,
    pub current_profit: Option<f64>,
    pub current_income: Option<f64>,
    pub distribution_share_profit: Option<f64>,
    pub average_nav_profit_margin: Option<f64>,
    pub average_share_current_profit: Option<f64>,
    pub share_nav: Option<f64>,
    pub sum_share_nav: Option<f64>,
    pub asset_nav: Option<f64>,
    pub sum_nav_rate: Option<f64>,
    pub nav_rate: Option<f64>,
}

pub type FundIncomeStatementsData = TimestampedItems<FundIncomeStatementItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundIncomeStatementItem {
    pub start_date_ms: Option<UnixMillis>,
    pub end_date_ms: Option<UnixMillis>,
    pub publish_date_ms: Option<UnixMillis>,
    pub income: Option<f64>,
    pub investment_income: Option<f64>,
    pub stock_investment_income: Option<f64>,
    pub bond_investment_income: Option<f64>,
    pub fund_investment_income: Option<f64>,
    pub dividend_income: Option<f64>,
    pub interest_income: Option<f64>,
    pub fair_value_income: Option<f64>,
    pub exchange_income: Option<f64>,
    pub other_income: Option<f64>,
    pub total_income: Option<f64>,
    pub fee: Option<f64>,
    pub manager_reward: Option<f64>,
    pub custodian_fee: Option<f64>,
    pub transaction_cost: Option<f64>,
    pub tax_surcharge: Option<f64>,
    pub total_fee: Option<f64>,
    pub total_profit: Option<f64>,
    pub net_profit: Option<f64>,
}

pub type FundBalanceSheetsData = TimestampedItems<FundBalanceSheetItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundBalanceSheetItem {
    pub start_date_ms: Option<UnixMillis>,
    pub end_date_ms: Option<UnixMillis>,
    pub publish_date_ms: Option<UnixMillis>,
    pub total_assets: Option<f64>,
    pub bank_deposit: Option<f64>,
    pub fund_investment: Option<f64>,
    pub stock_investment: Option<f64>,
    pub bond_investment: Option<f64>,
    pub transactional_financial_assets: Option<f64>,
    pub other_assets: Option<f64>,
    pub total_liability: Option<f64>,
    pub other_liability: Option<f64>,
    pub owner_total_equity: Option<f64>,
    pub undistributed_profit: Option<f64>,
    pub liability_and_owner_equity: Option<f64>,
}

pub type FundCompanyData = TimestampedItems<FundCompanyItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundCompanyItem {
    pub company_id: CompanyId,
    pub company_name: String,
    pub company_type: String,
    pub established_date_ms: UnixMillis,
    pub fund_count: u64,
    pub scale: f64,
}

pub type FundIndustryAllocationData = TimestampedItems<FundIndustryAllocationItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundIndustryAllocationItem {
    pub report_period: Option<String>,
    pub industry_name: Option<String>,
    pub ratio_pct: Option<f64>,
}

pub type FundIndicatorHistoryData = TimestampedItems<FundIndicatorHistoryItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundIndicatorHistoryItem {
    pub date_ms: UnixMillis,
    pub rsi_pct: Option<f64>,
    pub donchian_channel: Option<f64>,
    pub track_index_pe_ttm_five_year_percentile: Option<f64>,
}

pub type FundDrawdownsData = TimestampedItems<FundDrawdownsItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundDrawdownsItem {
    pub thscode: Thscode,
    pub ticker: String,
    pub week: Option<f64>,
    pub month: Option<f64>,
    pub tmonth: Option<f64>,
    pub hyear: Option<f64>,
    pub year: Option<f64>,
    pub twoyear: Option<f64>,
    pub tyear: Option<f64>,
    pub fyear: Option<f64>,
    pub nowyear: Option<f64>,
    pub now: Option<f64>,
}

pub type FundManagerStyleData = TimestampedItems<FundManagerStyleItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundManagerStyleItem {
    pub representative_fund_thscode: Option<Thscode>,
    pub representative_fund_ticker: Option<String>,
    pub representative_fund_name: Option<String>,
    pub investment_idea: Option<String>,
    pub total_fund_scale: Option<f64>,
    pub industry_preferences: Option<serde_json::Value>,
}

pub type FundManagerPerformanceData = TimestampedItems<FundManagerPerformanceItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundManagerPerformanceItem {
    pub date_ms: UnixMillis,
    pub manager_return_pct: Option<f64>,
    pub peer_return_pct: Option<f64>,
    pub benchmark_return_pct: Option<f64>,
}

pub type FundManagerExperienceData = TimestampedItems<FundManagerExperienceItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundManagerExperienceItem {
    pub awards: serde_json::Value,
    pub heavy_assets: serde_json::Value,
    pub investment_history: serde_json::Value,
}

pub type FundManagerDetailData = TimestampedItems<FundManagerDetailItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundManagerDetailItem {
    pub manager_id: ManagerId,
    pub manager_name: String,
    pub sex: Option<String>,
    pub degree: Option<String>,
    pub company_id: Option<CompanyId>,
    pub company_name: Option<String>,
    pub resume: Option<String>,
    pub photo_url: Option<String>,
    pub annual_return_pct: Option<f64>,
    pub maximum_return_pct: Option<f64>,
    pub radar_comparison: Vec<FundManagerRadarComparison>,
}

/// One manager-versus-peer radar node aligned by category and horizon.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundManagerRadarComparison {
    pub fund_category: String,
    pub horizon: String,
    pub manager_metrics: serde_json::Value,
    pub manager_scores: serde_json::Value,
    pub peer_average_scores: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundMarketSnapshotData {
    pub timestamp: Option<UnixMillis>,
    pub item: Vec<FundMarketSnapshotItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundMarketSnapshotItem {
    pub thscode: Thscode,
    pub ticker: String,
    pub last_price: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub prev_price: f64,
    pub price_change_ratio_pct: f64,
    pub price_change: f64,
    pub price_amplitude_ratio_pct: Option<f64>,
    pub volume: f64,
    pub turnover: f64,
    pub turnover_ratio_pct: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundMarketHistoricalData {
    pub timestamp: UnixMillis,
    pub thscode: Thscode,
    pub interval: DailyInterval,
    pub adjust: Option<Adjustment>,
    pub item: Vec<PriceBarItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundNewsData {
    pub timestamp: UnixMillis,
    pub limit: u32,
    pub offset: Option<Cursor>,
    pub has_more: bool,
    pub item: Vec<FundNewsItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundNewsItem {
    pub id: String,
    pub content_type: Option<String>,
    pub title: String,
    pub summary: Option<String>,
    pub source: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub author: Option<String>,
    pub publish_time_ms: Option<UnixMillis>,
    pub top: Option<bool>,
}

pub type FundOfferingsData = TimestampedItems<FundOfferingItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundOfferingItem {
    pub thscode: Option<Thscode>,
    pub ticker: String,
    pub subscription_start_ms: Option<UnixMillis>,
    pub subscription_end_ms: Option<UnixMillis>,
}

pub type FundPortfolioHistoryData = TimestampedItems<FundPortfolioHistoryItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundPortfolioHistoryItem {
    pub thscode: Option<Thscode>,
    pub ticker: Option<String>,
    pub name: Option<String>,
    pub asset_type: PortfolioAssetType,
    pub hold_ratio: Option<f64>,
    pub market_value: Option<f64>,
    pub period_increase_pct: Option<f64>,
    pub rank: Option<u64>,
    pub report_type: ReportType,
    pub end_date_ms: UnixMillis,
}

pub type FundReportDatesData = TimestampedItems<FundReportDateItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundReportDateItem {
    pub report_type: ReportType,
    pub report_type_name: String,
    pub start_date_ms: UnixMillis,
    pub end_date_ms: UnixMillis,
}

pub type FundAssetAllocationData = TimestampedItems<FundAssetAllocationItem>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundAssetAllocationItem {
    pub report_date_ms: UnixMillis,
    pub stock_ratio_pct: Option<f64>,
    pub bond_ratio_pct: Option<f64>,
    pub deposit_ratio_pct: Option<f64>,
    pub other_ratio_pct: Option<f64>,
}
