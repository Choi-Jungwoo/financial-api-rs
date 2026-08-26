use crate::endpoints;
use crate::{
    Client, Error, FundAssetAllocationData, FundHoldingsData, FundIndustryAllocationData,
    FundPortfolioHistoryData, FundReportDatesData, FundType, NaturalDate, ReportType, Response,
    Thscode,
};

use super::FundTarget;

impl Client {
    /// 获取定期披露的基金持仓及汇总比例。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_portfolio_holdings.rs"),
        "\n```"
    )]
    pub async fn fund_portfolio_holdings(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundHoldingsData>, Error> {
        self.fund_detail(endpoints::FUND_HOLDINGS, fund_type, thscode)
            .await
    }

    /// 获取历史披露的股票持仓。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_portfolio_stock_history.rs"),
        "\n```"
    )]
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

    /// 获取历史披露的债券持仓。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_portfolio_bond_history.rs"),
        "\n```"
    )]
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

    /// 获取可用的股票持仓报告期。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_portfolio_stock_report_dates.rs"),
        "\n```"
    )]
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

    /// 获取可用的债券持仓报告期。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_portfolio_bond_report_dates.rs"),
        "\n```"
    )]
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

    /// 获取基金资产配置。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_portfolio_asset_allocation.rs"),
        "\n```"
    )]
    pub async fn fund_portfolio_asset_allocation(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundAssetAllocationData>, Error> {
        self.fund_detail(endpoints::FUND_ASSET_ALLOCATION, fund_type, thscode)
            .await
    }

    /// 获取基金行业配置。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_portfolio_industry_allocation.rs"),
        "\n```"
    )]
    pub async fn fund_portfolio_industry_allocation(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundIndustryAllocationData>, Error> {
        self.fund_detail(endpoints::FUND_INDUSTRY_ALLOCATION, fund_type, thscode)
            .await
    }
}
