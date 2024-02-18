#![deny(clippy::all, clippy::nursery)] // missing_docs,
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

//! Networking library to add multiplayer capabilities to Arma 3s Eden Editor.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use std::net::SocketAddr;

pub mod error;
mod message;
mod network;

pub use crate::error::{Error, Result};
use crate::message::{Message, MessageSerde};
use crate::network::{Endpoint, Event, Handler, NetworkIO};

pub struct NetworkInterface<Command>
where
    Command: Send + 'static,
{
    addr: SocketAddr,
    io: NetworkIO<Command>,
}

impl NetworkInterface<()> {
    #[must_use]
    pub const fn build() -> Builder<NoAddr, NoAddr> {
        Builder {
            host: NoAddr,
            connect: NoAddr,
        }
    }
}

#[derive(Clone)]
struct Addr(SocketAddr);
struct NoAddr;

pub struct Builder<HostOn, ConnectTo> {
    host: HostOn,
    connect: ConnectTo,
}

impl Builder<NoAddr, NoAddr> {
    #[must_use]
    pub const fn connect_to(self, remote: SocketAddr) -> Builder<NoAddr, Addr> {
        Builder {
            host: NoAddr,
            connect: Addr(remote),
        }
    }

    #[must_use]
    pub const fn host_on(self, local: SocketAddr) -> Builder<Addr, NoAddr> {
        Builder {
            host: Addr(local),
            connect: NoAddr,
        }
    }
}

// Server
impl Builder<Addr, NoAddr> {
    #[must_use]
    pub fn as_client(self) -> Builder<Addr, Addr> {
        Builder {
            host: self.host.clone(),
            connect: Addr(self.host.0),
        }
    }

    pub fn run(self) -> Result<NetworkInterface<()>> {
        let io = NetworkIO::startup(PlaceHolderHandler(None));
        let addr = io
            .listen(self.host.0)
            .map_err(|err| Error::ListenOn(self.host.0, err))?;
        Ok(NetworkInterface { addr, io })
    }
}

// Client
impl Builder<NoAddr, Addr> {
    pub fn run(self) -> Result<NetworkInterface<()>> {
        let io = NetworkIO::startup(PlaceHolderHandler(None));
        let addr = io
            .connect(self.connect.0)
            .map_err(|err| Error::ConnectTo(self.connect.0, err))?;
        Ok(NetworkInterface { addr, io })
    }
}

// ClientServer
impl Builder<Addr, Addr> {
    pub fn run(self) -> Result<NetworkInterface<()>> {
        let io = NetworkIO::startup(PlaceHolderHandler(None));
        let addr = io
            .listen(self.host.0)
            .map_err(|err| Error::ListenOn(self.host.0, err))?;
        io.connect(addr)
            .map_err(|err| Error::ConnectTo(self.connect.0, err))?;
        Ok(NetworkInterface { addr, io })
    }
}

// WIP: remove
struct PlaceHolderHandler(Option<Endpoint>);

impl Handler<()> for PlaceHolderHandler {
    fn handle_net(&mut self, _io: &NetworkIO<()>, _event: Event) {}
    fn command_handler(&mut self, _io: &NetworkIO<()>, _command: ()) {}
}
