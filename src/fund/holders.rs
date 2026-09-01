use crate::endpoints;
use crate::{
    Client, Error, FundHoldersData, FundTopHoldersData, FundType, HolderMergeScope, Response,
    Thscode, ValidationError,
};

use super::fund_target_query;

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
        thscode: impl TryInto<Thscode, Error: Into<ValidationError>> + Send,
        merge_scope: HolderMergeScope,
    ) -> Result<Response<FundHoldersData>, Error> {
        let mut query = fund_target_query(fund_type, thscode)?;
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
        thscode: impl TryInto<Thscode, Error: Into<ValidationError>> + Send,
        limit: Option<u8>,
    ) -> Result<Response<FundTopHoldersData>, Error> {
        let mut query = fund_target_query(fund_type, thscode)?;
        if limit.is_some_and(|limit| limit == 0 || limit > 10) {
            return Err(ValidationError::new("limit", "must be in the range 1..=10").into());
        }
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
        for limit in [0, 11] {
            let error = client
                .fund_holders_top(FundType::Otc, "025480.OF", Some(limit))
                .await
                .unwrap_err();
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }
}
