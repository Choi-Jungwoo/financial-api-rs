use crate::endpoints;
use crate::{
    Client, Error, FundBalanceSheetsData, FundFinancialIndicatorsData, FundIncomeStatementsData,
    FundType, Response, Thscode,
};

impl Client {
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
}
