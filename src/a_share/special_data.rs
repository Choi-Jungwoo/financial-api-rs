use serde::de::DeserializeOwned;

use crate::endpoints;
use crate::endpoints::join_values;
use crate::{
    AShareCode, AnomalyData, AnomalyTag, Client, DragonTigerBoard, DragonTigerData, Error,
    HotListPeriod, HotStockData, HotStockHistoryData, HotStockTrendData, LadderData,
    LimitBreakData, LimitBreakSortField, LimitDownData, LimitDownSortField, LimitUpData,
    LimitUpSortField, NaturalDate, Page, Response, ShanghaiDateMillis, SortDirection,
    ValidationError,
};

use super::validate_date_order;

impl Client {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    fn client() -> Client {
        Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .reference_date(NaturalDate::parse("2026-08-25").unwrap())
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn stock_analysis_rejects_more_than_fifty_targets_before_transport() {
        let stock = AShareCode::new("600519.SH").unwrap();

        let error = client()
            .special_data_anomaly_analysis_stock(&vec![stock; 51])
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }

    #[tokio::test]
    async fn recent_date_capabilities_enforce_their_documented_windows() {
        let client = client();
        let stock = AShareCode::new("600519.SH").unwrap();

        for error in [
            client
                .special_data_hot_stock_rank_trend(
                    &stock,
                    NaturalDate::parse("2024-02-29").unwrap(),
                    NaturalDate::parse("2025-03-01").unwrap(),
                )
                .await
                .unwrap_err(),
            client
                .special_data_hot_stock_list_history(NaturalDate::parse("2025-08-24").unwrap())
                .await
                .unwrap_err(),
            client
                .special_data_dragon_tiger_list(
                    DragonTigerBoard::All,
                    Some(NaturalDate::parse("2026-08-26").unwrap()),
                )
                .await
                .unwrap_err(),
        ] {
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }
}
