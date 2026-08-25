use crate::endpoints;
use crate::endpoints::join_values;
use crate::{
    Client, Error, IndexCatalogData, IndexConstituentsData, IndexHistoricalData, IndexTag,
    PriceSnapshotData, Response, Thscode, UnixMillis, ValidationError,
};

const TEN_YEARS_MS: i64 = 315_576_000_000;

impl Client {
    /// List Tonghuashun indices for a catalog tag.
    pub async fn index_catalog_ths_index_list(
        &self,
        tag: IndexTag,
    ) -> Result<Response<IndexCatalogData>, Error> {
        self.get(endpoints::INDEX_CATALOG, &[("tag", tag.to_string())])
            .await
    }

    /// Fetch the current constituent list of one index.
    pub async fn index_constituents_ths_stock_list(
        &self,
        thscode: &Thscode,
    ) -> Result<Response<IndexConstituentsData>, Error> {
        self.get(
            endpoints::INDEX_CONSTITUENTS,
            &[("thscode", thscode.to_string())],
        )
        .await
    }

    /// Fetch snapshots for one or more exchange or Tonghuashun indices.
    pub async fn index_prices_snapshot(
        &self,
        thscodes: &[Thscode],
    ) -> Result<Response<PriceSnapshotData>, Error> {
        self.get(
            endpoints::INDEX_SNAPSHOT,
            &[("thscodes", join_values("thscodes", thscodes, None)?)],
        )
        .await
    }

    /// Fetch historical daily K-lines for one index.
    pub async fn index_prices_historical(
        &self,
        thscode: &Thscode,
        start: UnixMillis,
        end: UnixMillis,
    ) -> Result<Response<IndexHistoricalData>, Error> {
        validate_history_window(start, end)?;
        let query = [
            ("thscode", thscode.to_string()),
            ("interval", "1d".to_owned()),
            ("start", start.to_string()),
            ("end", end.to_string()),
        ];
        self.get(endpoints::INDEX_HISTORICAL, &query).await
    }
}

fn validate_history_window(start: UnixMillis, end: UnixMillis) -> Result<(), ValidationError> {
    if end < start {
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[test]
    fn historical_window_enforces_order_and_ten_year_limit() {
        let start = UnixMillis::new(1_000_000_000_000).unwrap();
        let before_start = UnixMillis::new(start.get() - 1).unwrap();
        let limit = UnixMillis::new(start.get() + TEN_YEARS_MS).unwrap();
        let after_limit = UnixMillis::new(limit.get() + 1).unwrap();

        assert!(validate_history_window(start, before_start).is_err());
        assert!(validate_history_window(start, limit).is_ok());
        assert!(validate_history_window(start, after_limit).is_err());
    }

    #[tokio::test]
    async fn historical_prices_keeps_window_validation_on_its_call_edge() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let index = Thscode::new("000300.SH").unwrap();
        let start = UnixMillis::new(1_000_000_000_000).unwrap();
        let invalid_ends = [
            UnixMillis::new(start.get() - 1).unwrap(),
            UnixMillis::new(start.get() + TEN_YEARS_MS + 1).unwrap(),
        ];

        for end in invalid_ends {
            let error = client
                .index_prices_historical(&index, start, end)
                .await
                .unwrap_err();
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }
}
