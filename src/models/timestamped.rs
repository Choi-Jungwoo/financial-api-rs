use serde::Deserialize;

use crate::UnixMillis;

/// A timestamped endpoint payload whose business records are stored in `item`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TimestampedItems<T> {
    pub timestamp: UnixMillis,
    pub item: Vec<T>,
}
