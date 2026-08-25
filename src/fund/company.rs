use crate::endpoints;
use crate::{Client, CompanyId, Error, FundCompanyData, Response};

impl Client {
    /// Fetch fund company details.
    pub async fn fund_companies_detail(
        &self,
        company_id: &CompanyId,
    ) -> Result<Response<FundCompanyData>, Error> {
        self.get(
            endpoints::FUND_COMPANY_DETAIL,
            &[("company_id", company_id.as_str())],
        )
        .await
    }
}
