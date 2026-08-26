use serde::Deserialize;

use crate::{
    Adjustment, CompanyId, Cursor, DailyInterval, FundCategoryCode, HolderRecordScope, ManagerId,
    PortfolioAssetType, ReportType, Thscode, UnixMillis,
};

use super::{PriceBarItem, TimestampedItems};

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
    pub standard_rate: Option<String>,
    pub discounted_rate: Option<String>,
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
    pub fund_type: FundCategoryCode,
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

/// 按类别和区间对齐的一项基金经理同类比较雷达节点。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FundManagerRadarComparison {
    pub fund_category: Option<String>,
    pub horizon: Option<String>,
    pub manager_metrics: Option<serde_json::Value>,
    pub manager_scores: Option<serde_json::Value>,
    pub peer_average_scores: Option<serde_json::Value>,
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
    pub report_date_ms: Option<UnixMillis>,
    pub stock_ratio_pct: Option<f64>,
    pub bond_ratio_pct: Option<f64>,
    pub deposit_ratio_pct: Option<f64>,
    pub other_ratio_pct: Option<f64>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        FundAssetAllocationData, FundDiagnosticsData, FundHoldersData, FundManagerDetailData,
        FundMarketHistoricalData, FundMarketSnapshotData, FundNavData, FundPortfolioHistoryData,
        FundProfileData,
    };

    #[test]
    fn profile_preserves_unit_bearing_rate_text() {
        let profile: FundProfileData = serde_json::from_value(json!({
            "timestamp": 1_i64,
            "item": [{
                "thscode": "025480.OF",
                "ticker": "025480",
                "rate_info": [
                    {
                        "rate_type": "purchase",
                        "standard_rate": "1.20%",
                        "discounted_rate": "0.12%"
                    },
                    {
                        "rate_type": "purchase",
                        "standard_rate": "500元/笔",
                        "discounted_rate": "500元/笔"
                    }
                ]
            }]
        }))
        .unwrap();

        assert_eq!(
            profile.item[0].rate_info[0].discounted_rate.as_deref(),
            Some("0.12%")
        );
        assert_eq!(
            profile.item[0].rate_info[1].standard_rate.as_deref(),
            Some("500元/笔")
        );
    }

    #[test]
    fn diagnostics_preserves_the_observed_fund_category_code() {
        let diagnostics: FundDiagnosticsData = serde_json::from_value(json!({
            "timestamp": 1_i64,
            "item": [{
                "thscode": "025480.OF",
                "ticker": "025480",
                "fund_type": "282001003",
                "peer_code": "000300.SH",
                "dimensions": [],
                "peer_dimensions": [],
                "probabilities": [],
                "ranges": [],
                "resilience": [],
                "peer_resilience": []
            }]
        }))
        .unwrap();

        assert_eq!(diagnostics.item[0].fund_type.as_str(), "282001003");
    }

    #[test]
    fn asset_allocation_preserves_a_missing_report_date() {
        let allocation: FundAssetAllocationData = serde_json::from_value(json!({
            "timestamp": 1_i64,
            "item": [{
                "report_date_ms": null,
                "stock_ratio_pct": 90.34,
                "bond_ratio_pct": 0,
                "deposit_ratio_pct": 9.25,
                "other_ratio_pct": 0.4
            }]
        }))
        .unwrap();

        assert_eq!(allocation.item[0].report_date_ms, None);
    }

    #[test]
    fn historical_response_preserves_the_documented_adjustment_marker() {
        let data: FundMarketHistoricalData = serde_json::from_value(json!({
            "timestamp": 1_716_105_600_000_i64,
            "thscode": "510300.SH",
            "interval": "1d",
            "adjust": null,
            "item": []
        }))
        .unwrap();

        assert_eq!(data.adjust, None);
    }

    #[test]
    fn nav_date_is_a_validated_unix_millisecond_value() {
        let data: FundNavData = serde_json::from_value(json!({
            "timestamp": 1_784_131_200_000_i64,
            "item": [{
                "nav_date": 1_752_595_200_000_i64,
                "unit_nav": 4.0713
            }]
        }))
        .unwrap();

        assert_eq!(data.item[0].nav_date.get(), 1_752_595_200_000);
        assert_eq!(data.item[0].adj_nav, None);
    }

    #[test]
    fn finite_response_states_reject_unknown_wire_values() {
        let holders = json!({
            "timestamp": 1_i64,
            "item": [{"merge_scope": "all", "report_date_ms": 1_i64}]
        });
        assert!(serde_json::from_value::<FundHoldersData>(holders).is_err());

        let portfolio = json!({
            "timestamp": 1_i64,
            "item": [{
                "asset_type": "crypto",
                "report_type": "quarter",
                "end_date_ms": 1_i64
            }]
        });
        assert!(serde_json::from_value::<FundPortfolioHistoryData>(portfolio).is_err());

        let historical = json!({
            "timestamp": 1_i64,
            "thscode": "510300.SH",
            "interval": "1h",
            "adjust": null,
            "item": []
        });
        assert!(serde_json::from_value::<FundMarketHistoricalData>(historical).is_err());
    }

    #[test]
    fn documented_nullable_snapshot_metadata_is_preserved() {
        let snapshot: FundMarketSnapshotData = serde_json::from_value(json!({
            "timestamp": null,
            "item": []
        }))
        .unwrap();

        assert_eq!(snapshot.timestamp, None);
    }

    #[test]
    fn manager_detail_requires_the_documented_radar_collection() {
        let without_radar = json!({
            "timestamp": 1_i64,
            "item": [{"manager_id": "manager-1", "manager_name": "测试经理"}]
        });

        assert!(serde_json::from_value::<FundManagerDetailData>(without_radar).is_err());

        let detail: FundManagerDetailData = serde_json::from_value(json!({
            "timestamp": 1_i64,
            "item": [{
                "manager_id": "manager-1",
                "manager_name": "测试经理",
                "radar_comparison": [{
                    "fund_category": "equity",
                    "horizon": "year",
                    "manager_metrics": {"annual_return_pct": 8.6},
                    "manager_scores": {"annual_return": 80},
                    "peer_average_scores": {"annual_return": 50}
                }]
            }]
        }))
        .unwrap();

        let radar = &detail.item[0].radar_comparison[0];
        assert_eq!(radar.fund_category.as_deref(), Some("equity"));
        assert_eq!(radar.horizon.as_deref(), Some("year"));
        assert_eq!(
            radar.manager_scores.as_ref().unwrap()["annual_return"],
            json!(80)
        );
    }

    #[test]
    fn manager_detail_preserves_incomplete_radar_placeholders() {
        let detail: FundManagerDetailData = serde_json::from_value(json!({
            "timestamp": 1_i64,
            "item": [{
                "manager_id": "H002417139",
                "manager_name": "测试经理",
                "radar_comparison": [{}]
            }]
        }))
        .unwrap();

        let radar = &detail.item[0].radar_comparison[0];
        assert_eq!(radar.fund_category, None);
        assert_eq!(radar.horizon, None);
        assert_eq!(radar.manager_metrics, None);
        assert_eq!(radar.manager_scores, None);
        assert_eq!(radar.peer_average_scores, None);
    }
}
