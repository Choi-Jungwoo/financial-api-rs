use crate::endpoints;
use crate::{
    Client, Error, FundHoldersData, FundTopHoldersData, FundType, HolderMergeScope, Response,
    Thscode, ValidationError,
};

use super::FundTarget;

impl Client {
    /// 获取基金持有人结构。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_holders_detail.rs"),
        "\n```"
    )]
    pub async fn fund_holders_detail(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        merge_scope: HolderMergeScope,
    ) -> Result<Response<FundHoldersData>, Error> {
        let mut query = FundTarget::new(fund_type, thscode)?.query();
        query.push(("merge_scope", merge_scope.to_string()));
        self.get(endpoints::FUND_HOLDERS_DETAIL, &query).await
    }

    /// 获取最多十名主要基金持有人。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_holders_top.rs"),
        "\n```"
    )]
    pub async fn fund_holders_top(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        limit: Option<u8>,
    ) -> Result<Response<FundTopHoldersData>, Error> {
        let target = FundTarget::new(fund_type, thscode)?;
        if limit.is_some_and(|limit| limit == 0 || limit > 10) {
            return Err(ValidationError::new("limit", "must be in the range 1..=10").into());
        }
        let mut query = target.query();
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        self.get(endpoints::FUND_HOLDERS_TOP, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[tokio::test]
    async fn top_holders_limit_is_one_through_ten() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let fund = Thscode::new("025480.OF").unwrap();

        for limit in [0, 11] {
            let error = client
                .fund_holders_top(FundType::Otc, &fund, Some(limit))
                .await
                .unwrap_err();
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }
}
