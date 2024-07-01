use std::{net::SocketAddr, time::Duration};

use message_io::{
    network::Transport,
    node::{self, NodeHandler},
};

use super::{ConnectionId, InternalMessage, NetworkHandler, NetworkListener, NetworkSerde};
use crate::{Error, Result};

/// Controller used to connect, remove and send messages over the network, can safely be shared between threads.
pub struct NetworkController<H: NetworkHandler>(NodeHandler<H::Command>);

impl<H: NetworkHandler> Clone for NetworkController<H> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<H: NetworkHandler> NetworkController<H> {
    pub fn new() -> (Self, NetworkListener<H>) {
        let (node, listener) = node::split::<H::Command>();
        (Self(node), NetworkListener::new(listener))
    }

    /// Listen on the given address. Returns the actual address listening on, port can be passed `0` for automatic selection.
    ///
    /// # Errors
    /// Returns an error if unable to listen on the given address.
    pub fn listen(&self, addr: SocketAddr) -> Result<SocketAddr> {
        let (_, new_addr) = self
            .0
            .network()
            .listen(Transport::FramedTcp, addr)
            .map_err(|err| Error::Listen(addr, err))?;
        let new_addr = SocketAddr::new(addr.ip(), new_addr.port()); // Fix bug where sometimes 0.0.0.0 is returned with automatic port selection
        Ok(new_addr)
    }

    /// Connect to the given address. Returns id used to identify the connection and own address its connected from.
    ///
    /// # Errors
    /// Returns an error if the address couldn't be used for a connection attempt.\
    /// *Note: this will never error if the connection failed, instead that is reflected in [`NetworkEvent::ConnectionAttempt`].*
    pub fn connect(&self, addr: SocketAddr) -> Result<(ConnectionId, SocketAddr)> {
        let (conn, new_addr) = self
            .0
            .network()
            .connect(Transport::FramedTcp, addr)
            .map_err(|err| Error::ConnectAttempt(addr, err))?;
        let new_addr = SocketAddr::new(addr.ip(), new_addr.port()); // Sometimes automatic port selection has an ip of 0.0.0.0
        Ok((ConnectionId(conn), new_addr))
    }

    /// Remove the given connection. Does not emit a [`NetworkEvent::ConnectionLost`] to the local event loop.
    ///
    /// Returns `false` if the connection is already removed.
    pub fn remove(&self, conn: ConnectionId) -> bool {
        let id = conn.0.resource_id();
        if self.0.network().is_ready(id) == Some(true) {
            self.send_internal(conn, &InternalMessage::Disconnected);
            self.0.network().remove(id)
        } else {
            false
        }
    }

    pub fn send(&self, conn: ConnectionId, message: H::SendMessage) {
        self.send_internal(conn, &InternalMessage::Handler(message));
    }

    fn send_internal(&self, conn: ConnectionId, message: &InternalMessage<H::SendMessage>) {
        match message.to_net() {
            Ok(bytes) => {
                self.0.network().send(conn.0, &bytes);
            }
            Err(err) => error!("failed to send message: {err}"),
        }
    }

    pub fn command(&self, command: H::Command, delay: Option<Duration>) {
        let signals = self.0.signals();
        if let Some(delay) = delay {
            signals.send_with_timer(command, delay);
        } else {
            signals.send(command);
        }
    }

    /// Shut down the controller's corresponding [`NetworkListener`].
    /// Has no effect if the listener isn't running.
    pub fn stop(&self) {
        self.0.stop();
    }
}
