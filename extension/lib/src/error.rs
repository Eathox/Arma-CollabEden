/// QOL type alias for Result to default to crate [`Error`]
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
/// Errors that can occur using this library.
pub enum Error {
    /// Failed to serialize message.
    #[error("Failed to serialize Message: {0}")]
    Serialize(#[from] ciborium::ser::Error<std::io::Error>),
    /// Failed to deserialize message.
    #[error("Failed to deserialize Message: {0}")]
    Deserialize(#[from] ciborium::de::Error<std::io::Error>),

    // /// Network IO error.
    // #[error("Network IO Error: {0}")]
    // IO(#[from] std::io::Error),
    /// Generic catch all error.
    /// WIP: only for during early development.
    #[deprecated(note = "don't use generic errors")]
    #[error("Generic Error: {0}")]
    Generic(String),
}

impl Error {
    /// Helper function to an create generic error.
    #[deprecated(note = "don't use generic errors")]
    #[inline]
    pub fn generic<T: std::fmt::Display>(e: T) -> Self {
        Self::Generic(e.to_string())
    }
}

impl arma_rs::IntoArma for Error {
    #[inline]
    fn to_arma(&self) -> arma_rs::Value {
        arma_rs::Value::String(self.to_string())
    }
}
