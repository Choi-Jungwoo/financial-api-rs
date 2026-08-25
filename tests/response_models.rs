use financial_api::{
    FinancialIndicatorsData, FundHoldersData, FundManagerDetailData, FundMarketHistoricalData,
    FundMarketSnapshotData, FundNavData, FundPortfolioHistoryData, HotStockData,
    IncomeStatementsData, LadderData, TradingDaysData, ValuationsData,
};
use serde_json::json;

#[test]
fn fund_historical_response_preserves_the_documented_adjustment_marker() {
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
fn fund_nav_date_is_a_validated_unix_millisecond_value() {
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
fn documented_nullable_snapshot_metadata_is_preserved() {
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

    let fund_snapshot: FundMarketSnapshotData = serde_json::from_value(json!({
        "timestamp": null,
        "item": []
    }))
    .unwrap();
    assert_eq!(fund_snapshot.timestamp, None);
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

#[test]
fn fund_manager_detail_requires_the_documented_radar_collection() {
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
    assert_eq!(radar.fund_category, "equity");
    assert_eq!(radar.horizon, "year");
    assert_eq!(radar.manager_scores["annual_return"], json!(80));
}
