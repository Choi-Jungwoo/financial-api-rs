use serde_json::json;

use crate::{
    AuctionSnapshotData, FinancialIndicatorsData, HotStockData, IncomeStatementsData, LadderData,
    TradingDaysData, ValuationsData,
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
