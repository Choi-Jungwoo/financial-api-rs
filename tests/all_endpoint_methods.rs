use std::collections::BTreeSet;

use financial_api::{
    Adjustment, AnomalyTag, ApiKey, AuctionStage, BusinessErrorKind, Client, DragonTigerBoard,
    Error, FinancialPeriod, FinancialRange, FundNavType, FundRange, FundType, HolderMergeScope,
    HotListPeriod, IndexTag, LimitBreakSortField, LimitDownSortField, LimitUpSortField,
    ManagerPerformanceRange, NaturalDate, OfferingStatus, Page, PortfolioAssetType,
    PriceSnapshotSelection, SUPPORTED_ENDPOINTS, SortDirection, TickerListRequest,
    TickerSearchRequest,
};
use serde_json::{Value, json};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

const TIMESTAMP: i64 = 1_716_105_600_000;
const END_TIMESTAMP: i64 = 1_716_192_000_000;

#[derive(Debug, Clone, Copy)]
struct SuccessfulEndpoint;

impl Respond for SuccessfulEndpoint {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        let data = minimal_data(request.url.path());
        ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "message": "success",
            "request_id": "all-endpoints",
            "data": data
        }))
    }
}

#[derive(Debug, Clone, Copy)]
struct DeniedEndpoint;

impl Respond for DeniedEndpoint {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "code": 2003,
            "message": "capability denied",
            "request_id": "all-endpoints-denied",
            "data": null
        }))
    }
}

fn minimal_data(path: &str) -> Value {
    match path {
        "/api/meta/tickers/search" | "/api/meta/tickers/list" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "name": "贵州茅台",
                "exchange": "SH",
                "asset_type": "a-share",
                "currency": "CNY"
            }]
        }),
        "/api/a-share/prices/snapshot" => price_snapshot("600519.SH", "600519"),
        "/api/a-share-index/prices/snapshot" => price_snapshot("000300.SH", "000300"),
        "/api/a-share/prices/historical" => json!({
            "timestamp": TIMESTAMP,
            "item": [price_bar()]
        }),
        "/api/a-share-index/prices/historical" => {
            json!({"timestamp": TIMESTAMP, "adjust": null, "item": [price_bar()]})
        }
        "/api/a-share-index/catalog/ths-index-list" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"thscode": "886042.TI", "name": "白酒概念"}]
        }),
        "/api/a-share-index/constituents/ths-stock-list" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"thscode": "600519.SH", "ticker": "600519", "name": "贵州茅台"}]
        }),
        "/api/a-share/corporate-actions/adjustment-factors" => json!({
            "thscode": "600519.SH",
            "ticker": "600519",
            "item": [{
                "ticker": "600519",
                "ex_date_ms": TIMESTAMP,
                "dividend_per_share": 1.0,
                "per_share_bonus": 0.0
            }]
        }),
        "/api/a-share/financials/income-statements" => json!({
            "timestamp": TIMESTAMP,
            "item": [financial_statement_meta()]
        }),
        "/api/a-share/financials/balance-sheets" => json!({
            "timestamp": TIMESTAMP,
            "item": [financial_statement_meta()]
        }),
        "/api/a-share/financials/cash-flow-statements" => json!({
            "timestamp": TIMESTAMP,
            "item": [financial_statement_meta()]
        }),
        "/api/a-share/financials/indicators" => json!({
            "thscode": "600519.SH",
            "report": "2025-4",
            "abilities": [{
                "ability": "growth",
                "indicators": [{"index_id": "total_assets_growth_ratio", "value": null}]
            }]
        }),
        "/api/a-share/calendar/trading-days" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"date_ms": TIMESTAMP, "date": "20240519"}]
        }),
        "/api/a-share/auction/snapshot" => json!({
            "timestamp": TIMESTAMP,
            "auction_phase": "final",
            "data_status": "ready",
            "total": 1,
            "item": [{"thscode": "600519.SH", "ticker": "600519", "name": "贵州茅台"}]
        }),
        "/api/a-share/auction/short-term-benchmark" => json!({
            "timestamp": TIMESTAMP,
            "date": "2026-08-25",
            "date_ms": TIMESTAMP,
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "name": "贵州茅台",
                "auction_pct": null,
                "tags": []
            }]
        }),
        "/api/a-share/valuations/snapshot" => {
            json!({
                "timestamp": TIMESTAMP,
                "total": 1,
                "item": [{"thscode": "600519.SH", "ticker": "600519", "name": "贵州茅台"}]
            })
        }
        "/api/a-share/special-data/anomaly-analysis-list"
        | "/api/a-share/special-data/anomaly-analysis-stock" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "stock_name": "贵州茅台",
                "analysis_content": "测试异动",
                "keyword_list": [],
                "thscode": "600519.SH",
                "tag_name": "涨停"
            }]
        }),
        "/api/a-share/special-data/hot-stock-list" | "/api/a-share/special-data/skyrocket-list" => {
            json!({
                "timestamp": TIMESTAMP,
                "item": [{
                    "thscode": "600519.SH",
                    "ticker": "600519",
                    "name": "贵州茅台",
                    "rank": 1,
                    "heat": "100",
                    "rank_trend": "up"
                }]
            })
        }
        "/api/a-share/special-data/hot-stock-list-history" => {
            json!({
                "date": "2026-08-25",
                "date_ms": TIMESTAMP,
                "item": [{
                    "thscode": "600519.SH",
                    "ticker": "600519",
                    "name": "贵州茅台",
                    "rank": 1
                }]
            })
        }
        "/api/a-share/special-data/hot-stock-rank-trend" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "date": "2026-08-25",
                "date_ms": TIMESTAMP,
                "rank": 1
            }]
        }),
        "/api/a-share/special-data/dragon-tiger-list" => json!({
            "timestamp": TIMESTAMP,
            "board_type": "all",
            "trade_date": "2026-08-25",
            "count": 1,
            "stock_count": 1,
            "stock_items": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "name": "贵州茅台"
            }],
            "hot_money_items": []
        }),
        "/api/a-share/special-data/limit-up-pool" => json!({
            "timestamp": TIMESTAMP,
            "pagination": {"total": 1, "pages": 1, "size": 50, "page": 1},
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "name": "贵州茅台",
                "is_st": false,
                "is_new": false,
                "last_price": 1.0,
                "price_change_ratio_pct": 10.0,
                "limit_up_time": "09:30",
                "continue_day_text": "首板",
                "continue_day_cnt": 1,
                "seal_money": 1.0,
                "max_seal_money": 1.0
            }]
        }),
        "/api/a-share/special-data/limit-down-pool" => json!({
            "timestamp": TIMESTAMP,
            "pagination": {"total": 1, "pages": 1, "size": 50, "page": 1},
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "name": "贵州茅台",
                "last_price": 1.0,
                "price_change_ratio_pct": -10.0,
                "first_limit_time": "09:30",
                "last_limit_time": "14:59",
                "turnover_ratio_pct": 1.0
            }]
        }),
        "/api/a-share/special-data/limit-break-pool" => json!({
            "timestamp": TIMESTAMP,
            "pagination": {"total": 1, "pages": 1, "size": 50, "page": 1},
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "name": "贵州茅台",
                "last_price": 1.0,
                "price_change_ratio_pct": 9.0,
                "open_times": 1,
                "turnover_ratio_pct": 1.0,
                "turnover": 1.0
            }]
        }),
        "/api/a-share/special-data/limit-up-ladder" => json!({
            "timestamp": TIMESTAMP,
            "window": {
                "length": 30,
                "date_list": [],
                "board_caps": {
                    "two_board": 0,
                    "three_board": 0,
                    "four_board": 0,
                    "five_board": 0,
                    "six_board": 0,
                    "seven_over": 0
                }
            },
            "item": [{
                "date": "20260825",
                "boards": {
                    "two_board": [{
                        "thscode": "600519.SH",
                        "ticker": "600519",
                        "name": "贵州茅台",
                        "board_num": 2,
                        "sign_level": 1
                    }],
                    "three_board": [],
                    "four_board": [],
                    "five_board": [],
                    "six_board": [],
                    "seven_over": []
                }
            }]
        }),
        "/api/fund/companies/detail" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "company_id": "company-1",
                "company_name": "测试基金公司",
                "company_type": "公募",
                "established_date_ms": TIMESTAMP,
                "fund_count": 1,
                "scale": 1.0
            }]
        }),
        "/api/fund/corporate-actions/dividends" => {
            json!({
                "timestamp": TIMESTAMP,
                "item": [{"per_ten_cash_before_tax": 1.0}]
            })
        }
        "/api/fund/diagnostics/detail" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "thscode": "025480.OF",
                "ticker": "025480",
                "fund_type": "otc",
                "peer_code": "peer",
                "dimensions": {},
                "peer_dimensions": {},
                "probabilities": {},
                "ranges": {},
                "resilience": {},
                "peer_resilience": {}
            }]
        }),
        "/api/fund/financials/indicators" => {
            json!({"timestamp": TIMESTAMP, "item": [{"current_profit": 1.0}]})
        }
        "/api/fund/financials/income-statements" => {
            json!({"timestamp": TIMESTAMP, "item": [{"net_profit": 1.0}]})
        }
        "/api/fund/financials/balance-sheets" => {
            json!({"timestamp": TIMESTAMP, "item": [{"total_assets": 1.0}]})
        }
        "/api/fund/holders/detail" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"merge_scope": "merged", "report_date_ms": TIMESTAMP}]
        }),
        "/api/fund/holders/top" => {
            json!({
                "timestamp": TIMESTAMP,
                "limit": 10,
                "item": [{"holder_name": "测试持有人"}]
            })
        }
        "/api/fund/portfolio/holdings" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "thscode": "600519.SH",
                "ticker": "600519",
                "asset_type": "stock"
            }]
        }),
        "/api/fund/managers/investment-style" => {
            json!({
                "timestamp": TIMESTAMP,
                "item": [{"investment_idea": "长期投资"}]
            })
        }
        "/api/fund/managers/performance" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"date_ms": TIMESTAMP, "manager_return_pct": 1.0}]
        }),
        "/api/fund/managers/experience" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"awards": {}, "heavy_assets": {}, "investment_history": {}}]
        }),
        "/api/fund/managers/detail" => json!({
            "timestamp": TIMESTAMP,
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
        }),
        "/api/fund/market/snapshot" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "thscode": "510300.SH",
                "ticker": "510300",
                "last_price": 1.0,
                "open_price": 1.0,
                "high_price": 1.1,
                "low_price": 0.9,
                "prev_price": 0.9,
                "price_change_ratio_pct": 1.0,
                "price_change": 0.1,
                "volume": 10.0,
                "turnover": 10.0
            }]
        }),
        "/api/fund/market/historical" => json!({
            "timestamp": TIMESTAMP,
            "thscode": "510300.SH",
            "interval": "1d",
            "adjust": null,
            "item": [price_bar()]
        }),
        "/api/fund/news/article-list" => json!({
            "timestamp": TIMESTAMP,
            "limit": 20,
            "offset": null,
            "has_more": false,
            "item": [{"id": "article-1", "title": "测试资讯"}]
        }),
        "/api/fund/offerings/list" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "thscode": "025480.OF",
                "ticker": "025480",
                "subscription_start_ms": TIMESTAMP,
                "subscription_end_ms": TIMESTAMP
            }]
        }),
        "/api/fund/performance/nav" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"nav_date": TIMESTAMP, "unit_nav": 1.0}]
        }),
        "/api/fund/performance/returns" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"return_month": 1.0}]
        }),
        "/api/fund/performance/indicators-historical" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"date_ms": TIMESTAMP}]
        }),
        "/api/fund/performance/drawdowns" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"thscode": "025480.OF", "ticker": "025480"}]
        }),
        "/api/fund/portfolio/stock-history" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "asset_type": "stock",
                "report_type": "quarter",
                "end_date_ms": TIMESTAMP
            }]
        }),
        "/api/fund/portfolio/bond-history" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "asset_type": "bond",
                "report_type": "quarter",
                "end_date_ms": TIMESTAMP
            }]
        }),
        "/api/fund/portfolio/stock-report-dates" | "/api/fund/portfolio/bond-report-dates" => {
            json!({
                "timestamp": TIMESTAMP,
                "item": [{
                    "report_type": "quarter",
                    "report_type_name": "季度",
                    "start_date_ms": TIMESTAMP,
                    "end_date_ms": TIMESTAMP
                }]
            })
        }
        "/api/fund/portfolio/asset-allocation" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"report_date_ms": TIMESTAMP, "stock_ratio_pct": 1.0}]
        }),
        "/api/fund/portfolio/industry-allocation" => json!({
            "timestamp": TIMESTAMP,
            "item": [{"industry_name": "金融"}]
        }),
        "/api/fund/profile/detail" => json!({
            "timestamp": TIMESTAMP,
            "item": [{
                "thscode": "025480.OF",
                "ticker": "025480",
                "fund_name": "测试基金"
            }]
        }),
        "/api/dump/market-dumps/daily-k/download-url"
        | "/api/dump/market-dumps/daily-k-10d/download-url"
        | "/api/dump/market-dumps/adjustment-factors/download-url" => json!({
            "presigned_url": "https://storage.example.test/dump.parquet?signature=redacted",
            "presigned_url_expires_at": "2026-08-25T08:05:00Z"
        }),
        _ => panic!("missing typed response fixture for {path}"),
    }
}

fn price_snapshot(thscode: &str, ticker: &str) -> Value {
    json!({
        "timestamp": null,
        "total": 1,
        "item": [{
            "thscode": thscode,
            "ticker": ticker,
            "last_price": 1.0,
            "price_change": 0.1,
            "price_change_ratio_pct": 1.0,
            "open_price": 1.0,
            "high_price": 1.1,
            "low_price": 0.9,
            "prev_price": 0.9,
            "volume": 10.0,
            "turnover": 10.0
        }]
    })
}

fn price_bar() -> Value {
    json!({
        "date_ms": TIMESTAMP,
        "open_price": 1.0,
        "high_price": 1.1,
        "low_price": 0.9,
        "close_price": 1.0,
        "volume": 10.0,
        "turnover": 10.0
    })
}

fn financial_statement_meta() -> Value {
    json!({
        "thscode": "600519.SH",
        "ticker": "600519",
        "period": "annual",
        "fiscal_year": 2025,
        "fiscal_period": "FY",
        "report_date_ms": TIMESTAMP,
        "period_end_ms": TIMESTAMP,
        "currency": "CNY"
    })
}

fn client(server: &MockServer) -> Client {
    Client::builder(ApiKey::new("test-api-key").unwrap())
        .base_url(server.uri())
        .reference_date(NaturalDate::parse("2026-08-25").unwrap())
        .build()
        .unwrap()
}

fn expected_query(path: &str) -> Option<&'static str> {
    match path {
        "/api/meta/tickers/search" => Some("q=600519&limit=10"),
        "/api/meta/tickers/list" => Some("limit=1000&offset=0"),
        "/api/a-share-index/catalog/ths-index-list" => Some("tag=cn_concept"),
        "/api/a-share-index/constituents/ths-stock-list" => Some("thscode=000300.SH"),
        "/api/a-share-index/prices/snapshot" => Some("thscodes=000300.SH"),
        "/api/a-share-index/prices/historical" => {
            Some("thscode=000300.SH&interval=1d&start=1716105600000&end=1716192000000")
        }
        "/api/a-share/prices/snapshot" => Some("thscodes=600519.SH"),
        "/api/a-share/prices/historical" => Some(
            "thscode=600519.SH&interval=1d&start=1716105600000&end=1716192000000&adjust=none&offset=0",
        ),
        "/api/a-share/corporate-actions/adjustment-factors" => {
            Some("thscode=600519.SH&from=2026-08-25")
        }
        "/api/a-share/financials/income-statements"
        | "/api/a-share/financials/balance-sheets"
        | "/api/a-share/financials/cash-flow-statements" => {
            Some("thscode=600519.SH&period=annual&limit=1")
        }
        "/api/a-share/financials/indicators" => Some("thscode=600519.SH&report=2025-4"),
        "/api/a-share/calendar/trading-days" => None,
        "/api/a-share/auction/snapshot" => Some("thscodes=600519.SH&stage=final"),
        "/api/a-share/auction/short-term-benchmark" => Some("date=2026-08-25"),
        "/api/a-share/valuations/snapshot" => Some("thscodes=600519.SH"),
        "/api/a-share/special-data/anomaly-analysis-list" => Some("tag_codes=LIMIT_UP"),
        "/api/a-share/special-data/anomaly-analysis-stock" => Some("thscodes=600519.SH"),
        "/api/a-share/special-data/dragon-tiger-list" => Some("board_type=all&date=2026-08-25"),
        "/api/a-share/special-data/hot-stock-list" => Some("period=day"),
        "/api/a-share/special-data/hot-stock-list-history" => Some("date=2026-08-25"),
        "/api/a-share/special-data/hot-stock-rank-trend" => {
            Some("thscode=600519.SH&start_date=2026-08-25&end_date=2026-08-25")
        }
        "/api/a-share/special-data/limit-up-pool"
        | "/api/a-share/special-data/limit-down-pool"
        | "/api/a-share/special-data/limit-break-pool" => {
            Some("page=1&size=50&sort_field=last_price&sort_dir=desc")
        }
        "/api/a-share/special-data/limit-up-ladder" => None,
        "/api/a-share/special-data/skyrocket-list" => Some("period=hour"),
        "/api/fund/companies/detail" => Some("company_id=company-1"),
        "/api/fund/corporate-actions/dividends"
        | "/api/fund/diagnostics/detail"
        | "/api/fund/financials/indicators"
        | "/api/fund/financials/income-statements"
        | "/api/fund/financials/balance-sheets"
        | "/api/fund/portfolio/holdings"
        | "/api/fund/performance/returns"
        | "/api/fund/performance/drawdowns"
        | "/api/fund/portfolio/asset-allocation"
        | "/api/fund/portfolio/industry-allocation"
        | "/api/fund/profile/detail" => Some("fund_type=otc&thscode=025480.OF"),
        "/api/fund/holders/detail" => Some("fund_type=otc&thscode=025480.OF&merge_scope=all"),
        "/api/fund/holders/top" => Some("fund_type=otc&thscode=025480.OF&limit=10"),
        "/api/fund/managers/investment-style"
        | "/api/fund/managers/experience"
        | "/api/fund/managers/detail" => Some("manager_id=manager-1"),
        "/api/fund/managers/performance" => Some("manager_id=manager-1&range=month"),
        "/api/fund/market/snapshot" => Some("thscode=510300.SH"),
        "/api/fund/market/historical" => {
            Some("thscode=510300.SH&interval=1d&start=1716105600000&end=1716192000000")
        }
        "/api/fund/news/article-list" => Some("fund_type=otc&thscode=025480.OF&limit=20"),
        "/api/fund/offerings/list" => Some("subscribe=active"),
        "/api/fund/performance/nav" => {
            Some("fund_type=otc&thscode=025480.OF&range=month&nav_type=unit")
        }
        "/api/fund/performance/indicators-historical" => {
            Some("fund_type=otc&thscode=025480.OF&start=1716105600000&end=1716192000000")
        }
        "/api/fund/portfolio/stock-history" | "/api/fund/portfolio/bond-history" => {
            Some("fund_type=otc&thscode=025480.OF&report_type=quarter&end_date=2026-08-25")
        }
        "/api/fund/portfolio/stock-report-dates" | "/api/fund/portfolio/bond-report-dates" => {
            Some("fund_type=otc&thscode=025480.OF&report_type=quarter")
        }
        "/api/dump/market-dumps/daily-k/download-url"
        | "/api/dump/market-dumps/daily-k-10d/download-url"
        | "/api/dump/market-dumps/adjustment-factors/download-url" => None,
        _ => panic!("missing query contract for {path}"),
    }
}

#[tokio::test]
async fn every_catalogued_endpoint_has_an_executable_client_method() {
    let server = MockServer::start().await;
    Mock::given(wiremock::matchers::method("GET"))
        .respond_with(SuccessfulEndpoint)
        .mount(&server)
        .await;
    let denied_server = MockServer::start().await;
    Mock::given(wiremock::matchers::method("GET"))
        .respond_with(DeniedEndpoint)
        .mount(&denied_server)
        .await;

    let denied_client = client(&denied_server);
    let client = client(&server);
    let date_text = "2026-08-25";
    let date = NaturalDate::parse(date_text).unwrap();

    macro_rules! assert_success {
        ($future:expr, $data:ident => $predicate:expr) => {{
            let response = $future.await.unwrap();
            let $data = response.data();
            assert!(
                $predicate,
                "typed field assertion failed for {}",
                stringify!($future)
            );
        }};
    }

    macro_rules! assert_denied {
        ($future:expr, $_data:ident => $_predicate:expr) => {{
            let Error::Business(error) = $future.await.unwrap_err() else {
                panic!("expected business error for {}", stringify!($future));
            };
            assert_eq!(error.kind(), BusinessErrorKind::PermissionDenied);
        }};
    }

    macro_rules! for_each_endpoint {
        ($apply:ident, $api:ident) => {
            $apply!(
                $api.tickers_search(&TickerSearchRequest::new("600519").unwrap()),
                data => data.item[0].name == "贵州茅台"
            );
            $apply!(
                $api.tickers_list(&TickerListRequest::new()),
                data => data.item[0].asset_type.as_str() == "a-share"
            );
            $apply!(
                $api.index_catalog_ths_index_list(IndexTag::Concept),
                data => data.item[0].thscode.as_str() == "886042.TI"
            );
            $apply!(
                $api.index_constituents_ths_stock_list("000300.SH"),
                data => data.item[0].name == "贵州茅台"
            );
            $apply!(
                $api.index_prices_snapshot(&["000300.SH"]),
                data => data.item[0].thscode.as_str() == "000300.SH"
            );
            $apply!(
                $api.index_prices_historical("000300.SH", TIMESTAMP, END_TIMESTAMP),
                data => data.item[0].close_price == 1.0
            );
            $apply!(
                $api.prices_snapshot(
                    &PriceSnapshotSelection::targets(["600519.SH"]).unwrap()
                ),
                data => data.total == 1
            );
            $apply!(
                $api.prices_historical(
                    "600519.SH",
                    TIMESTAMP,
                    END_TIMESTAMP,
                    Adjustment::None,
                    0
                ),
                data => data.item[0].close_price == 1.0
            );
            $apply!(
                $api.corp_actions_adjustment_factors("600519.SH", Some(date_text), None),
                data => data.item[0].ex_date_ms.get() == TIMESTAMP
            );
            $apply!(
                $api.financials_income_statements(
                    "600519.SH",
                    FinancialPeriod::Annual,
                    FinancialRange::recent(1).unwrap()
                ),
                data => data.item[0].meta.fiscal_year == 2025
            );
            $apply!(
                $api.financials_balance_sheets(
                    "600519.SH",
                    FinancialPeriod::Annual,
                    FinancialRange::recent(1).unwrap()
                ),
                data => data.item[0].meta.fiscal_year == 2025
            );
            $apply!(
                $api.financials_cash_flow_statements(
                    "600519.SH",
                    FinancialPeriod::Annual,
                    FinancialRange::recent(1).unwrap()
                ),
                data => data.item[0].meta.fiscal_year == 2025
            );
            $apply!(
                $api.financials_indicators("600519.SH", "2025-4"),
                data => data.abilities[0].indicators[0].value.is_none()
            );
            $apply!(
                $api.calendar_trading_days(),
                data => data.item[0].date.to_string() == "20240519"
            );
            $apply!(
                $api.a_share_auction_snapshot(
                    &["600519.SH"],
                    AuctionStage::Final
                ),
                data => data.item[0].name == "贵州茅台"
            );
            $apply!(
                $api.a_share_auction_short_term_benchmark(Some(date_text)),
                data => data.item[0].tags.is_empty()
            );
            $apply!(
                $api.a_share_valuations_snapshot(&["600519.SH"]),
                data => data.item[0].name.as_deref() == Some("贵州茅台")
            );
            $apply!(
                $api.special_data_anomaly_analysis_list(&[AnomalyTag::LimitUp]),
                data => data.item[0].tag_name == "涨停"
            );
            $apply!(
                $api.special_data_anomaly_analysis_stock(&["600519.SH"]),
                data => data.item[0].analysis_content == "测试异动"
            );
            $apply!(
                $api.special_data_dragon_tiger_list(
                    DragonTigerBoard::All,
                    Some(date_text)
                ),
                data => data.stock_items[0].name == "贵州茅台"
            );
            $apply!(
                $api.special_data_hot_stock_list(HotListPeriod::Day),
                data => data.item[0].rank_trend.as_str() == "up"
            );
            $apply!(
                $api.special_data_hot_stock_list_history(date_text),
                data => data.item[0].rank == 1
            );
            $apply!(
                $api.special_data_hot_stock_rank_trend(
                    "600519.SH",
                    date_text,
                    date_text
                ),
                data => data.item[0].date == date
            );
            $apply!(
                $api.special_data_limit_up_pool(
                    None,
                    Page::default(),
                    LimitUpSortField::LastPrice,
                    SortDirection::Descending
                ),
                data => data.item[0].continue_day_cnt == 1
            );
            $apply!(
                $api.special_data_limit_down_pool(
                    None,
                    Page::default(),
                    LimitDownSortField::LastPrice,
                    SortDirection::Descending
                ),
                data => data.item[0].first_limit_time == "09:30"
            );
            $apply!(
                $api.special_data_limit_break_pool(
                    None,
                    Page::default(),
                    LimitBreakSortField::LastPrice,
                    SortDirection::Descending
                ),
                data => data.item[0].open_times == 1
            );
            $apply!(
                $api.special_data_limit_up_ladder(),
                data => data.item[0].date.to_string() == "2026-08-25"
            );
            $apply!(
                $api.special_data_skyrocket_list(HotListPeriod::Hour),
                data => data.item[0].rank == 1
            );
            $apply!(
                $api.fund_companies_detail("company-1"),
                data => data.item[0].company_name == "测试基金公司"
            );
            $apply!(
                $api.fund_corporate_actions_dividends(FundType::Otc, "025480.OF"),
                data => data.item[0].per_ten_cash_before_tax == Some(1.0)
            );
            $apply!(
                $api.fund_diagnostics_detail(FundType::Otc, "025480.OF"),
                data => data.item[0].peer_code == "peer"
            );
            $apply!(
                $api.fund_financials_indicators(FundType::Otc, "025480.OF"),
                data => data.item[0].current_profit == Some(1.0)
            );
            $apply!(
                $api.fund_financials_income_statements(FundType::Otc, "025480.OF"),
                data => data.item[0].net_profit == Some(1.0)
            );
            $apply!(
                $api.fund_financials_balance_sheets(FundType::Otc, "025480.OF"),
                data => data.item[0].total_assets == Some(1.0)
            );
            $apply!(
                $api.fund_holders_detail(
                    FundType::Otc,
                    "025480.OF",
                    HolderMergeScope::All
                ),
                data => data.item[0].merge_scope.as_str() == "merged"
            );
            $apply!(
                $api.fund_holders_top(FundType::Otc, "025480.OF", Some(10)),
                data => data.item[0].holder_name.as_deref() == Some("测试持有人")
            );
            $apply!(
                $api.fund_portfolio_holdings(FundType::Otc, "025480.OF"),
                data => data.item[0].asset_type == Some(PortfolioAssetType::Stock)
            );
            $apply!(
                $api.fund_managers_investment_style("manager-1"),
                data => data.item[0].investment_idea.as_deref() == Some("长期投资")
            );
            $apply!(
                $api.fund_managers_performance(
                    "manager-1",
                    ManagerPerformanceRange::Month
                ),
                data => data.item[0].manager_return_pct == Some(1.0)
            );
            $apply!(
                $api.fund_managers_experience("manager-1"),
                data => data.item[0].awards.is_object()
            );
            $apply!(
                $api.fund_managers_detail("manager-1"),
                data => data.item[0].radar_comparison[0].fund_category.as_deref() == Some("equity")
            );
            $apply!(
                $api.fund_market_snapshot("510300.SH"),
                data => data.item[0].last_price == 1.0
            );
            $apply!(
                $api.fund_market_historical(
                    "510300.SH",
                    TIMESTAMP,
                    END_TIMESTAMP
                ),
                data => data.item[0].close_price == 1.0
            );
            $apply!(
                $api.fund_news_article_list(
                    FundType::Otc,
                    "025480.OF",
                    Some(20),
                    None
                ),
                data => data.item[0].title == "测试资讯"
            );
            $apply!(
                $api.fund_offerings_list(OfferingStatus::Active),
                data => data.item[0].thscode.as_ref().is_some_and(|code| code.as_str() == "025480.OF")
            );
            $apply!(
                $api.fund_performance_nav(
                    FundType::Otc,
                    "025480.OF",
                    Some(FundRange::Month),
                    FundNavType::Unit
                ),
                data => data.item[0].unit_nav == Some(1.0)
            );
            $apply!(
                $api.fund_performance_returns(FundType::Otc, "025480.OF"),
                data => data.item[0].return_month == Some(1.0)
            );
            $apply!(
                $api.fund_performance_indicators_historical(
                    FundType::Otc,
                    "025480.OF",
                    TIMESTAMP,
                    END_TIMESTAMP
                ),
                data => data.item[0].date_ms.get() == TIMESTAMP
            );
            $apply!(
                $api.fund_performance_drawdowns(FundType::Otc, "025480.OF"),
                data => data.item[0].thscode.as_str() == "025480.OF"
            );
            $apply!(
                $api.fund_portfolio_stock_history(
                    FundType::Otc,
                    "025480.OF",
                    "quarter",
                    date_text
                ),
                data => data.item[0].asset_type == PortfolioAssetType::Stock
            );
            $apply!(
                $api.fund_portfolio_bond_history(
                    FundType::Otc,
                    "025480.OF",
                    "quarter",
                    date_text
                ),
                data => data.item[0].asset_type == PortfolioAssetType::Bond
            );
            $apply!(
                $api.fund_portfolio_stock_report_dates(
                    FundType::Otc,
                    "025480.OF",
                    Some("quarter")
                ),
                data => data.item[0].report_type_name == "季度"
            );
            $apply!(
                $api.fund_portfolio_bond_report_dates(
                    FundType::Otc,
                    "025480.OF",
                    Some("quarter")
                ),
                data => data.item[0].report_type.as_str() == "quarter"
            );
            $apply!(
                $api.fund_portfolio_asset_allocation(FundType::Otc, "025480.OF"),
                data => data.item[0].stock_ratio_pct == Some(1.0)
            );
            $apply!(
                $api.fund_portfolio_industry_allocation(FundType::Otc, "025480.OF"),
                data => data.item[0].industry_name.as_deref() == Some("金融")
            );
            $apply!(
                $api.fund_profile_detail(FundType::Otc, "025480.OF"),
                data => data.item[0].fund_name.as_deref() == Some("测试基金")
            );
            $apply!(
                $api.market_dump_daily_k(),
                data => data.presigned_url.expose().contains("dump.parquet")
            );
            $apply!(
                $api.market_dump_daily_k_10d(),
                data => data.presigned_url.expose().contains("dump.parquet")
            );
            $apply!(
                $api.market_dump_adjustment_factors(),
                data => data.presigned_url.expose().contains("dump.parquet")
            );
        };
    }

    for_each_endpoint!(assert_success, client);
    for_each_endpoint!(assert_denied, denied_client);

    let requests = server.received_requests().await.unwrap();
    let denied_requests = denied_server.received_requests().await.unwrap();
    let requested_paths = requests
        .iter()
        .map(|request| request.url.path())
        .collect::<BTreeSet<_>>();
    let denied_paths = denied_requests
        .iter()
        .map(|request| request.url.path())
        .collect::<BTreeSet<_>>();
    let supported_paths = SUPPORTED_ENDPOINTS
        .iter()
        .map(|endpoint| endpoint.path)
        .collect::<BTreeSet<_>>();

    assert_eq!(requests.len(), SUPPORTED_ENDPOINTS.len());
    assert_eq!(requested_paths, supported_paths);
    assert_eq!(denied_requests.len(), SUPPORTED_ENDPOINTS.len());
    assert_eq!(denied_paths, supported_paths);
    assert!(requests.iter().chain(&denied_requests).all(|request| {
        request
            .headers
            .get("x-api-key")
            .is_some_and(|value| value == "test-api-key")
    }));
    for request in requests.iter().chain(&denied_requests) {
        assert_eq!(
            request.url.query(),
            expected_query(request.url.path()),
            "query mismatch for {}",
            request.url.path()
        );
    }
}
