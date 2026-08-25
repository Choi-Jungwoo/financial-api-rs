mod configuration;
mod credentials;
mod reference_date;
mod response;
mod transport;

#[cfg(test)]
mod tests;

use std::time::Duration;

use reqwest::header::HeaderValue;

use crate::NaturalDate;

/// Validated API key stored as a sensitive HTTP header value.
#[derive(Clone)]
pub struct ApiKey(HeaderValue);

/// Successful response after the common business envelope has been checked.
#[derive(Debug, Clone, PartialEq)]
pub struct Response<T> {
    request_id: String,
    data: T,
}

/// Builder for optional client-wide transport configuration.
#[derive(Debug)]
pub struct ClientBuilder {
    api_key: ApiKey,
    base_url: String,
    timeout: Duration,
    reference_date: Option<NaturalDate>,
}

/// Reusable async API client.
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: reqwest::Url,
    reference_date: Option<NaturalDate>,
}
