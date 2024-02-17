use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Ping,
    Pong,
    Disconnected,

    ArmaEvent(String, arma_rs::Value),
}

/// Message that can be sent over network.
pub trait MessageSerde: Serialize + DeserializeOwned {
    /// Serialize message into a byte vector for sending over network.
    ///
    /// # Errors
    /// An error is returned if the message cannot be serialized.
    #[inline]
    fn to_bytes(&self) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
        let mut buffer = Vec::new();
        ciborium::into_writer(self, &mut buffer)?;
        Ok(buffer)
    }

    /// Deserialize message from a byte slice send over network.
    ///
    /// # Errors
    /// An error is returned if the message cannot be deserialized.
    #[inline]
    fn from_bytes<T: MessageSerde>(bytes: &[u8]) -> Result<T, ciborium::de::Error<std::io::Error>> {
        ciborium::from_reader(bytes)
    }
}

impl MessageSerde for Message {}
