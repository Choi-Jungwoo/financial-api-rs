use crate::endpoints;
use crate::{Client, Cursor, Error, FundNewsData, FundType, Response, ValidationError};

use super::fund_target_query;

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
        thscode: impl AsRef<str> + Send,
        limit: Option<u32>,
        offset: Option<&str>,
    ) -> Result<Response<FundNewsData>, Error> {
        let mut query = fund_target_query(fund_type, thscode)?;
        if limit == Some(0) {
            return Err(ValidationError::new("limit", "must be at least 1").into());
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(offset) = offset {
            query.push(("offset", Cursor::new(offset)?.into_string()));
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
        let error = client
            .fund_news_article_list(FundType::Otc, "025480.OF", Some(0), None)
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }
}
