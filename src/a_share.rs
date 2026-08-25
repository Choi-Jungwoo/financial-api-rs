use crate::endpoints;
use crate::types::join_values;
use crate::{
    AShareCode, Adjustment, AdjustmentFactorsData, AnomalyData, AnomalyTag, AuctionBenchmarkData,
    AuctionSnapshotData, AuctionStage, BalanceSheetsData, CashFlowStatementsData, Client,
    DragonTigerBoard, DragonTigerData, Error, FinancialIndicatorsData, FinancialPeriod,
    FinancialRange, FinancialReport, HistoricalData, HotListPeriod, HotStockData,
    HotStockHistoryData, HotStockTrendData, IncomeStatementsData, LadderData, LimitBreakData,
    LimitBreakSortField, LimitDownData, LimitDownSortField, LimitUpData, LimitUpSortField,
    NaturalDate, Page, PriceSnapshotData, Response, ShanghaiDateMillis, SortDirection,
    TradingDaysData, UnixMillis, ValidationError, ValuationsData,
};
use serde::de::DeserializeOwned;

const TEN_YEARS_MS: i64 = 315_576_000_000;

/// Either explicit targets or a page of the complete A-share universe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriceSnapshotSelection {
    Targets(Vec<AShareCode>),
    MarketPage { limit: u32, offset: u32 },
}

impl PriceSnapshotSelection {
    pub fn targets(targets: Vec<AShareCode>) -> Result<Self, ValidationError> {
        if targets.is_empty() {
            return Err(ValidationError::new("thscodes", "must not be empty"));
        }
        Ok(Self::Targets(targets))
    }

    pub const fn market_page(limit: u32, offset: u32) -> Result<Self, ValidationError> {
        if limit == 0 {
            return Err(ValidationError::new("limit", "must be at least 1"));
        }
        Ok(Self::MarketPage { limit, offset })
    }
}

impl Client {
    /// Fetch current A-share price snapshots.
    pub async fn prices_snapshot(
        &self,
        selection: &PriceSnapshotSelection,
    ) -> Result<Response<PriceSnapshotData>, Error> {
        let query = match selection {
            PriceSnapshotSelection::Targets(codes) => {
                vec![("thscodes", join_values("thscodes", codes, None)?)]
            }
            PriceSnapshotSelection::MarketPage { limit, offset } => {
                vec![("limit", limit.to_string()), ("offset", offset.to_string())]
            }
        };
        self.get(endpoints::PRICES_SNAPSHOT, &query).await
    }

    /// Fetch one target's historical daily K-line data.
    pub async fn prices_historical(
        &self,
        thscode: &AShareCode,
        start: UnixMillis,
        end: UnixMillis,
        adjustment: Adjustment,
        offset: u32,
    ) -> Result<Response<HistoricalData>, Error> {
        validate_millis_window(start, end, Some(TEN_YEARS_MS))?;
        let query = vec![
            ("thscode", thscode.to_string()),
            ("interval", "1d".to_owned()),
            ("start", start.to_string()),
            ("end", end.to_string()),
            ("adjust", adjustment.to_string()),
            ("offset", offset.to_string()),
        ];
        self.get(endpoints::PRICES_HISTORICAL, &query).await
    }

    /// Fetch raw corporate action events used for price adjustment.
    pub async fn corp_actions_adjustment_factors(
        &self,
        thscode: &AShareCode,
        from: Option<NaturalDate>,
        to: Option<NaturalDate>,
    ) -> Result<Response<AdjustmentFactorsData>, Error> {
        let mut query = vec![("thscode", thscode.to_string())];
        if let (Some(from), Some(to)) = (from, to) {
            validate_date_order(from, to, "to")?;
        }
        if let Some(from) = from {
            query.push(("from", from.to_string()));
        }
        if let Some(to) = to {
            query.push(("to", to.to_string()));
        }
        self.get(endpoints::ADJUSTMENT_FACTORS, &query).await
    }

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

    /// Fetch the service's recent A-share trading-day sequence.
    pub async fn calendar_trading_days(&self) -> Result<Response<TradingDaysData>, Error> {
        self.get(endpoints::TRADING_DAYS, &[] as &[(&str, &str)])
            .await
    }

    /// Fetch auction snapshots for one or more A-share targets.
    pub async fn a_share_auction_snapshot(
        &self,
        thscodes: &[AShareCode],
        stage: AuctionStage,
    ) -> Result<Response<AuctionSnapshotData>, Error> {
        let query = [
            ("thscodes", join_values("thscodes", thscodes, None)?),
            ("stage", stage.to_string()),
        ];
        self.get(endpoints::AUCTION_SNAPSHOT, &query).await
    }

    /// Fetch the short-term auction benchmark, optionally for a natural day.
    pub async fn a_share_auction_short_term_benchmark(
        &self,
        date: Option<NaturalDate>,
    ) -> Result<Response<AuctionBenchmarkData>, Error> {
        let query = date
            .map(|date| vec![("date", date.to_string())])
            .unwrap_or_default();
        self.get(endpoints::AUCTION_BENCHMARK, &query).await
    }

    /// Fetch latest valuation metrics for up to 100 A-share targets.
    pub async fn a_share_valuations_snapshot(
        &self,
        thscodes: &[AShareCode],
    ) -> Result<Response<ValuationsData>, Error> {
        let query = [("thscodes", join_values("thscodes", thscodes, Some(100))?)];
        self.get(endpoints::VALUATIONS_SNAPSHOT, &query).await
    }

    /// Fetch today's anomaly rows, optionally filtered with OR-combined tags.
    pub async fn special_data_anomaly_analysis_list(
        &self,
        tag_codes: &[AnomalyTag],
    ) -> Result<Response<AnomalyData>, Error> {
        let query = if tag_codes.is_empty() {
            Vec::new()
        } else {
            vec![("tag_codes", join_values("tag_codes", tag_codes, None)?)]
        };
        self.get(endpoints::ANOMALY_LIST, &query).await
    }

    /// Fetch today's anomaly rows for up to 50 A-share targets.
    pub async fn special_data_anomaly_analysis_stock(
        &self,
        thscodes: &[AShareCode],
    ) -> Result<Response<AnomalyData>, Error> {
        let query = [("thscodes", join_values("thscodes", thscodes, Some(50))?)];
        self.get(endpoints::ANOMALY_STOCK, &query).await
    }

    /// Fetch the dragon-tiger list.
    pub async fn special_data_dragon_tiger_list(
        &self,
        board_type: DragonTigerBoard,
        date: Option<NaturalDate>,
    ) -> Result<Response<DragonTigerData>, Error> {
        if let Some(date) = date {
            self.validate_recent_date(date, "date")?;
        }
        let mut query = vec![("board_type", board_type.to_string())];
        if let Some(date) = date {
            query.push(("date", date.to_string()));
        }
        self.get(endpoints::DRAGON_TIGER, &query).await
    }

    /// Fetch the current hot-stock list.
    pub async fn special_data_hot_stock_list(
        &self,
        period: HotListPeriod,
    ) -> Result<Response<HotStockData>, Error> {
        self.get(endpoints::HOT_STOCK_LIST, &[("period", period.to_string())])
            .await
    }

    /// Fetch a historical hot-stock ranking for one natural day.
    pub async fn special_data_hot_stock_list_history(
        &self,
        date: NaturalDate,
    ) -> Result<Response<HotStockHistoryData>, Error> {
        self.validate_recent_date(date, "date")?;
        self.get(endpoints::HOT_STOCK_HISTORY, &[("date", date.to_string())])
            .await
    }

    /// Fetch one A-share target's hot-rank trend.
    pub async fn special_data_hot_stock_rank_trend(
        &self,
        thscode: &AShareCode,
        start_date: NaturalDate,
        end_date: NaturalDate,
    ) -> Result<Response<HotStockTrendData>, Error> {
        validate_date_order(start_date, end_date, "end_date")?;
        self.validate_recent_date(start_date, "start_date")?;
        self.validate_recent_date(end_date, "end_date")?;
        if start_date
            .checked_add_years(1)
            .is_none_or(|limit| end_date > limit)
        {
            return Err(
                ValidationError::new("end_date", "date range must not exceed one year").into(),
            );
        }
        let query = [
            ("thscode", thscode.to_string()),
            ("start_date", start_date.to_string()),
            ("end_date", end_date.to_string()),
        ];
        self.get(endpoints::HOT_STOCK_TREND, &query).await
    }

    /// Fetch the limit-up pool.
    pub async fn special_data_limit_up_pool(
        &self,
        date_ms: Option<ShanghaiDateMillis>,
        page: Page,
        sort_field: LimitUpSortField,
        sort_dir: SortDirection,
    ) -> Result<Response<LimitUpData>, Error> {
        self.special_data_pool(
            endpoints::LIMIT_UP_POOL,
            date_ms,
            page,
            sort_field.to_string(),
            sort_dir,
        )
        .await
    }

    /// Fetch the limit-down pool.
    pub async fn special_data_limit_down_pool(
        &self,
        date_ms: Option<ShanghaiDateMillis>,
        page: Page,
        sort_field: LimitDownSortField,
        sort_dir: SortDirection,
    ) -> Result<Response<LimitDownData>, Error> {
        self.special_data_pool(
            endpoints::LIMIT_DOWN_POOL,
            date_ms,
            page,
            sort_field.to_string(),
            sort_dir,
        )
        .await
    }

    /// Fetch the limit-break pool.
    pub async fn special_data_limit_break_pool(
        &self,
        date_ms: Option<ShanghaiDateMillis>,
        page: Page,
        sort_field: LimitBreakSortField,
        sort_dir: SortDirection,
    ) -> Result<Response<LimitBreakData>, Error> {
        self.special_data_pool(
            endpoints::LIMIT_BREAK_POOL,
            date_ms,
            page,
            sort_field.to_string(),
            sort_dir,
        )
        .await
    }

    async fn special_data_pool<T: DeserializeOwned>(
        &self,
        path: &str,
        date_ms: Option<ShanghaiDateMillis>,
        page: Page,
        sort_field: String,
        sort_dir: SortDirection,
    ) -> Result<Response<T>, Error> {
        let mut query = vec![
            ("page", page.number().to_string()),
            ("size", page.size().to_string()),
            ("sort_field", sort_field),
            ("sort_dir", sort_dir.to_string()),
        ];
        if let Some(date_ms) = date_ms {
            query.push(("date_ms", date_ms.to_string()));
        }
        self.get(path, &query).await
    }

    fn validate_recent_date(
        &self,
        date: NaturalDate,
        field: &'static str,
    ) -> Result<(), ValidationError> {
        let today = self.shanghai_today();
        let earliest = today
            .checked_add_years(-1)
            .ok_or_else(|| ValidationError::new(field, "could not calculate one-year window"))?;
        if date < earliest || date > today {
            return Err(ValidationError::new(
                field,
                "must be within the previous natural year and not in the future",
            ));
        }
        Ok(())
    }

    /// Fetch the fixed 30-trading-day limit-up ladder.
    pub async fn special_data_limit_up_ladder(&self) -> Result<Response<LadderData>, Error> {
        self.get(endpoints::LIMIT_UP_LADDER, &[] as &[(&str, &str)])
            .await
    }

    /// Fetch the current skyrocket ranking.
    pub async fn special_data_skyrocket_list(
        &self,
        period: HotListPeriod,
    ) -> Result<Response<HotStockData>, Error> {
        self.get(endpoints::SKYROCKET_LIST, &[("period", period.to_string())])
            .await
    }
}

fn validate_millis_window(
    start: UnixMillis,
    end: UnixMillis,
    maximum: Option<i64>,
) -> Result<(), ValidationError> {
    if end < start {
        return Err(ValidationError::new(
            "end",
            "must not be earlier than start",
        ));
    }
    if maximum.is_some_and(|maximum| end.get() - start.get() > maximum) {
        return Err(ValidationError::new(
            "end",
            "requested time window exceeds the endpoint limit",
        ));
    }
    Ok(())
}

fn validate_date_order(
    start: NaturalDate,
    end: NaturalDate,
    end_field: &'static str,
) -> Result<(), ValidationError> {
    if end < start {
        return Err(ValidationError::new(
            end_field,
            "must not be earlier than the start date",
        ));
    }
    Ok(())
}
