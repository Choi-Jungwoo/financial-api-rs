use crate::endpoints;
use crate::{Client, Error, FundDividendsData, FundType, Response, Thscode, ValidationError};

impl Client {
    /// 获取基金历史分红记录。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_corporate_actions_dividends.rs"),
        "\n```"
    )]
    pub async fn fund_corporate_actions_dividends(
        &self,
        fund_type: FundType,
        thscode: impl TryInto<Thscode, Error: Into<ValidationError>>,
    ) -> Result<Response<FundDividendsData>, Error> {
        self.fund_detail(endpoints::FUND_DIVIDENDS, fund_type, thscode)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[tokio::test]
    async fn dividends_keeps_fund_target_validation_on_its_call_edge() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        for error in [
            client
                .fund_corporate_actions_dividends(FundType::Otc, "510300.SH")
                .await
                .unwrap_err(),
            client
                .fund_corporate_actions_dividends(FundType::Exchange, "025480.OF")
                .await
                .unwrap_err(),
        ] {
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }
}
