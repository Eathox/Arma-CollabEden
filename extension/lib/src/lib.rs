#![deny(missing_docs, clippy::all, clippy::nursery)]
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

//! Networking library to add multiplayer capabilities to Arma 3s Eden Editor.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use std::net::SocketAddr;

mod error;
mod message;
mod network;

pub use error::{Error, Result};
use message::{Message, MessageSerde};
use network::{Endpoint, Event, Handler, NetworkIO};

/// Manager responsible for networking.
/// Can be configured to either be either a server, client or a client hosted server.
pub struct Manager<H: Handler> {
    addr: SocketAddr,
    io: NetworkIO<H>,
}

impl<H: Handler> Manager<H> {
    /// Create a new network manager.
    #[allow(private_interfaces)]
    #[must_use]
    pub const fn builder() -> ManagerBuilder<NoAddr, NoAddr> {
        ManagerBuilder {
            host: NoAddr,
            connect: NoAddr,
        }
    }

    /// For clients the address its connected to, for servers the address its listening on.
    #[must_use]
    pub const fn server_addr(&self) -> SocketAddr {
        self.addr
    }
}

struct Addr(SocketAddr);
struct NoAddr;

/// Builder for [`Manager`].
pub struct ManagerBuilder<HostOn, ConnectTo> {
    host: HostOn,
    connect: ConnectTo,
}

impl ManagerBuilder<NoAddr, NoAddr> {
    /// Configure to be a client connecting to the given address.
    #[inline]
    #[must_use]
    pub const fn connect_to(self, remote: SocketAddr) -> ManagerBuilder<NoAddr, Addr> {
        ManagerBuilder {
            host: NoAddr,
            connect: Addr(remote),
        }
    }

    /// Configure to be a server hosting on the given address.
    #[inline]
    #[must_use]
    pub const fn host_on(self, local: SocketAddr) -> ManagerBuilder<Addr, NoAddr> {
        ManagerBuilder {
            host: Addr(local),
            connect: NoAddr,
        }
    }
}

// Server
impl ManagerBuilder<Addr, NoAddr> {
    /// Convert to a client hosted server.
    #[inline]
    #[must_use]
    pub const fn as_client(self) -> ManagerBuilder<Addr, Addr> {
        ManagerBuilder {
            host: Addr(self.host.0),
            connect: Addr(self.host.0),
        }
    }

    /// Complete the configuration and boot up the server.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to listen on.
    #[inline]
    pub fn startup(self) -> Result<Manager<PlaceHolderHandler>> {
        let io = NetworkIO::startup(PlaceHolderHandler(None));
        let addr = io
            .listen(self.host.0)
            .map_err(|err| Error::Listen(self.host.0, err))?;
        Ok(Manager { addr, io })
    }
}

// Client
impl ManagerBuilder<NoAddr, Addr> {
    /// Complete the configuration and boot up the client.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to connect to.
    #[inline]
    pub fn startup(self) -> Result<Manager<PlaceHolderHandler>> {
        let io = NetworkIO::startup(PlaceHolderHandler(None));
        let addr = io
            .connect(self.connect.0)
            .map_err(|err| Error::ConnectAttempt(self.connect.0, err))?;
        Ok(Manager { addr, io })
    }
}

// ClientServer
impl ManagerBuilder<Addr, Addr> {
    /// Complete the configuration and boot up the client server.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to listen on.
    #[inline]
    pub fn startup(self) -> Result<Manager<PlaceHolderHandler>> {
        let io = NetworkIO::startup(PlaceHolderHandler(None));
        let addr = io
            .listen(self.host.0)
            .map_err(|err| Error::Listen(self.host.0, err))?;
        io.connect(addr)
            // Should never happen since we just got a valid addr
            .map_err(|err| Error::ConnectAttempt(self.connect.0, err))?;
        Ok(Manager { addr, io })
    }
}

// WIP: remove
struct PlaceHolderHandler(Option<Endpoint>);

impl Handler for PlaceHolderHandler {
    type Command = ();

    fn handle_net(&mut self, _io: &NetworkIO<Self>, _event: Event) {}
    fn handle_message(&mut self, _io: &NetworkIO<Self>, _endpoint: Endpoint, _message: &Message) {}
    fn handle_command(&mut self, _io: &NetworkIO<Self>, _command: ()) {}
}
