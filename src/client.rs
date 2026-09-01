mod configuration;
mod credentials;
mod reference_date;
mod response;
mod transport;

use std::time::Duration;

use reqwest::header::HeaderValue;

use crate::NaturalDate;

/// 以敏感 HTTP 请求头值保存的已校验 API Key。
#[derive(Clone)]
pub struct ApiKey(HeaderValue);

/// 通过通用业务信封校验后的成功响应。
#[derive(Debug, Clone, PartialEq)]
pub struct Response<T> {
    request_id: String,
    data: T,
}

/// 用于配置客户端级可选传输参数的构建器。
#[derive(Debug)]
pub struct ClientBuilder {
    api_key: ApiKey,
    base_url: String,
    timeout: Duration,
    reference_date: Option<NaturalDate>,
}

/// 可复用的异步 API 客户端。
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: reqwest::Url,
    reference_date: Option<NaturalDate>,
}

#[cfg(test)]
mod tests {
    use crate::{ApiKey, BusinessErrorKind, Client, Error, TickerSearchRequest};
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
        let request = TickerSearchRequest::new("贵州茅台").unwrap();

        let response = client.tickers_search(&request).await.unwrap();

        assert_eq!(response.request_id(), "request-1");
        assert_eq!(response.data().item[0].thscode.as_str(), "600519.SH");
    }

    #[tokio::test]
    async fn business_failures_preserve_recovery_categories_after_transport_success() {
        for (code, expected) in [
            (2001, BusinessErrorKind::Authentication),
            (2003, BusinessErrorKind::PermissionDenied),
            (4001, BusinessErrorKind::RateLimited),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/api/meta/tickers/search"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "code": code,
                    "message": "request rejected",
                    "request_id": "request-2",
                    "data": null
                })))
                .mount(&server)
                .await;

            let client = Client::builder(ApiKey::new("test-api-key").unwrap())
                .base_url(server.uri())
                .build()
                .unwrap();
            let request = TickerSearchRequest::new("600519").unwrap();

            match client.tickers_search(&request).await.unwrap_err() {
                Error::Business(error) => {
                    assert_eq!(error.kind(), expected);
                    assert_eq!(error.code(), code);
                    assert_eq!(error.request_id(), "request-2");
                }
                other => panic!("expected business error, got {other:?}"),
            }
        }
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
        let request = TickerSearchRequest::new("600519").unwrap();

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
        let request = TickerSearchRequest::new("600519").unwrap();

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
        let request = TickerSearchRequest::new("600519").unwrap();

        assert!(matches!(
            client.tickers_search(&request).await.unwrap_err(),
            Error::Transport { .. }
        ));
    }

    #[test]
    fn client_debug_output_does_not_expose_the_credential() {
        let client = Client::builder(ApiKey::new("super-secret-test-key").unwrap())
            .build()
            .unwrap();

        assert!(!format!("{client:?}").contains("super-secret-test-key"));
    }
}
