use crate::endpoints;
use crate::{Client, CompanyId, Error, FundCompanyData, Response};

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
        company_id: &CompanyId,
    ) -> Result<Response<FundCompanyData>, Error> {
        self.get(
            endpoints::FUND_COMPANY_DETAIL,
            &[("company_id", company_id.as_str())],
        )
        .await
    }
}
