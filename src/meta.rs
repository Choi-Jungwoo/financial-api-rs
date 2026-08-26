use serde::Serialize;

use crate::endpoints;
use crate::{AssetType, Client, Error, Exchange, Response, SearchQuery, TickerData};

/// 用于标的搜索和跨市场消歧的参数。
#[derive(Debug, Clone, Serialize)]
pub struct TickerSearchRequest {
    q: SearchQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    exchange: Option<Exchange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_type: Option<String>,
    limit: u8,
}

impl TickerSearchRequest {
    #[must_use]
    pub const fn new(q: SearchQuery) -> Self {
        Self {
            q,
            exchange: None,
            asset_type: None,
            limit: 10,
        }
    }

    #[must_use]
    pub const fn exchange(mut self, exchange: Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }

    #[must_use]
    pub fn asset_types(mut self, asset_types: impl IntoIterator<Item = AssetType>) -> Self {
        self.asset_type = join_asset_types(asset_types);
        self
    }

    pub fn limit(mut self, limit: u8) -> Result<Self, crate::ValidationError> {
        if !(1..=50).contains(&limit) {
            return Err(crate::ValidationError::new(
                "limit",
                "must be in the range 1..=50",
            ));
        }
        self.limit = limit;
        Ok(self)
    }
}

impl Client {
    /// 按完整标的代码、纯代码或名称搜索标的。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/tickers_search.rs"),
        "\n```"
    )]
    pub async fn tickers_search(
        &self,
        request: &TickerSearchRequest,
    ) -> Result<Response<TickerData>, Error> {
        self.get(endpoints::TICKERS_SEARCH, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::{TickerListRequest, TickerSearchRequest};
    use crate::SearchQuery;

    #[test]
    fn request_builders_enforce_documented_page_bounds() {
        let search = || TickerSearchRequest::new(SearchQuery::new("600519").unwrap());
        assert!(search().limit(0).is_err());
        assert!(search().limit(1).is_ok());
        assert!(search().limit(50).is_ok());
        assert!(search().limit(51).is_err());

        assert!(TickerListRequest::new().page(0, 0).is_err());
        assert!(TickerListRequest::new().page(1, 0).is_ok());
        assert!(TickerListRequest::new().page(10_000, u32::MAX).is_ok());
        assert!(TickerListRequest::new().page(10_001, 0).is_err());
    }
}

/// 用于浏览标的代码表的参数。
#[derive(Debug, Clone, Serialize)]
pub struct TickerListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_type: Option<String>,
    limit: u16,
    offset: u32,
}

impl TickerListRequest {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            asset_type: None,
            limit: 1_000,
            offset: 0,
        }
    }

    #[must_use]
    pub fn asset_types(mut self, asset_types: impl IntoIterator<Item = AssetType>) -> Self {
        self.asset_type = join_asset_types(asset_types);
        self
    }

    pub fn page(mut self, limit: u16, offset: u32) -> Result<Self, crate::ValidationError> {
        if limit == 0 || limit > 10_000 {
            return Err(crate::ValidationError::new(
                "limit",
                "must be in the range 1..=10000",
            ));
        }
        self.limit = limit;
        self.offset = offset;
        Ok(self)
    }
}

fn join_asset_types(asset_types: impl IntoIterator<Item = AssetType>) -> Option<String> {
    let mut values = Vec::new();
    for asset_type in asset_types {
        let value = asset_type.as_str();
        if !values.contains(&value) {
            values.push(value);
        }
    }
    (!values.is_empty()).then(|| values.join(","))
}

impl Default for TickerListRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// 分页浏览规范化的标的代码表。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/tickers_list.rs"),
        "\n```"
    )]
    pub async fn tickers_list(
        &self,
        request: &TickerListRequest,
    ) -> Result<Response<TickerData>, Error> {
        self.get(endpoints::TICKERS_LIST, request).await
    }
}
