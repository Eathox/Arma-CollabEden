#![deny(missing_docs, clippy::all, clippy::nursery)]
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

//! Networking library to add multiplayer capabilities to Arma 3s Eden Editor.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

pub mod error;
mod message;
mod network;

pub use crate::error::{Error, Result};
use crate::message::{Message, MessageSerde};
use crate::network::{Endpoint, Event, Handler, NetworkIO};
