use std::{net::SocketAddr, time::Duration};

use super::Handler;
use crate::{
    networking::{ListenerLifetime, NetworkController},
    ArmaEvent, InstanceManager, NetEntityId,
};

#[derive(Debug)]
pub enum Command {
    RequestEntityNetId,
    SendArmaEvent(ArmaEvent),

    Disconnect,
    PingLoop(Duration),
}

/// Client network manager.
#[must_use]
pub struct Manager {
    _lifetime: ListenerLifetime,
    controller: NetworkController<Handler>,
    addr: SocketAddr,
    server_addr: SocketAddr,
}

impl InstanceManager for Manager {
    #[inline]
    fn addr(&self) -> std::net::SocketAddr {
        self.addr
    }

    #[inline]
    fn server_addr(&self) -> std::net::SocketAddr {
        self.server_addr
    }

    #[inline]
    fn disconnect(&self) {
        self.controller.command(Command::Disconnect, None);
    }

    #[inline]
    fn stop(&self) {
        self.controller.stop();
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        self.stop();
    }
}

impl Manager {
    pub(crate) const fn new(
        lifetime: ListenerLifetime,
        controller: NetworkController<Handler>,
        addr: SocketAddr,
        server_addr: SocketAddr,
    ) -> Self {
        Self {
            _lifetime: lifetime,
            controller,
            addr,
            server_addr,
        }
    }

    /// Reserve a unused unique entity network id, the id is returned by [`ClientOutput::EntityNetId`].
    pub fn reserve_net_id(&self) {
        self.controller.command(Command::RequestEntityNetId, None);
    }

    /// Broadcast an arma event over the network.
    pub fn arma_event(&self, name: &str, data: arma_rs::Value) {
        let command = Command::SendArmaEvent(ArmaEvent::Event {
            name: name.to_owned(),
            params: data,
        });
        self.controller.command(command, None);
    }

    /// Broadcast an arma entity event over the network.
    pub fn arma_entity_event(&self, net_id: NetEntityId, name: &str, data: arma_rs::Value) {
        let command = Command::SendArmaEvent(ArmaEvent::EntityEvent {
            id: net_id,
            name: name.to_owned(),
            params: data,
        });
        self.controller.command(command, None);
    }
}
