mod request;

use self::request::{validate_exchange_target, validate_history_range, validate_target};
use crate::endpoints;
use crate::{
    Client, CompanyId, Cursor, Error, FundAssetAllocationData, FundBalanceSheetsData,
    FundCompanyData, FundDiagnosticsData, FundDividendsData, FundDrawdownsData,
    FundFinancialIndicatorsData, FundHoldersData, FundHoldingsData, FundIncomeStatementsData,
    FundIndicatorHistoryData, FundIndustryAllocationData, FundManagerDetailData,
    FundManagerExperienceData, FundManagerPerformanceData, FundManagerStyleData,
    FundMarketHistoricalData, FundMarketSnapshotData, FundNavData, FundNavType, FundNewsData,
    FundOfferingsData, FundPortfolioHistoryData, FundProfileData, FundRange, FundReportDatesData,
    FundReturnsData, FundTopHoldersData, FundType, HolderMergeScope, ManagerId,
    ManagerPerformanceRange, NaturalDate, OfferingStatus, ReportType, Response, Thscode,
    UnixMillis, ValidationError,
};
use serde::de::DeserializeOwned;

impl Client {
    /// Fetch fund company details.
    pub async fn fund_companies_detail(
        &self,
        company_id: &CompanyId,
    ) -> Result<Response<FundCompanyData>, Error> {
        self.get(
            endpoints::FUND_COMPANY_DETAIL,
            &[("company_id", company_id.as_str())],
        )
        .await
    }

    /// Fetch historical fund dividends.
    pub async fn fund_corporate_actions_dividends(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundDividendsData>, Error> {
        self.fund_detail(endpoints::FUND_DIVIDENDS, fund_type, thscode)
            .await
    }

    /// Fetch fund diagnostic dimensions.
    pub async fn fund_diagnostics_detail(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundDiagnosticsData>, Error> {
        self.fund_detail(endpoints::FUND_DIAGNOSTICS, fund_type, thscode)
            .await
    }

    /// Fetch fund financial indicators.
    pub async fn fund_financials_indicators(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundFinancialIndicatorsData>, Error> {
        self.fund_detail(endpoints::FUND_INDICATORS, fund_type, thscode)
            .await
    }

    /// Fetch fund income statements.
    pub async fn fund_financials_income_statements(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundIncomeStatementsData>, Error> {
        self.fund_detail(endpoints::FUND_INCOME_STATEMENTS, fund_type, thscode)
            .await
    }

    /// Fetch fund balance sheets.
    pub async fn fund_financials_balance_sheets(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundBalanceSheetsData>, Error> {
        self.fund_detail(endpoints::FUND_BALANCE_SHEETS, fund_type, thscode)
            .await
    }

    /// Fetch fund holder structure.
    pub async fn fund_holders_detail(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        merge_scope: HolderMergeScope,
    ) -> Result<Response<FundHoldersData>, Error> {
        validate_target(fund_type, thscode)?;
        let query = [
            ("fund_type", fund_type.to_string()),
            ("thscode", thscode.to_string()),
            ("merge_scope", merge_scope.to_string()),
        ];
        self.get(endpoints::FUND_HOLDERS_DETAIL, &query).await
    }

    /// Fetch up to ten largest fund holders.
    pub async fn fund_holders_top(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        limit: Option<u8>,
    ) -> Result<Response<FundTopHoldersData>, Error> {
        validate_target(fund_type, thscode)?;
        if limit.is_some_and(|limit| limit == 0 || limit > 10) {
            return Err(ValidationError::new("limit", "must be in the range 1..=10").into());
        }
        let mut query = fund_query(fund_type, thscode);
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        self.get(endpoints::FUND_HOLDERS_TOP, &query).await
    }

    /// Fetch periodically disclosed fund holdings and summary ratios.
    pub async fn fund_portfolio_holdings(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundHoldingsData>, Error> {
        self.fund_detail(endpoints::FUND_HOLDINGS, fund_type, thscode)
            .await
    }

    /// Fetch a fund manager's investment style.
    pub async fn fund_managers_investment_style(
        &self,
        manager_id: &ManagerId,
    ) -> Result<Response<FundManagerStyleData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_STYLE, manager_id, None)
            .await
    }

    /// Fetch a fund manager's performance series.
    pub async fn fund_managers_performance(
        &self,
        manager_id: &ManagerId,
        range: ManagerPerformanceRange,
    ) -> Result<Response<FundManagerPerformanceData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_PERFORMANCE, manager_id, Some(range))
            .await
    }

    /// Fetch a fund manager's professional experience.
    pub async fn fund_managers_experience(
        &self,
        manager_id: &ManagerId,
    ) -> Result<Response<FundManagerExperienceData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_EXPERIENCE, manager_id, None)
            .await
    }

    /// Fetch a fund manager's detailed profile.
    pub async fn fund_managers_detail(
        &self,
        manager_id: &ManagerId,
    ) -> Result<Response<FundManagerDetailData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_DETAIL, manager_id, None)
            .await
    }

    async fn fund_manager<T: DeserializeOwned>(
        &self,
        path: &str,
        manager_id: &ManagerId,
        range: Option<ManagerPerformanceRange>,
    ) -> Result<Response<T>, Error> {
        let mut query = vec![("manager_id", manager_id.to_string())];
        if let Some(range) = range {
            query.push(("range", range.to_string()));
        }
        self.get(path, &query).await
    }

    /// Fetch the latest exchange-traded fund snapshot.
    pub async fn fund_market_snapshot(
        &self,
        thscode: &Thscode,
    ) -> Result<Response<FundMarketSnapshotData>, Error> {
        validate_exchange_target(thscode)?;
        self.get(
            endpoints::FUND_MARKET_SNAPSHOT,
            &[("thscode", thscode.as_str())],
        )
        .await
    }

    /// Fetch exchange-traded fund historical daily prices.
    pub async fn fund_market_historical(
        &self,
        thscode: &Thscode,
        start: UnixMillis,
        end: UnixMillis,
    ) -> Result<Response<FundMarketHistoricalData>, Error> {
        validate_exchange_target(thscode)?;
        validate_history_range(start, end)?;
        let query = [
            ("thscode", thscode.to_string()),
            ("interval", "1d".to_owned()),
            ("start", start.to_string()),
            ("end", end.to_string()),
        ];
        self.get(endpoints::FUND_MARKET_HISTORICAL, &query).await
    }

    /// Fetch cursor-paged fund news.
    pub async fn fund_news_article_list(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        limit: Option<u32>,
        offset: Option<&Cursor>,
    ) -> Result<Response<FundNewsData>, Error> {
        validate_target(fund_type, thscode)?;
        if limit == Some(0) {
            return Err(ValidationError::new("limit", "must be at least 1").into());
        }
        let mut query = fund_query(fund_type, thscode);
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(offset) = offset {
            query.push(("offset", offset.to_string()));
        }
        self.get(endpoints::FUND_NEWS, &query).await
    }

    /// Fetch active or upcoming fund offerings.
    pub async fn fund_offerings_list(
        &self,
        status: OfferingStatus,
    ) -> Result<Response<FundOfferingsData>, Error> {
        self.get(
            endpoints::FUND_OFFERINGS,
            &[("subscribe", status.to_string())],
        )
        .await
    }

    /// Fetch fund NAV series.
    pub async fn fund_performance_nav(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        range: Option<FundRange>,
        nav_type: FundNavType,
    ) -> Result<Response<FundNavData>, Error> {
        validate_target(fund_type, thscode)?;
        let mut query = fund_query(fund_type, thscode);
        if let Some(range) = range {
            query.push(("range", range.to_string()));
        }
        query.push(("nav_type", nav_type.to_string()));
        self.get(endpoints::FUND_NAV, &query).await
    }

    /// Fetch multi-range fund returns and peer rankings.
    pub async fn fund_performance_returns(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundReturnsData>, Error> {
        self.fund_detail(endpoints::FUND_RETURNS, fund_type, thscode)
            .await
    }

    /// Fetch historical fund performance indicators.
    pub async fn fund_performance_indicators_historical(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        start: UnixMillis,
        end: UnixMillis,
    ) -> Result<Response<FundIndicatorHistoryData>, Error> {
        validate_target(fund_type, thscode)?;
        validate_history_range(start, end)?;
        let mut query = fund_query(fund_type, thscode);
        query.push(("start", start.to_string()));
        query.push(("end", end.to_string()));
        self.get(endpoints::FUND_INDICATORS_HISTORICAL, &query)
            .await
    }

    /// Fetch standard-period maximum drawdowns.
    pub async fn fund_performance_drawdowns(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundDrawdownsData>, Error> {
        self.fund_detail(endpoints::FUND_DRAWDOWNS, fund_type, thscode)
            .await
    }

    /// Fetch historical disclosed stock holdings.
    pub async fn fund_portfolio_stock_history(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        report_type: &ReportType,
        end_date: NaturalDate,
    ) -> Result<Response<FundPortfolioHistoryData>, Error> {
        self.fund_portfolio_history(
            endpoints::FUND_STOCK_HISTORY,
            fund_type,
            thscode,
            report_type,
            end_date,
        )
        .await
    }

    /// Fetch historical disclosed bond holdings.
    pub async fn fund_portfolio_bond_history(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        report_type: &ReportType,
        end_date: NaturalDate,
    ) -> Result<Response<FundPortfolioHistoryData>, Error> {
        self.fund_portfolio_history(
            endpoints::FUND_BOND_HISTORY,
            fund_type,
            thscode,
            report_type,
            end_date,
        )
        .await
    }

    async fn fund_portfolio_history(
        &self,
        path: &str,
        fund_type: FundType,
        thscode: &Thscode,
        report_type: &ReportType,
        end_date: NaturalDate,
    ) -> Result<Response<FundPortfolioHistoryData>, Error> {
        validate_target(fund_type, thscode)?;
        let mut query = fund_query(fund_type, thscode);
        query.push(("report_type", report_type.to_string()));
        query.push(("end_date", end_date.to_string()));
        self.get(path, &query).await
    }

    /// Fetch available stock-holding report dates.
    pub async fn fund_portfolio_stock_report_dates(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        report_type: Option<&ReportType>,
    ) -> Result<Response<FundReportDatesData>, Error> {
        self.fund_report_dates(
            endpoints::FUND_STOCK_REPORT_DATES,
            fund_type,
            thscode,
            report_type,
        )
        .await
    }

    /// Fetch available bond-holding report dates.
    pub async fn fund_portfolio_bond_report_dates(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        report_type: Option<&ReportType>,
    ) -> Result<Response<FundReportDatesData>, Error> {
        self.fund_report_dates(
            endpoints::FUND_BOND_REPORT_DATES,
            fund_type,
            thscode,
            report_type,
        )
        .await
    }

    async fn fund_report_dates<T: DeserializeOwned>(
        &self,
        path: &str,
        fund_type: FundType,
        thscode: &Thscode,
        report_type: Option<&ReportType>,
    ) -> Result<Response<T>, Error> {
        validate_target(fund_type, thscode)?;
        let mut query = fund_query(fund_type, thscode);
        if let Some(report_type) = report_type {
            query.push(("report_type", report_type.to_string()));
        }
        self.get(path, &query).await
    }

    /// Fetch fund asset allocation.
    pub async fn fund_portfolio_asset_allocation(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundAssetAllocationData>, Error> {
        self.fund_detail(endpoints::FUND_ASSET_ALLOCATION, fund_type, thscode)
            .await
    }

    /// Fetch fund industry allocation.
    pub async fn fund_portfolio_industry_allocation(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundIndustryAllocationData>, Error> {
        self.fund_detail(endpoints::FUND_INDUSTRY_ALLOCATION, fund_type, thscode)
            .await
    }

    /// Fetch fund basic profile details.
    pub async fn fund_profile_detail(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundProfileData>, Error> {
        self.fund_detail(endpoints::FUND_PROFILE, fund_type, thscode)
            .await
    }

    async fn fund_detail<T: DeserializeOwned>(
        &self,
        path: &str,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<T>, Error> {
        validate_target(fund_type, thscode)?;
        self.get(path, &fund_query(fund_type, thscode)).await
    }
}

fn fund_query(fund_type: FundType, thscode: &Thscode) -> Vec<(&'static str, String)> {
    vec![
        ("fund_type", fund_type.to_string()),
        ("thscode", thscode.to_string()),
    ]
}
