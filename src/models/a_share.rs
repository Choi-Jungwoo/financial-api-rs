use serde::{Deserialize, Deserializer};

use crate::{
    AShareCode, AuctionPhase, CompactDate, DragonTigerBoard, FinancialAbilityKind,
    FinancialIndicatorId, FinancialPeriod, FinancialReport, FiscalPeriod, NaturalDate,
    PreciseDecimal, RankTrend, UnixMillis, ValidationError,
};

use super::TimestampedItems;

/// 指定 A 股标的的复权事件。
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

/// 上市公司三类财务报表共用的字段。
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AuctionSnapshotData {
    pub timestamp: UnixMillis,
    pub auction_phase: AuctionPhase,
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
    #[serde(deserialize_with = "deserialize_ladder_dates")]
    pub date_list: Vec<NaturalDate>,
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
    #[serde(deserialize_with = "deserialize_ladder_date")]
    pub date: NaturalDate,
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

fn deserialize_ladder_dates<'de, D>(deserializer: D) -> Result<Vec<NaturalDate>, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|value| parse_ladder_date(&value).map_err(serde::de::Error::custom))
        .collect()
}

fn deserialize_ladder_date<'de, D>(deserializer: D) -> Result<NaturalDate, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_ladder_date(&value).map_err(serde::de::Error::custom)
}

fn parse_ladder_date(value: &str) -> Result<NaturalDate, ValidationError> {
    match value.len() {
        8 => CompactDate::parse(value).map(CompactDate::natural_date),
        10 => NaturalDate::parse(value),
        _ => Err(ValidationError::new(
            "date",
            "must use YYYYMMDD or YYYY-MM-DD format",
        )),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        AuctionSnapshotData, FinancialIndicatorsData, HotStockData, IncomeStatementsData,
        LadderData, TradingDaysData, ValuationsData,
    };

    #[test]
    fn auction_snapshot_accepts_the_observed_closed_response_phase() {
        let snapshot: AuctionSnapshotData = serde_json::from_value(json!({
            "timestamp": 1_i64,
            "auction_phase": "closed",
            "data_status": "final",
            "total": 0,
            "item": []
        }))
        .unwrap();

        assert_eq!(snapshot.auction_phase.as_str(), "closed");
    }

    #[test]
    fn financial_indicators_accept_observed_upstream_identifiers() {
        let indicators: FinancialIndicatorsData = serde_json::from_value(json!({
            "thscode": "600519.SH",
            "report": "2025-4",
            "abilities": [{
                "ability": "growth",
                "indicators": [
                    {
                        "index_id": "calculate_operating_income_yoy_growth_ratio",
                        "value": "-1.20600400"
                    },
                    {
                        "index_id": "calculate_operating_profit_yoy_growth_ratio",
                        "value": "-4.07693800"
                    },
                    {
                        "index_id": "fixed_asset_invest_expansion_ratio",
                        "value": "2.81954600"
                    },
                    {
                        "index_id": "calculate_parent_holder_net_profit_yoy_growth_ratio",
                        "value": "-4.53225500"
                    }
                ]
            }]
        }))
        .unwrap();

        let identifiers = indicators.abilities[0]
            .indicators
            .iter()
            .map(|indicator| indicator.index_id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            identifiers,
            [
                "operating_income_yoy_growth_ratio",
                "operating_profit_yoy_growth_ratio",
                "fixed_asset_invest_expansion_ratio",
                "parent_holder_net_profit_yoy_growth_ratio",
            ]
        );
    }

    #[test]
    fn finite_response_states_reject_unknown_wire_values() {
        let hot_stock = json!({
            "timestamp": 1_i64,
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "name": "贵州茅台",
                "rank": 1,
                "heat": "100",
                "rank_trend": "sideways"
            }]
        });
        assert!(serde_json::from_value::<HotStockData>(hot_stock).is_err());

        let statement = json!({
            "timestamp": 1_i64,
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "period": "annual",
                "fiscal_year": 2025,
                "fiscal_period": "H1",
                "report_date_ms": 1_i64,
                "period_end_ms": 1_i64,
                "currency": "CNY"
            }]
        });
        assert!(serde_json::from_value::<IncomeStatementsData>(statement).is_err());

        for indicators in [
            json!({
                "thscode": "600519.SH",
                "report": "2025-4",
                "abilities": [{"ability": "momentum", "indicators": []}]
            }),
            json!({
                "thscode": "600519.SH",
                "report": "2025-4",
                "abilities": [{
                    "ability": "growth",
                    "indicators": [{"index_id": "unknown_ratio", "value": null}]
                }]
            }),
        ] {
            assert!(serde_json::from_value::<FinancialIndicatorsData>(indicators).is_err());
        }
    }

    #[test]
    fn compact_wire_dates_are_validated_at_deserialization() {
        let trading_days = json!({
            "timestamp": 1_i64,
            "item": [{"date_ms": 1_i64, "date": "20250229"}]
        });
        assert!(serde_json::from_value::<TradingDaysData>(trading_days).is_err());

        let ladder = json!({
            "timestamp": 1_i64,
            "window": {
                "length": 1,
                "date_list": ["20250229"],
                "board_caps": {
                    "two_board": 0,
                    "three_board": 0,
                    "four_board": 0,
                    "five_board": 0,
                    "six_board": 0,
                    "seven_over": 0
                }
            },
            "item": []
        });
        assert!(serde_json::from_value::<LadderData>(ladder).is_err());
    }

    #[test]
    fn ladder_accepts_observed_hyphenated_natural_dates() {
        let ladder: LadderData = serde_json::from_value(json!({
            "timestamp": 1_i64,
            "window": {
                "length": 2,
                "date_list": ["2026-08-26", "20260825"],
                "board_caps": {
                    "two_board": 4,
                    "three_board": 4,
                    "four_board": 4,
                    "five_board": 2,
                    "six_board": 1,
                    "seven_over": 1
                }
            },
            "item": [
                {
                    "date": "2026-08-26",
                    "boards": {
                        "two_board": [],
                        "three_board": [],
                        "four_board": [],
                        "five_board": [],
                        "six_board": [],
                        "seven_over": []
                    }
                },
                {
                    "date": "20260825",
                    "boards": {
                        "two_board": [],
                        "three_board": [],
                        "four_board": [],
                        "five_board": [],
                        "six_board": [],
                        "seven_over": []
                    }
                }
            ]
        }))
        .unwrap();

        assert_eq!(ladder.window.date_list[0].to_string(), "2026-08-26");
        assert_eq!(ladder.window.date_list[1].to_string(), "2026-08-25");
        assert_eq!(ladder.item[0].date.to_string(), "2026-08-26");
        assert_eq!(ladder.item[1].date.to_string(), "2026-08-25");
    }

    #[test]
    fn documented_nullable_valuation_metadata_is_preserved() {
        let valuations: ValuationsData = serde_json::from_value(json!({
            "timestamp": null,
            "total": 1,
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "name": null,
                "pe_ttm": null,
                "pe_mrq": null,
                "pb_mrq": null,
                "ps_ttm": null,
                "pcf_ttm": null
            }]
        }))
        .unwrap();

        assert_eq!(valuations.timestamp, None);
        assert_eq!(valuations.item[0].name, None);
    }

    #[test]
    fn valuation_decimals_preserve_digits_beyond_binary_float_precision() {
        let data: ValuationsData = serde_json::from_str(
            r#"{
                "timestamp": 1,
                "total": 1,
                "item": [{
                    "thscode": "600519.SH",
                    "ticker": "600519",
                    "name": "贵州茅台",
                    "pe_ttm": 123456789012345.123456,
                    "pe_mrq": -0.000000000000000000123456,
                    "pb_mrq": null,
                    "ps_ttm": null,
                    "pcf_ttm": null
                }]
            }"#,
        )
        .unwrap();

        assert_eq!(
            data.item[0].pe_ttm.as_ref().unwrap().to_string(),
            "123456789012345.123456"
        );
        assert_eq!(
            data.item[0].pe_mrq.as_ref().unwrap().to_string(),
            "-0.000000000000000000123456"
        );
    }
}
