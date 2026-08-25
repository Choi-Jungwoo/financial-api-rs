use crate::endpoints;
use crate::{Client, Error, FundOfferingsData, OfferingStatus, Response};

impl Client {
    /// Fetch active or upcoming fund offerings.
    pub async fn fund_offerings_list(
        &self,
        status: OfferingStatus,
    ) -> Result<Response<FundOfferingsData>, Error> {
        self.get(
            endpoints::FUND_OFFERINGS,
            &[("subscribe", status.to_string())],
        )
        .await
    }
}
