use serde::de::DeserializeOwned;

use crate::endpoints;
use crate::{
    AShareCode, BalanceSheetsData, CashFlowStatementsData, Client, Error, FinancialIndicatorsData,
    FinancialPeriod, FinancialRange, FinancialReport, IncomeStatementsData, Response,
};

use super::{TEN_YEARS_MS, validate_millis_window};

impl Client {
    /// 获取利润表。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/financials_income_statements.rs"),
        "\n```"
    )]
    pub async fn financials_income_statements(
        &self,
        thscode: &AShareCode,
        period: FinancialPeriod,
        range: FinancialRange,
    ) -> Result<Response<IncomeStatementsData>, Error> {
        self.financial_statements(endpoints::INCOME_STATEMENTS, thscode, period, range)
            .await
    }

    /// 获取资产负债表。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/financials_balance_sheets.rs"),
        "\n```"
    )]
    pub async fn financials_balance_sheets(
        &self,
        thscode: &AShareCode,
        period: FinancialPeriod,
        range: FinancialRange,
    ) -> Result<Response<BalanceSheetsData>, Error> {
        self.financial_statements(endpoints::BALANCE_SHEETS, thscode, period, range)
            .await
    }

    /// 获取现金流量表。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/financials_cash_flow_statements.rs"),
        "\n```"
    )]
    pub async fn financials_cash_flow_statements(
        &self,
        thscode: &AShareCode,
        period: FinancialPeriod,
        range: FinancialRange,
    ) -> Result<Response<CashFlowStatementsData>, Error> {
        self.financial_statements(endpoints::CASH_FLOW_STATEMENTS, thscode, period, range)
            .await
    }

    async fn financial_statements<T: DeserializeOwned>(
        &self,
        path: &str,
        thscode: &AShareCode,
        period: FinancialPeriod,
        range: FinancialRange,
    ) -> Result<Response<T>, Error> {
        let mut query = vec![
            ("thscode", thscode.to_string()),
            ("period", period.to_string()),
        ];
        match range {
            FinancialRange::Recent { limit } => query.push(("limit", limit.to_string())),
            FinancialRange::Between { start, end } => {
                validate_millis_window(start, end, Some(TEN_YEARS_MS))?;
                query.push(("start", start.to_string()));
                query.push(("end", end.to_string()));
            }
        }
        self.get(path, &query).await
    }

    /// 获取指定报告期的五类财务指标。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/financials_indicators.rs"),
        "\n```"
    )]
    pub async fn financials_indicators(
        &self,
        thscode: &AShareCode,
        report: &FinancialReport,
    ) -> Result<Response<FinancialIndicatorsData>, Error> {
        let query = [("thscode", thscode.as_str()), ("report", report.as_str())];
        self.get(endpoints::FINANCIAL_INDICATORS, &query).await
    }
}
