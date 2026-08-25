use crate::endpoints;
use crate::{Client, Error, FundDiagnosticsData, FundType, Response, Thscode};

impl Client {
    /// Fetch fund diagnostic dimensions.
    pub async fn fund_diagnostics_detail(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundDiagnosticsData>, Error> {
        self.fund_detail(endpoints::FUND_DIAGNOSTICS, fund_type, thscode)
            .await
    }
}
