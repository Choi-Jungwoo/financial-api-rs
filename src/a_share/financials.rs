use serde::de::DeserializeOwned;

use crate::endpoints;
use crate::{
    AShareCode, BalanceSheetsData, CashFlowStatementsData, Client, Error, FinancialIndicatorsData,
    FinancialPeriod, FinancialRange, FinancialReport, IncomeStatementsData, Response,
};

use super::{TEN_YEARS_MS, validate_millis_window};

impl Client {
    /// Fetch income statements.
    pub async fn financials_income_statements(
        &self,
        thscode: &AShareCode,
        period: FinancialPeriod,
        range: FinancialRange,
    ) -> Result<Response<IncomeStatementsData>, Error> {
        self.financial_statements(endpoints::INCOME_STATEMENTS, thscode, period, range)
            .await
    }

    /// Fetch balance sheets.
    pub async fn financials_balance_sheets(
        &self,
        thscode: &AShareCode,
        period: FinancialPeriod,
        range: FinancialRange,
    ) -> Result<Response<BalanceSheetsData>, Error> {
        self.financial_statements(endpoints::BALANCE_SHEETS, thscode, period, range)
            .await
    }

    /// Fetch cash-flow statements.
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

    /// Fetch the five groups of financial indicators for one report.
    pub async fn financials_indicators(
        &self,
        thscode: &AShareCode,
        report: &FinancialReport,
    ) -> Result<Response<FinancialIndicatorsData>, Error> {
        let query = [("thscode", thscode.as_str()), ("report", report.as_str())];
        self.get(endpoints::FINANCIAL_INDICATORS, &query).await
    }
}
