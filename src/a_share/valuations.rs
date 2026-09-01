use crate::endpoints;
use crate::endpoints::join_a_share_codes;
use crate::{AShareCode, Client, Error, Response, ValidationError, ValuationsData};

impl Client {
    /// 获取最多 100 个 A 股标的的最新估值指标。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/a_share_valuations_snapshot.rs"),
        "\n```"
    )]
    pub async fn a_share_valuations_snapshot(
        &self,
        thscodes: impl IntoIterator<Item = impl TryInto<AShareCode, Error: Into<ValidationError>>>
        + Send,
    ) -> Result<Response<ValuationsData>, Error> {
        let query = [("thscodes", join_a_share_codes(thscodes, Some(100))?)];
        self.get(endpoints::VALUATIONS_SNAPSHOT, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AShareCode, ApiKey};

    #[tokio::test]
    async fn snapshot_rejects_more_than_one_hundred_targets_before_transport() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let stock = AShareCode::new("600519.SH").unwrap();

        let error = client
            .a_share_valuations_snapshot(&vec![stock; 101])
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }
}
