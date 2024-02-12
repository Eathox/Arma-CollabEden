#![deny(missing_docs, clippy::all, clippy::nursery)]
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

//! Networking library to add multiplayer capabilities to Arma 3s Eden Editor.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use serde::{de::DeserializeOwned, Serialize};

mod error;

pub use error::{Error, Result};

/// Message that can be sent between networking managers.
trait NetworkMessage: Serialize + DeserializeOwned {}

/// Serialize message into a byte vector for sending over network.
///
/// # Errors
/// An error is returned if the message cannot be serialized.
#[inline]
fn serialize_message<T>(message: &T) -> Result<Vec<u8>>
where
    T: NetworkMessage,
{
    let mut buffer = Vec::new();
    ciborium::into_writer(&message, &mut buffer)?;
    Ok(buffer)
}

/// Deserialize message from a byte slice send over network.
///
/// # Errors
/// An error is returned if the message cannot be deserialized.
#[inline]
fn deserialize_message<T>(bytes: &[u8]) -> Result<T>
where
    T: NetworkMessage,
{
    let value = ciborium::from_reader(bytes)?;
    Ok(value)
}
