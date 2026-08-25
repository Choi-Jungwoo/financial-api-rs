use crate::endpoints;
use crate::endpoints::join_values;
use crate::{AShareCode, Client, Error, Response, ValuationsData};

impl Client {
    /// Fetch latest valuation metrics for up to 100 A-share targets.
    pub async fn a_share_valuations_snapshot(
        &self,
        thscodes: &[AShareCode],
    ) -> Result<Response<ValuationsData>, Error> {
        let query = [("thscodes", join_values("thscodes", thscodes, Some(100))?)];
        self.get(endpoints::VALUATIONS_SNAPSHOT, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

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
