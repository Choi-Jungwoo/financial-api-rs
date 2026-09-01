use serde::de::DeserializeOwned;

use super::Response;
use crate::error::{BusinessError, Error};

impl<T> Response<T> {
    /// 上游请求追踪标识。
    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// 端点特定的响应数据。
    #[must_use]
    pub const fn data(&self) -> &T {
        &self.data
    }

    /// 消耗响应并返回端点数据。
    #[must_use]
    pub fn into_data(self) -> T {
        self.data
    }
}

impl Response<serde_json::Value> {
    /// 将无损 JSON 数据解码为应用自有的响应类型。
    pub fn into_typed<T: DeserializeOwned>(self) -> Result<Response<T>, Error> {
        let data = serde_json::from_value(self.data)
            .map_err(|source| Error::InvalidResponse { source })?;
        Ok(Response {
            request_id: self.request_id,
            data,
        })
    }
}

pub(super) fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<Response<T>, Error> {
    let envelope: Envelope =
        serde_json::from_slice(bytes).map_err(|source| Error::InvalidResponse { source })?;

    if envelope.code != 0 {
        return Err(
            BusinessError::new(envelope.code, envelope.message, envelope.request_id).into(),
        );
    }

    let data = serde_json::from_value(envelope.data)
        .map_err(|source| Error::InvalidResponse { source })?;
    Ok(Response {
        request_id: envelope.request_id,
        data,
    })
}

#[derive(serde::Deserialize)]
struct Envelope {
    code: i64,
    message: String,
    request_id: String,
    data: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::decode;
    use crate::Error;

    fn assert_invalid_response(body: Value) {
        let bytes = serde_json::to_vec(&body).unwrap();
        assert!(matches!(
            decode::<Value>(&bytes).unwrap_err(),
            Error::InvalidResponse { .. }
        ));
    }

    #[test]
    fn business_errors_require_the_complete_common_envelope() {
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
            assert_invalid_response(body);
        }
    }

    #[test]
    fn malformed_success_response_is_an_invalid_response() {
        assert!(matches!(
            decode::<Value>(b"not-json").unwrap_err(),
            Error::InvalidResponse { .. }
        ));
    }

    #[test]
    fn successful_envelope_requires_trace_and_data_fields() {
        for body in [
            json!({"code": 0, "message": "success", "data": {}}),
            json!({"code": 0, "message": "success", "request_id": "request"}),
            json!({"code": 0, "request_id": "request", "data": {}}),
        ] {
            assert_invalid_response(body);
        }
    }
}
