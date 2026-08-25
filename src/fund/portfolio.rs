use crate::endpoints;
use crate::{
    Client, Error, FundAssetAllocationData, FundHoldingsData, FundIndustryAllocationData,
    FundPortfolioHistoryData, FundReportDatesData, FundType, NaturalDate, ReportType, Response,
    Thscode,
};

use super::FundTarget;

impl Client {
    /// Fetch periodically disclosed fund holdings and summary ratios.
    pub async fn fund_portfolio_holdings(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundHoldingsData>, Error> {
        self.fund_detail(endpoints::FUND_HOLDINGS, fund_type, thscode)
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
        let mut query = FundTarget::new(fund_type, thscode)?.query();
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

    async fn fund_report_dates(
        &self,
        path: &str,
        fund_type: FundType,
        thscode: &Thscode,
        report_type: Option<&ReportType>,
    ) -> Result<Response<FundReportDatesData>, Error> {
        let mut query = FundTarget::new(fund_type, thscode)?.query();
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
}
