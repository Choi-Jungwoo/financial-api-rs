use crate::endpoints;
use crate::endpoints::join_values;
use crate::{
    AShareCode, AuctionBenchmarkData, AuctionSnapshotData, AuctionStage, Client, Error,
    NaturalDate, Response,
};

impl Client {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[tokio::test]
    async fn snapshot_requires_at_least_one_target_before_transport() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();

        let error = client
            .a_share_auction_snapshot(&[], AuctionStage::Final)
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }
}
