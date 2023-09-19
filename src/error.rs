pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[deprecated(note = "don't use generic errors")]
    #[error("Generic Error: {0}")]
    Generic(String),
}

impl Error {
    #[deprecated(note = "don't use generic errors")]
    pub fn generic<T: std::fmt::Display>(e: T) -> Self {
        Self::Generic(e.to_string())
    }
}

impl arma_rs::IntoArma for Error {
    fn to_arma(&self) -> arma_rs::Value {
        arma_rs::Value::String(self.to_string())
    }
}
