use crate::endpoints;
use crate::{Client, Error, FundOfferingsData, OfferingStatus, Response};

impl Client {
    /// 获取正在认购或即将认购的新发基金。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_offerings_list.rs"),
        "\n```"
    )]
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
