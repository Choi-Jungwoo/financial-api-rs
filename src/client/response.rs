use serde::de::DeserializeOwned;

use super::Response;
use crate::error::{BusinessError, Error};

#[cfg(test)]
mod tests;

impl<T> Response<T> {
    /// Upstream trace identifier.
    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// Endpoint-specific response data.
    #[must_use]
    pub const fn data(&self) -> &T {
        &self.data
    }

    /// Consume the response and return its endpoint data.
    #[must_use]
    pub fn into_data(self) -> T {
        self.data
    }
}

impl Response<serde_json::Value> {
    /// Decode lossless JSON data into an application-owned response type.
    pub fn into_typed<T: DeserializeOwned>(self) -> Result<Response<T>, Error> {
        let data = serde_json::from_value(self.data).map_err(|source| Error::InvalidResponse {
            source: Some(source),
        })?;
        Ok(Response {
            request_id: self.request_id,
            data,
        })
    }
}

pub(super) fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<Response<T>, Error> {
    let envelope: Envelope =
        serde_json::from_slice(bytes).map_err(|source| Error::InvalidResponse {
            source: Some(source),
        })?;

    if envelope.code != 0 {
        return Err(
            BusinessError::new(envelope.code, envelope.message, envelope.request_id).into(),
        );
    }

    let data = serde_json::from_value(envelope.data).map_err(|source| Error::InvalidResponse {
        source: Some(source),
    })?;
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
