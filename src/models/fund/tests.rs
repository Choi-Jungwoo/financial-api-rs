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
