use serde::{Deserialize, Serialize};

/// JSON decimal number whose source digits are preserved without binary-float conversion.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::Display, derive_more::From,
)]
#[serde(transparent)]
#[display("{_0}")]
pub struct PreciseDecimal(serde_json::Number);

impl PreciseDecimal {
    /// Borrow the lossless JSON number representation.
    #[must_use]
    pub const fn as_number(&self) -> &serde_json::Number {
        &self.0
    }
}
