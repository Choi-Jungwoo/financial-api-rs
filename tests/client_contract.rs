use financial_api::{ApiKey, BusinessErrorKind, Client, Error, SearchQuery, TickerSearchRequest};
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn ticker_search_sends_auth_and_decodes_success_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/meta/tickers/search"))
        .and(header("X-api-key", "test-api-key"))
        .and(query_param("q", "贵州茅台"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "message": "success",
            "request_id": "request-1",
            "data": {
                "timestamp": 1716105600000_i64,
                "item": [{
                    "thscode": "600519.SH",
                    "ticker": "600519",
                    "name": "贵州茅台",
                    "exchange": "SH",
                    "asset_type": "a-share",
                    "currency": "CNY"
                }]
            }
        })))
        .mount(&server)
        .await;

    let client = Client::builder(ApiKey::new("test-api-key").unwrap())
        .base_url(server.uri())
        .build()
        .unwrap();
    let request = TickerSearchRequest::new(SearchQuery::new("贵州茅台").unwrap());

    let response = client.tickers_search(&request).await.unwrap();

    assert_eq!(response.request_id(), "request-1");
    assert_eq!(response.data().item[0].thscode.as_str(), "600519.SH");
}

#[tokio::test]
async fn business_failure_is_distinct_from_transport_success() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/meta/tickers/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 2003,
            "message": "capability denied",
            "request_id": "request-2",
            "data": null
        })))
        .mount(&server)
        .await;

    let client = Client::builder(ApiKey::new("test-api-key").unwrap())
        .base_url(server.uri())
        .build()
        .unwrap();
    let request = TickerSearchRequest::new(SearchQuery::new("600519").unwrap());

    let error = client.tickers_search(&request).await.unwrap_err();

    match error {
        Error::Business(error) => {
            assert_eq!(error.kind(), BusinessErrorKind::PermissionDenied);
            assert_eq!(error.code(), 2003);
            assert_eq!(error.request_id(), "request-2");
        }
        other => panic!("expected business error, got {other:?}"),
    }
}

#[tokio::test]
async fn business_errors_require_the_complete_common_envelope() {
    for body in [
        json!({
            "code": 2003,
            "message": "request rejected",
            "request_id": "request-1"
        }),
        json!({
            "code": 2003,
            "message": "request rejected",
            "data": null
        }),
        json!({
            "code": 2003,
            "request_id": "request-3",
            "data": null
        }),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/meta/tickers/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url(server.uri())
            .build()
            .unwrap();
        let request = TickerSearchRequest::new(SearchQuery::new("600519").unwrap());

        assert!(matches!(
            client.tickers_search(&request).await.unwrap_err(),
            Error::InvalidResponse { .. }
        ));
    }
}

#[tokio::test]
async fn malformed_success_response_is_an_invalid_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/meta/tickers/search"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;

    let client = Client::builder(ApiKey::new("test-api-key").unwrap())
        .base_url(server.uri())
        .build()
        .unwrap();
    let request = TickerSearchRequest::new(SearchQuery::new("600519").unwrap());

    assert!(matches!(
        client.tickers_search(&request).await.unwrap_err(),
        Error::InvalidResponse { .. }
    ));
}

#[tokio::test]
async fn non_success_http_status_is_distinct_from_business_failure() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/meta/tickers/search"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&server)
        .await;

    let client = Client::builder(ApiKey::new("test-api-key").unwrap())
        .base_url(server.uri())
        .build()
        .unwrap();
    let request = TickerSearchRequest::new(SearchQuery::new("600519").unwrap());

    assert!(matches!(
        client.tickers_search(&request).await.unwrap_err(),
        Error::HttpStatus { status: 503 }
    ));
}

#[tokio::test]
async fn redirects_never_forward_the_api_key_to_another_origin() {
    let origin = MockServer::start().await;
    let target = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/meta/tickers/search"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("Location", format!("{}/capture", target.uri())),
        )
        .mount(&origin)
        .await;
    Mock::given(method("GET"))
        .and(path("/capture"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&target)
        .await;

    let client = Client::builder(ApiKey::new("redirect-secret").unwrap())
        .base_url(origin.uri())
        .build()
        .unwrap();
    let request = TickerSearchRequest::new(SearchQuery::new("600519").unwrap());

    assert!(matches!(
        client.tickers_search(&request).await.unwrap_err(),
        Error::HttpStatus { status: 302 }
    ));
    assert!(target.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn request_timeout_is_a_transport_failure() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/meta/tickers/search"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_millis(100))
                .set_body_json(json!({"code": 0, "data": {}})),
        )
        .mount(&server)
        .await;

    let client = Client::builder(ApiKey::new("test-api-key").unwrap())
        .base_url(server.uri())
        .timeout(Duration::from_millis(1))
        .build()
        .unwrap();
    let request = TickerSearchRequest::new(SearchQuery::new("600519").unwrap());

    assert!(matches!(
        client.tickers_search(&request).await.unwrap_err(),
        Error::Transport { .. }
    ));
}

#[test]
fn credential_debug_output_is_redacted() {
    let key = ApiKey::new("super-secret-test-key").unwrap();
    assert!(!format!("{key:?}").contains("super-secret-test-key"));

    let client = Client::builder(key).build().unwrap();
    assert!(!format!("{client:?}").contains("super-secret-test-key"));
}

#[test]
fn base_url_rejects_userinfo_credentials() {
    let error = Client::builder(ApiKey::new("test-api-key").unwrap())
        .base_url("https://user:secret@fuyao.aicubes.cn/")
        .build()
        .unwrap_err();

    let Error::InvalidInput(error) = error else {
        panic!("expected invalid base URL");
    };
    assert_eq!(error.field(), "base_url");
}

#[tokio::test]
async fn successful_envelope_requires_trace_and_data_fields() {
    for body in [
        json!({"code": 0, "message": "success", "data": {}}),
        json!({"code": 0, "message": "success", "request_id": "request"}),
        json!({"code": 0, "request_id": "request", "data": {}}),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/meta/tickers/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url(server.uri())
            .build()
            .unwrap();
        let request = TickerSearchRequest::new(SearchQuery::new("600519").unwrap());
        assert!(matches!(
            client.tickers_search(&request).await.unwrap_err(),
            Error::InvalidResponse { .. }
        ));
    }
}
