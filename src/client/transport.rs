use serde::Serialize;
use serde::de::DeserializeOwned;

use super::{Client, Response, response};
use crate::{Error, ValidationError};

impl Client {
    pub(crate) async fn get<Q, T>(&self, path: &str, query: &Q) -> Result<Response<T>, Error>
    where
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = self
            .base_url
            .join(path.trim_start_matches('/'))
            .map_err(|_| {
                ValidationError::new("endpoint_path", "could not be joined to the API base URL")
            })?;
        let response = self
            .http
            .get(url)
            .query(query)
            .send()
            .await
            .map_err(|source| Error::Transport { source })?;
        let status = response.status();
        if !status.is_success() {
            return Err(Error::HttpStatus {
                status: status.as_u16(),
            });
        }
        let bytes = response
            .bytes()
            .await
            .map_err(|source| Error::Transport { source })?;
        response::decode(&bytes)
    }
}
