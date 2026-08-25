use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use super::market_time::UnixMillis;
use super::wire::wire_enum;
use crate::ValidationError;

/// Mutually exclusive recent-period or timestamp-range financial query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinancialRange {
    Recent { limit: u8 },
    Between { start: UnixMillis, end: UnixMillis },
}

impl FinancialRange {
    pub const fn recent(limit: u8) -> Result<Self, ValidationError> {
        if limit == 0 || limit > 20 {
            return Err(ValidationError::new("limit", "must be in the range 1..=20"));
        }
        Ok(Self::Recent { limit })
    }

    pub const fn between(start: UnixMillis, end: UnixMillis) -> Result<Self, ValidationError> {
        if end.get() < start.get() {
            return Err(ValidationError::new(
                "end",
                "must not be earlier than start",
            ));
        }
        Ok(Self::Between { start, end })
    }
}

/// Financial report identifier in `YYYY-[1-4]` form.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct FinancialReport(String);

impl FinancialReport {
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let bytes = value.as_bytes();
        let valid = bytes.len() == 6
            && bytes[..4].iter().all(u8::is_ascii_digit)
            && bytes[4] == b'-'
            && matches!(bytes[5], b'1'..=b'4');
        if !valid {
            return Err(ValidationError::new("report", "must use YYYY-[1-4] format"));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for FinancialReport {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for FinancialReport {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

wire_enum! {
    /// Financial statement report frequency.
    pub enum FinancialPeriod {
        Annual => "annual",
        Quarterly => "quarterly",
    }
}

wire_enum! {
    /// Fiscal period returned by listed-company financial statements.
    pub enum FiscalPeriod {
        FullYear => "FY",
        FirstQuarter => "Q1",
        SecondQuarter => "Q2",
        ThirdQuarter => "Q3",
        FourthQuarter => "Q4",
    }
}

wire_enum! {
    /// Fixed financial-analysis ability group.
    pub enum FinancialAbilityKind {
        Growth => "growth",
        Profitability => "profitability",
        Solvency => "solvency",
        Operation => "operation",
        CashFlow => "cash-flow",
    }
}

wire_enum! {
    /// Financial indicator identifier defined by the upstream contract.
    pub enum FinancialIndicatorId {
        TotalAssetsGrowthRatio => "total_assets_growth_ratio",
        NetProfitYoyGrowthRatio => "net_profit_yoy_growth_ratio",
        OperatingIncomeYoyGrowthRatio => "operating_income_yoy_growth_ratio",
        OperatingProfitYoyGrowthRatio => "operating_profit_yoy_growth_ratio",
        SaleGrossMargin => "sale_gross_margin",
        SaleNetInterestRatio => "sale_net_interest_ratio",
        TotalAssetsNetRatio => "total_assets_net_ratio",
        DeductWeightedAverageReturnOnEquity => "index_deduct_weighted_avg_roe",
        WeightedAverageReturnOnEquity => "index_weighted_avg_roe",
        CurrentRatio => "current_ratio",
        QuickRatio => "quick_ratio",
        AssetsDebtRatio => "assets_debt_ratio",
        CashRatio => "cash_ratio",
        EarnedInterestMultiple => "earned_interest_multiple",
        LongTermDebtEquityRatio => "long_term_debt_equity_ratio",
        TotalAssetsTurnoverRatio => "total_assets_turnover_ratio",
        InventoryTurnoverRatio => "inventory_turnover_ratio",
        CurrentAssetsTurnoverRatio => "current_assets_turnover_ratio",
        ReceiveAccountTurnoverRatio => "receive_account_turnover_ratio",
        CashOperatingIndex => "cash_operating_index",
        OperatingCashFlowNetDivideIncome => "operating_cash_flow_net_divide_income",
        NetProfitCashContent => "net_profit_cash_content",
        OperatingCashNetYoyGrowthRatio => "operating_cash_net_yoy_growth_ratio",
        CashMeetInvestRatio => "cash_meet_invest_ratio",
    }
}

#[cfg(test)]
mod tests {
    use super::{FinancialRange, FinancialReport};
    use crate::UnixMillis;

    #[test]
    fn financial_range_makes_conflicting_modes_unrepresentable() {
        assert!(FinancialRange::recent(1).is_ok());
        assert!(FinancialRange::recent(20).is_ok());
        assert!(FinancialRange::recent(0).is_err());
        assert!(FinancialRange::recent(21).is_err());

        let start = UnixMillis::new(1_700_000_000_000).unwrap();
        let end = UnixMillis::new(1_600_000_000_000).unwrap();
        assert!(FinancialRange::between(start, end).is_err());
    }

    #[test]
    fn financial_report_uses_standard_fallible_conversion() {
        let report: FinancialReport = "2024-4".parse().unwrap();

        assert_eq!(report.as_str(), "2024-4");
        assert!("2024-5".parse::<FinancialReport>().is_err());
    }
}
