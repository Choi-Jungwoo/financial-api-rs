use crate::endpoints;
use crate::endpoints::join_thscodes;
use crate::types::TEN_YEARS_MS;
use crate::{
    Client, Error, IndexCatalogData, IndexConstituentsData, IndexHistoricalData, IndexTag,
    PriceSnapshotData, Response, Thscode, UnixMillis, ValidationError,
};

impl Client {
    /// 按目录标签列出同花顺指数。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/index_catalog_ths_index_list.rs"),
        "\n```"
    )]
    pub async fn index_catalog_ths_index_list(
        &self,
        tag: IndexTag,
    ) -> Result<Response<IndexCatalogData>, Error> {
        self.get(endpoints::INDEX_CATALOG, &[("tag", tag.to_string())])
            .await
    }

    /// 获取指定指数当前的成分股列表。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/index_constituents_ths_stock_list.rs"),
        "\n```"
    )]
    pub async fn index_constituents_ths_stock_list(
        &self,
        thscode: impl TryInto<Thscode, Error: Into<ValidationError>>,
    ) -> Result<Response<IndexConstituentsData>, Error> {
        let thscode: Thscode = thscode.try_into().map_err(Into::into)?;
        self.get(
            endpoints::INDEX_CONSTITUENTS,
            &[("thscode", thscode.as_str())],
        )
        .await
    }

    /// 获取一个或多个交易所指数或同花顺指数的行情快照。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/index_prices_snapshot.rs"),
        "\n```"
    )]
    pub async fn index_prices_snapshot(
        &self,
        thscodes: impl IntoIterator<Item = impl TryInto<Thscode, Error: Into<ValidationError>>>,
    ) -> Result<Response<PriceSnapshotData>, Error> {
        self.get(
            endpoints::INDEX_SNAPSHOT,
            &[("thscodes", join_thscodes(thscodes, None)?)],
        )
        .await
    }

    /// 获取指定指数的历史日 K 线。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/index_prices_historical.rs"),
        "\n```"
    )]
    pub async fn index_prices_historical(
        &self,
        thscode: impl TryInto<Thscode, Error: Into<ValidationError>>,
        start: impl TryInto<UnixMillis, Error: Into<ValidationError>>,
        end: impl TryInto<UnixMillis, Error: Into<ValidationError>>,
    ) -> Result<Response<IndexHistoricalData>, Error> {
        let thscode: Thscode = thscode.try_into().map_err(Into::into)?;
        let start: UnixMillis = start.try_into().map_err(Into::into)?;
        let end: UnixMillis = end.try_into().map_err(Into::into)?;
        validate_history_window(start, end)?;
        let query = [
            ("thscode", thscode.into_string()),
            ("interval", "1d".to_owned()),
            ("start", start.to_string()),
            ("end", end.to_string()),
        ];
        self.get(endpoints::INDEX_HISTORICAL, &query).await
    }
}

fn validate_history_window(start: UnixMillis, end: UnixMillis) -> Result<(), ValidationError> {
    if end < start {
        return Err(ValidationError::new(
            "end",
            "must not be earlier than start",
        ));
    }
    if end.get() - start.get() > TEN_YEARS_MS {
        return Err(ValidationError::new(
            "end",
            "requested time window exceeds ten years",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::ApiKey;

    struct LocalThscode(Rc<str>);

    impl TryFrom<LocalThscode> for Thscode {
        type Error = ValidationError;

        fn try_from(value: LocalThscode) -> Result<Self, Self::Error> {
            Self::new(value.0)
        }
    }

    #[test]
    fn historical_window_enforces_order_and_ten_year_limit() {
        let start = UnixMillis::new(1_000_000_000_000).unwrap();
        let before_start = UnixMillis::new(start.get() - 1).unwrap();
        let limit = UnixMillis::new(start.get() + TEN_YEARS_MS).unwrap();
        let after_limit = UnixMillis::new(limit.get() + 1).unwrap();

        assert!(validate_history_window(start, before_start).is_err());
        assert!(validate_history_window(start, limit).is_ok());
        assert!(validate_history_window(start, after_limit).is_err());
    }

    #[tokio::test]
    async fn historical_prices_keeps_window_validation_on_its_call_edge() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let index = Thscode::new("000300.SH").unwrap();
        let start = UnixMillis::new(1_000_000_000_000).unwrap();
        let invalid_ends = [
            UnixMillis::new(start.get() - 1).unwrap(),
            UnixMillis::new(start.get() + TEN_YEARS_MS + 1).unwrap(),
        ];

        for end in invalid_ends {
            let error = client
                .index_prices_historical(&index, start.get(), end.get())
                .await
                .unwrap_err();
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn endpoint_input_does_not_require_send() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();

        let error = client
            .index_constituents_ths_stock_list(LocalThscode(Rc::from("invalid")))
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }
}
