use crate::endpoints;
use crate::{Client, CompanyId, Error, FundCompanyData, Response, ValidationError};

impl Client {
    /// 获取基金公司详情。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_companies_detail.rs"),
        "\n```"
    )]
    pub async fn fund_companies_detail(
        &self,
        company_id: impl TryInto<CompanyId, Error: Into<ValidationError>>,
    ) -> Result<Response<FundCompanyData>, Error> {
        let company_id: CompanyId = company_id.try_into().map_err(Into::into)?;
        self.get(
            endpoints::FUND_COMPANY_DETAIL,
            &[("company_id", company_id.as_str())],
        )
        .await
    }
}
