//! Errors that can occur using this library.

use std::net::SocketAddr;

/// QOL type alias for Result to default to crate [`Error`]
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
/// Errors that can occur using this library.
pub enum Error {
    #[error("Failed to start listening on: {0}: {1}")]
    ListenOn(SocketAddr, std::io::Error),
    #[error("Failed to send connection attempt to: {0}: {1}")]
    ConnectTo(SocketAddr, std::io::Error),

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
