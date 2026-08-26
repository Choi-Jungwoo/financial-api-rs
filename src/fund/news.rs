use crate::endpoints;
use crate::{Client, Cursor, Error, FundNewsData, FundType, Response, Thscode, ValidationError};

use super::FundTarget;

impl Client {
    /// 按不透明游标分页获取基金新闻。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_news_article_list.rs"),
        "\n```"
    )]
    pub async fn fund_news_article_list(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        limit: Option<u32>,
        offset: Option<&Cursor>,
    ) -> Result<Response<FundNewsData>, Error> {
        let target = FundTarget::new(fund_type, thscode)?;
        if limit == Some(0) {
            return Err(ValidationError::new("limit", "must be at least 1").into());
        }
        let mut query = target.query();
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(offset) = offset {
            query.push(("offset", offset.to_string()));
        }
        self.get(endpoints::FUND_NEWS, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[tokio::test]
    async fn article_limit_must_be_positive() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let fund = Thscode::new("025480.OF").unwrap();

        let error = client
            .fund_news_article_list(FundType::Otc, &fund, Some(0), None)
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }
}
