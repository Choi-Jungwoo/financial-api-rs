use crate::endpoints;
use crate::{Client, Error, FundProfileData, FundType, Response, Thscode};

impl Client {
    /// Fetch fund basic profile details.
    pub async fn fund_profile_detail(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundProfileData>, Error> {
        self.fund_detail(endpoints::FUND_PROFILE, fund_type, thscode)
            .await
    }
}
