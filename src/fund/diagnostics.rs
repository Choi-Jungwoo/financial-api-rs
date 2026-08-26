use crate::endpoints;
use crate::{Client, Error, FundDiagnosticsData, FundType, Response, Thscode};

impl Client {
    /// 获取基金诊断维度。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_diagnostics_detail.rs"),
        "\n```"
    )]
    pub async fn fund_diagnostics_detail(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundDiagnosticsData>, Error> {
        self.fund_detail(endpoints::FUND_DIAGNOSTICS, fund_type, thscode)
            .await
    }
}
