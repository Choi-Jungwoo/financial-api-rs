use crate::endpoints;
use crate::{Client, Error, FundProfileData, FundType, Response};

impl Client {
    /// 获取基金基本资料。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_profile_detail.rs"),
        "\n```"
    )]
    pub async fn fund_profile_detail(
        &self,
        fund_type: FundType,
        thscode: impl AsRef<str> + Send,
    ) -> Result<Response<FundProfileData>, Error> {
        self.fund_detail(endpoints::FUND_PROFILE, fund_type, thscode)
            .await
    }
}
