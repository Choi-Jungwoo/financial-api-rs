use crate::endpoints;
use crate::{
    Client, Error, FundBalanceSheetsData, FundFinancialIndicatorsData, FundIncomeStatementsData,
    FundType, Response, Thscode, ValidationError,
};

impl Client {
    /// 获取基金财务指标。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_financials_indicators.rs"),
        "\n```"
    )]
    pub async fn fund_financials_indicators(
        &self,
        fund_type: FundType,
        thscode: impl TryInto<Thscode, Error: Into<ValidationError>> + Send,
    ) -> Result<Response<FundFinancialIndicatorsData>, Error> {
        self.fund_detail(endpoints::FUND_INDICATORS, fund_type, thscode)
            .await
    }

    /// 获取基金利润表。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_financials_income_statements.rs"),
        "\n```"
    )]
    pub async fn fund_financials_income_statements(
        &self,
        fund_type: FundType,
        thscode: impl TryInto<Thscode, Error: Into<ValidationError>> + Send,
    ) -> Result<Response<FundIncomeStatementsData>, Error> {
        self.fund_detail(endpoints::FUND_INCOME_STATEMENTS, fund_type, thscode)
            .await
    }

    /// 获取基金资产负债表。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_financials_balance_sheets.rs"),
        "\n```"
    )]
    pub async fn fund_financials_balance_sheets(
        &self,
        fund_type: FundType,
        thscode: impl TryInto<Thscode, Error: Into<ValidationError>> + Send,
    ) -> Result<Response<FundBalanceSheetsData>, Error> {
        self.fund_detail(endpoints::FUND_BALANCE_SHEETS, fund_type, thscode)
            .await
    }
}
