use std::{net::SocketAddr, time::Duration};

use super::Handler;
use crate::{
    networking::{ListenerLifetime, NetworkController},
    InstanceManager,
};

#[derive(Debug)]
pub enum Command {
    Shutdown,
    PingLoop(Duration),
}

/// Dedicated server network manager.
#[must_use]
pub struct Manager {
    _lifetime: ListenerLifetime,
    controller: NetworkController<Handler>,
    addr: SocketAddr,
}

impl InstanceManager for Manager {
    #[inline]
    fn addr(&self) -> std::net::SocketAddr {
        self.addr
    }

    #[inline]
    fn server_addr(&self) -> std::net::SocketAddr {
        self.addr
    }

    #[inline]
    fn stop(&self) {
        self.controller.command(Command::Shutdown, None);
    }

    #[inline]
    fn force_stop(&self) {
        self.controller.stop();
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        self.force_stop();
    }
}

impl Manager {
    pub(crate) const fn new(
        lifetime: ListenerLifetime,
        controller: NetworkController<Handler>,
        addr: SocketAddr,
    ) -> Self {
        Self {
            _lifetime: lifetime,
            controller,
            addr,
        }
    }
}
