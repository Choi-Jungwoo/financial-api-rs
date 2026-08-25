/// A stable name and path for one supported REST capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndpointInfo {
    pub name: &'static str,
    pub path: &'static str,
}

macro_rules! define_endpoints {
    ($($constant:ident => ($name:literal, $path:literal)),+ $(,)?) => {
        $(pub(crate) const $constant: &str = $path;)+

        /// Every currently available REST endpoint in the upstream contract.
        pub const SUPPORTED_ENDPOINTS: &[EndpointInfo] = &[
            $(EndpointInfo { name: $name, path: $path },)+
        ];
    };
}

define_endpoints! {
    INDEX_CATALOG => ("index_catalog_ths_index_list", "/api/a-share-index/catalog/ths-index-list"),
    INDEX_CONSTITUENTS => ("index_constituents_ths_stock_list", "/api/a-share-index/constituents/ths-stock-list"),
    INDEX_HISTORICAL => ("index_prices_historical", "/api/a-share-index/prices/historical"),
    INDEX_SNAPSHOT => ("index_prices_snapshot", "/api/a-share-index/prices/snapshot"),
    AUCTION_BENCHMARK => ("a_share_auction_short_term_benchmark", "/api/a-share/auction/short-term-benchmark"),
    AUCTION_SNAPSHOT => ("a_share_auction_snapshot", "/api/a-share/auction/snapshot"),
    TRADING_DAYS => ("calendar_trading_days", "/api/a-share/calendar/trading-days"),
    ADJUSTMENT_FACTORS => ("corp_actions_adjustment_factors", "/api/a-share/corporate-actions/adjustment-factors"),
    BALANCE_SHEETS => ("financials_balance_sheets", "/api/a-share/financials/balance-sheets"),
    CASH_FLOW_STATEMENTS => ("financials_cash_flow_statements", "/api/a-share/financials/cash-flow-statements"),
    INCOME_STATEMENTS => ("financials_income_statements", "/api/a-share/financials/income-statements"),
    FINANCIAL_INDICATORS => ("financials_indicators", "/api/a-share/financials/indicators"),
    PRICES_HISTORICAL => ("prices_historical", "/api/a-share/prices/historical"),
    PRICES_SNAPSHOT => ("prices_snapshot", "/api/a-share/prices/snapshot"),
    ANOMALY_LIST => ("special_data_anomaly_analysis_list", "/api/a-share/special-data/anomaly-analysis-list"),
    ANOMALY_STOCK => ("special_data_anomaly_analysis_stock", "/api/a-share/special-data/anomaly-analysis-stock"),
    DRAGON_TIGER => ("special_data_dragon_tiger_list", "/api/a-share/special-data/dragon-tiger-list"),
    HOT_STOCK_LIST => ("special_data_hot_stock_list", "/api/a-share/special-data/hot-stock-list"),
    HOT_STOCK_HISTORY => ("special_data_hot_stock_list_history", "/api/a-share/special-data/hot-stock-list-history"),
    HOT_STOCK_TREND => ("special_data_hot_stock_rank_trend", "/api/a-share/special-data/hot-stock-rank-trend"),
    LIMIT_BREAK_POOL => ("special_data_limit_break_pool", "/api/a-share/special-data/limit-break-pool"),
    LIMIT_DOWN_POOL => ("special_data_limit_down_pool", "/api/a-share/special-data/limit-down-pool"),
    LIMIT_UP_LADDER => ("special_data_limit_up_ladder", "/api/a-share/special-data/limit-up-ladder"),
    LIMIT_UP_POOL => ("special_data_limit_up_pool", "/api/a-share/special-data/limit-up-pool"),
    SKYROCKET_LIST => ("special_data_skyrocket_list", "/api/a-share/special-data/skyrocket-list"),
    VALUATIONS_SNAPSHOT => ("a_share_valuations_snapshot", "/api/a-share/valuations/snapshot"),
    DUMP_ADJUSTMENT_FACTORS => ("market_dump_adjustment_factors", "/api/dump/market-dumps/adjustment-factors/download-url"),
    DUMP_DAILY_K_10D => ("market_dump_daily_k_10d", "/api/dump/market-dumps/daily-k-10d/download-url"),
    DUMP_DAILY_K => ("market_dump_daily_k", "/api/dump/market-dumps/daily-k/download-url"),
    FUND_COMPANY_DETAIL => ("fund_companies_detail", "/api/fund/companies/detail"),
    FUND_DIVIDENDS => ("fund_corporate_actions_dividends", "/api/fund/corporate-actions/dividends"),
    FUND_DIAGNOSTICS => ("fund_diagnostics_detail", "/api/fund/diagnostics/detail"),
    FUND_BALANCE_SHEETS => ("fund_financials_balance_sheets", "/api/fund/financials/balance-sheets"),
    FUND_INCOME_STATEMENTS => ("fund_financials_income_statements", "/api/fund/financials/income-statements"),
    FUND_INDICATORS => ("fund_financials_indicators", "/api/fund/financials/indicators"),
    FUND_HOLDERS_DETAIL => ("fund_holders_detail", "/api/fund/holders/detail"),
    FUND_HOLDERS_TOP => ("fund_holders_top", "/api/fund/holders/top"),
    FUND_MANAGER_DETAIL => ("fund_managers_detail", "/api/fund/managers/detail"),
    FUND_MANAGER_EXPERIENCE => ("fund_managers_experience", "/api/fund/managers/experience"),
    FUND_MANAGER_STYLE => ("fund_managers_investment_style", "/api/fund/managers/investment-style"),
    FUND_MANAGER_PERFORMANCE => ("fund_managers_performance", "/api/fund/managers/performance"),
    FUND_MARKET_HISTORICAL => ("fund_market_historical", "/api/fund/market/historical"),
    FUND_MARKET_SNAPSHOT => ("fund_market_snapshot", "/api/fund/market/snapshot"),
    FUND_NEWS => ("fund_news_article_list", "/api/fund/news/article-list"),
    FUND_OFFERINGS => ("fund_offerings_list", "/api/fund/offerings/list"),
    FUND_DRAWDOWNS => ("fund_performance_drawdowns", "/api/fund/performance/drawdowns"),
    FUND_INDICATORS_HISTORICAL => ("fund_performance_indicators_historical", "/api/fund/performance/indicators-historical"),
    FUND_NAV => ("fund_performance_nav", "/api/fund/performance/nav"),
    FUND_RETURNS => ("fund_performance_returns", "/api/fund/performance/returns"),
    FUND_ASSET_ALLOCATION => ("fund_portfolio_asset_allocation", "/api/fund/portfolio/asset-allocation"),
    FUND_BOND_HISTORY => ("fund_portfolio_bond_history", "/api/fund/portfolio/bond-history"),
    FUND_BOND_REPORT_DATES => ("fund_portfolio_bond_report_dates", "/api/fund/portfolio/bond-report-dates"),
    FUND_HOLDINGS => ("fund_portfolio_holdings", "/api/fund/portfolio/holdings"),
    FUND_INDUSTRY_ALLOCATION => ("fund_portfolio_industry_allocation", "/api/fund/portfolio/industry-allocation"),
    FUND_STOCK_HISTORY => ("fund_portfolio_stock_history", "/api/fund/portfolio/stock-history"),
    FUND_STOCK_REPORT_DATES => ("fund_portfolio_stock_report_dates", "/api/fund/portfolio/stock-report-dates"),
    FUND_PROFILE => ("fund_profile_detail", "/api/fund/profile/detail"),
    TICKERS_LIST => ("tickers_list", "/api/meta/tickers/list"),
    TICKERS_SEARCH => ("tickers_search", "/api/meta/tickers/search"),
}
