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
