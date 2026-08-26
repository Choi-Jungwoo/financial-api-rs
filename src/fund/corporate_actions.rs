use crate::endpoints;
use crate::{Client, Error, FundDividendsData, FundType, Response, Thscode};

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
        thscode: &Thscode,
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
        let exchange_fund = Thscode::new("510300.SH").unwrap();
        let otc_fund = Thscode::new("025480.OF").unwrap();

        for error in [
            client
                .fund_corporate_actions_dividends(FundType::Otc, &exchange_fund)
                .await
                .unwrap_err(),
            client
                .fund_corporate_actions_dividends(FundType::Exchange, &otc_fund)
                .await
                .unwrap_err(),
        ] {
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }
}
