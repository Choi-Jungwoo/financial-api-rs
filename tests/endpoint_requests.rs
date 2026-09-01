use financial_api::{
    AShareCode, Adjustment, ApiKey, Client, Cursor, Error, FundType, IndexTag, JsonValue,
    LimitBreakSortField, MarketDumpUrl, NaturalDate, Page, SortDirection, TickerListRequest,
    TickerSearchRequest, UnixMillis,
};
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn success(data: JsonValue) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({
        "code": 0,
        "message": "success",
        "request_id": "request",
        "data": data
    }))
}

fn client(server: &MockServer) -> Client {
    Client::builder(ApiKey::new("test-api-key").unwrap())
        .base_url(server.uri())
        .reference_date(NaturalDate::parse("2026-08-25").unwrap())
        .build()
        .unwrap()
}

#[tokio::test]
async fn historical_prices_accepts_validated_domain_inputs() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/a-share/prices/historical"))
        .and(query_param("thscode", "600519.SH"))
        .and(query_param("start", "1716105600000"))
        .and(query_param("end", "1716192000000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 2003,
            "message": "capability denied",
            "request_id": "typed-input",
            "data": null
        })))
        .mount(&server)
        .await;

    let target = AShareCode::try_from("600519.SH").unwrap();
    let start = UnixMillis::try_from(1_716_105_600_000).unwrap();
    let end = UnixMillis::try_from(1_716_192_000_000).unwrap();

    let error = client(&server)
        .prices_historical(&target, start, end, Adjustment::None, 0)
        .await
        .unwrap_err();

    assert!(matches!(error, Error::Business(_)));
}

#[tokio::test]
async fn representative_endpoint_families_preserve_their_wire_contracts() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/meta/tickers/list"))
        .and(query_param("asset_type", "a-share,fund-etf"))
        .and(query_param("limit", "500"))
        .and(query_param("offset", "1000"))
        .respond_with(success(json!({"timestamp": 1716105600000_i64, "item": []})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/a-share-index/catalog/ths-index-list"))
        .and(query_param("tag", "industry"))
        .respond_with(success(json!({"timestamp": 1716105600000_i64, "item": []})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/fund/news/article-list"))
        .and(query_param("fund_type", "exchange"))
        .and(query_param("thscode", "510300.SH"))
        .and(query_param("limit", "20"))
        .and(query_param("offset", "opaque+/cursor=="))
        .respond_with(success(json!({
            "timestamp": 1716105600000_i64,
            "limit": 20,
            "offset": null,
            "has_more": false,
            "item": []
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/a-share/special-data/limit-break-pool"))
        .and(query_param("date_ms", "1716134400000"))
        .and(query_param("page", "2"))
        .and(query_param("size", "80"))
        .and(query_param("sort_field", "open_times"))
        .and(query_param("sort_dir", "asc"))
        .respond_with(success(json!({
            "timestamp": 1716105600000_i64,
            "pagination": {"total": 0, "pages": 0, "size": 80, "page": 2},
            "item": []
        })))
        .mount(&server)
        .await;

    let client = client(&server);
    let cursor = Cursor::try_from("opaque+/cursor==").unwrap();
    let date = NaturalDate::try_from("2024-05-20").unwrap();
    let list = TickerListRequest::new()
        .asset_types([
            financial_api::AssetType::AShare,
            financial_api::AssetType::FundEtf,
        ])
        .page(500, 1000)
        .unwrap();
    client.tickers_list(&list).await.unwrap();
    client
        .index_catalog_ths_index_list(IndexTag::Industry)
        .await
        .unwrap();
    client
        .fund_news_article_list(FundType::Exchange, " 510300.sh ", Some(20), Some(&cursor))
        .await
        .unwrap();
    client
        .special_data_limit_break_pool(
            Some(date),
            Page::new(2, 80).unwrap(),
            LimitBreakSortField::OpenTimes,
            SortDirection::Ascending,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn market_dump_url_is_typed_and_redacted_in_debug_output() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/dump/market-dumps/daily-k/download-url"))
        .respond_with(success(json!({
            "presigned_url": "https://storage.example/file?secret=signature",
            "presigned_url_expires_at": "2026-08-25T08:05:00Z"
        })))
        .mount(&server)
        .await;

    let response = client(&server).market_dump_daily_k().await.unwrap();
    let data: &MarketDumpUrl = response.data();

    assert_eq!(
        data.presigned_url.expose(),
        "https://storage.example/file?secret=signature"
    );
    assert_eq!(
        data.presigned_url_expires_at.unix_timestamp(),
        1_787_645_100
    );
    assert!(!format!("{data:?}").contains("signature"));
}

#[tokio::test]
async fn market_dump_rejects_invalid_authorization_metadata() {
    for data in [
        json!({
            "presigned_url": "not-a-url",
            "presigned_url_expires_at": "2026-08-25T08:05:00Z"
        }),
        json!({
            "presigned_url": "https://storage.example/file",
            "presigned_url_expires_at": "tomorrow"
        }),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/dump/market-dumps/daily-k/download-url"))
            .respond_with(success(data))
            .mount(&server)
            .await;

        assert!(matches!(
            client(&server).market_dump_daily_k().await.unwrap_err(),
            Error::InvalidResponse { .. }
        ));
    }
}

#[tokio::test]
async fn valid_upper_bounds_reach_transport_with_exact_queries() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 2003,
            "message": "capability denied",
            "request_id": "boundary",
            "data": null
        })))
        .mount(&server)
        .await;

    let client = client(&server);
    let start = 1_000_000_000_000;
    let ten_year_end = start + 315_576_000_000;
    let search = TickerSearchRequest::new("600519")
        .unwrap()
        .limit(50)
        .unwrap();
    let list = TickerListRequest::new().page(10_000, u32::MAX).unwrap();

    macro_rules! assert_business_error {
        ($future:expr) => {
            assert!(matches!($future.await.unwrap_err(), Error::Business(_)))
        };
    }

    assert_business_error!(client.tickers_search(&search));
    assert_business_error!(client.tickers_list(&list));
    assert_business_error!(client.special_data_limit_break_pool(
        None,
        Page::new(u32::MAX, 200).unwrap(),
        LimitBreakSortField::OpenTimes,
        SortDirection::Ascending,
    ));
    assert_business_error!(client.fund_holders_top(FundType::Otc, "025480.OF", Some(10)));
    assert_business_error!(client.index_prices_historical("000300.SH", start, ten_year_end));
    assert_business_error!(client.prices_historical(
        "600519.SH",
        start,
        ten_year_end,
        Adjustment::Backward,
        u32::MAX,
    ));

    let actual = server
        .received_requests()
        .await
        .unwrap()
        .into_iter()
        .map(|request| {
            (
                request.url.path().to_owned(),
                request.url.query().unwrap_or_default().to_owned(),
            )
        })
        .collect::<std::collections::BTreeSet<_>>();
    let expected = [
        (
            "/api/meta/tickers/search",
            "q=600519&limit=50",
        ),
        (
            "/api/meta/tickers/list",
            "limit=10000&offset=4294967295",
        ),
        (
            "/api/a-share/special-data/limit-break-pool",
            "page=4294967295&size=200&sort_field=open_times&sort_dir=asc",
        ),
        (
            "/api/fund/holders/top",
            "fund_type=otc&thscode=025480.OF&limit=10",
        ),
        (
            "/api/a-share-index/prices/historical",
            "thscode=000300.SH&interval=1d&start=1000000000000&end=1315576000000",
        ),
        (
            "/api/a-share/prices/historical",
            "thscode=600519.SH&interval=1d&start=1000000000000&end=1315576000000&adjust=backward&offset=4294967295",
        ),
    ]
    .into_iter()
    .map(|(path, query)| (path.to_owned(), query.to_owned()))
    .collect();

    assert_eq!(actual, expected);
}
