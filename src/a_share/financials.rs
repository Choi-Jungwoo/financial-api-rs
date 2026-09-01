use serde::de::DeserializeOwned;

use crate::endpoints;
use crate::types::TEN_YEARS_MS;
use crate::{
    AShareCode, BalanceSheetsData, CashFlowStatementsData, Client, Error, FinancialIndicatorsData,
    FinancialPeriod, FinancialReport, IncomeStatementsData, Response, UnixMillis, ValidationError,
};

/// “最近报告期”和“时间戳区间”互斥的财务查询。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinancialRange(FinancialRangeKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FinancialRangeKind {
    Recent { limit: u8 },
    Between { start: UnixMillis, end: UnixMillis },
}

impl FinancialRange {
    pub const fn recent(limit: u8) -> Result<Self, ValidationError> {
        if limit == 0 || limit > 20 {
            return Err(ValidationError::new("limit", "must be in the range 1..=20"));
        }
        Ok(Self(FinancialRangeKind::Recent { limit }))
    }

    pub fn between(
        start: impl TryInto<UnixMillis, Error: Into<ValidationError>>,
        end: impl TryInto<UnixMillis, Error: Into<ValidationError>>,
    ) -> Result<Self, ValidationError> {
        let start: UnixMillis = start.try_into().map_err(Into::into)?;
        let end: UnixMillis = end.try_into().map_err(Into::into)?;
        if end.get() < start.get() {
            return Err(ValidationError::new(
                "end",
                "must not be earlier than start",
            ));
        }
        if end.get() - start.get() > TEN_YEARS_MS {
            return Err(ValidationError::new(
                "end",
                "requested time window exceeds ten years",
            ));
        }
        Ok(Self(FinancialRangeKind::Between { start, end }))
    }
}

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
        thscode: impl TryInto<AShareCode, Error: Into<ValidationError>> + Send,
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
        thscode: impl TryInto<AShareCode, Error: Into<ValidationError>> + Send,
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
        thscode: impl TryInto<AShareCode, Error: Into<ValidationError>> + Send,
        period: FinancialPeriod,
        range: FinancialRange,
    ) -> Result<Response<CashFlowStatementsData>, Error> {
        self.financial_statements(endpoints::CASH_FLOW_STATEMENTS, thscode, period, range)
            .await
    }

    async fn financial_statements<T: DeserializeOwned>(
        &self,
        path: &str,
        thscode: impl TryInto<AShareCode, Error: Into<ValidationError>> + Send,
        period: FinancialPeriod,
        range: FinancialRange,
    ) -> Result<Response<T>, Error> {
        let thscode: AShareCode = thscode.try_into().map_err(Into::into)?;
        let mut query = vec![
            ("thscode", thscode.into_string()),
            ("period", period.to_string()),
        ];
        match range.0 {
            FinancialRangeKind::Recent { limit } => {
                query.push(("limit", limit.to_string()));
            }
            FinancialRangeKind::Between { start, end } => {
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
        thscode: impl TryInto<AShareCode, Error: Into<ValidationError>> + Send,
        report: impl TryInto<FinancialReport, Error: Into<ValidationError>> + Send,
    ) -> Result<Response<FinancialIndicatorsData>, Error> {
        let thscode: AShareCode = thscode.try_into().map_err(Into::into)?;
        let report: FinancialReport = report.try_into().map_err(Into::into)?;
        let query = [("thscode", thscode.as_str()), ("report", report.as_str())];
        self.get(endpoints::FINANCIAL_INDICATORS, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::FinancialRange;

    #[test]
    fn financial_range_rejects_invalid_limits_and_reversed_windows() {
        assert!(FinancialRange::recent(1).is_ok());
        assert!(FinancialRange::recent(20).is_ok());
        assert!(FinancialRange::recent(0).is_err());
        assert!(FinancialRange::recent(21).is_err());

        assert!(FinancialRange::between(1_700_000_000_000, 1_600_000_000_000).is_err());
    }

    #[test]
    fn financial_range_rejects_windows_over_ten_years_at_construction() {
        assert!(FinancialRange::between(1_000_000_000_000, 1_315_576_000_001).is_err());
    }
}
