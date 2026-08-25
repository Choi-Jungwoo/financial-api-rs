use std::collections::BTreeSet;

use financial_api::SUPPORTED_ENDPOINTS;

#[test]
fn catalog_matches_every_available_rest_endpoint_in_llms_full() {
    let expected = BTreeSet::from([
        "/api/a-share-index/catalog/ths-index-list",
        "/api/a-share-index/constituents/ths-stock-list",
        "/api/a-share-index/prices/historical",
        "/api/a-share-index/prices/snapshot",
        "/api/a-share/auction/short-term-benchmark",
        "/api/a-share/auction/snapshot",
        "/api/a-share/calendar/trading-days",
        "/api/a-share/corporate-actions/adjustment-factors",
        "/api/a-share/financials/balance-sheets",
        "/api/a-share/financials/cash-flow-statements",
        "/api/a-share/financials/income-statements",
        "/api/a-share/financials/indicators",
        "/api/a-share/prices/historical",
        "/api/a-share/prices/snapshot",
        "/api/a-share/special-data/anomaly-analysis-list",
        "/api/a-share/special-data/anomaly-analysis-stock",
        "/api/a-share/special-data/dragon-tiger-list",
        "/api/a-share/special-data/hot-stock-list",
        "/api/a-share/special-data/hot-stock-list-history",
        "/api/a-share/special-data/hot-stock-rank-trend",
        "/api/a-share/special-data/limit-break-pool",
        "/api/a-share/special-data/limit-down-pool",
        "/api/a-share/special-data/limit-up-ladder",
        "/api/a-share/special-data/limit-up-pool",
        "/api/a-share/special-data/skyrocket-list",
        "/api/a-share/valuations/snapshot",
        "/api/dump/market-dumps/adjustment-factors/download-url",
        "/api/dump/market-dumps/daily-k-10d/download-url",
        "/api/dump/market-dumps/daily-k/download-url",
        "/api/fund/companies/detail",
        "/api/fund/corporate-actions/dividends",
        "/api/fund/diagnostics/detail",
        "/api/fund/financials/balance-sheets",
        "/api/fund/financials/income-statements",
        "/api/fund/financials/indicators",
        "/api/fund/holders/detail",
        "/api/fund/holders/top",
        "/api/fund/managers/detail",
        "/api/fund/managers/experience",
        "/api/fund/managers/investment-style",
        "/api/fund/managers/performance",
        "/api/fund/market/historical",
        "/api/fund/market/snapshot",
        "/api/fund/news/article-list",
        "/api/fund/offerings/list",
        "/api/fund/performance/drawdowns",
        "/api/fund/performance/indicators-historical",
        "/api/fund/performance/nav",
        "/api/fund/performance/returns",
        "/api/fund/portfolio/asset-allocation",
        "/api/fund/portfolio/bond-history",
        "/api/fund/portfolio/bond-report-dates",
        "/api/fund/portfolio/holdings",
        "/api/fund/portfolio/industry-allocation",
        "/api/fund/portfolio/stock-history",
        "/api/fund/portfolio/stock-report-dates",
        "/api/fund/profile/detail",
        "/api/meta/tickers/list",
        "/api/meta/tickers/search",
    ]);

    let actual = SUPPORTED_ENDPOINTS
        .iter()
        .map(|endpoint| endpoint.path)
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
    assert_eq!(SUPPORTED_ENDPOINTS.len(), expected.len());
}
