use crate::endpoints;
use crate::types::join_values;
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
        if end < start {
            return Err(ValidationError::new("end", "must not be earlier than start").into());
        }
        if end.get() - start.get() > TEN_YEARS_MS {
            return Err(
                ValidationError::new("end", "requested time window exceeds ten years").into(),
            );
        }
        let query = [
            ("thscode", thscode.to_string()),
            ("interval", "1d".to_owned()),
            ("start", start.to_string()),
            ("end", end.to_string()),
        ];
        self.get(endpoints::INDEX_HISTORICAL, &query).await
    }
}
