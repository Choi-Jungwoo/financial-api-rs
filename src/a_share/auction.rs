use crate::endpoints;
use crate::endpoints::join_a_share_codes;
use crate::{
    AShareCode, AuctionBenchmarkData, AuctionSnapshotData, AuctionStage, Client, Error,
    NaturalDate, OptionalInput, Response, ValidationError,
};

impl Client {
    /// 获取一个或多个 A 股标的的集合竞价快照。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/a_share_auction_snapshot.rs"),
        "\n```"
    )]
    pub async fn a_share_auction_snapshot(
        &self,
        thscodes: impl IntoIterator<Item = impl TryInto<AShareCode, Error: Into<ValidationError>>>,
        stage: AuctionStage,
    ) -> Result<Response<AuctionSnapshotData>, Error> {
        let query = [
            ("thscodes", join_a_share_codes(thscodes, None)?),
            ("stage", stage.to_string()),
        ];
        self.get(endpoints::AUCTION_SNAPSHOT, &query).await
    }

    /// 获取集合竞价短线基准，可指定自然日。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/a_share_auction_short_term_benchmark.rs"),
        "\n```"
    )]
    pub async fn a_share_auction_short_term_benchmark(
        &self,
        date: impl TryInto<OptionalInput<NaturalDate>, Error: Into<ValidationError>>,
    ) -> Result<Response<AuctionBenchmarkData>, Error> {
        let date: OptionalInput<NaturalDate> = date.try_into().map_err(Into::into)?;
        let query = date
            .into_inner()
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
            .a_share_auction_snapshot([] as [&str; 0], AuctionStage::Final)
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }
}
