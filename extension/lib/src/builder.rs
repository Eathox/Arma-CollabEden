use std::net::SocketAddr;

use crate::{
    handlers::{client, server},
    network::NetworkController,
    ClientManager, CommonManager, OutputReceiver, OutputSender, Result, ServerManager,
};

pub struct Connect(SocketAddr);
pub struct Host(SocketAddr);
pub struct None;

/// Builder for [`InstanceManager`].
///
/// [`InstanceManager`]: crate::InstanceManager
pub struct ManagerBuilder<H, C> {
    host: H,
    connect: C,
    ping_enabled: bool,
}

impl Default for ManagerBuilder<None, None> {
    /// Start building a new network manager.
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            host: None,
            connect: None,
            ping_enabled: true,
        }
    }
}

impl ManagerBuilder<None, None> {
    /// Configure to be a client connecting to the given address.
    #[inline]
    #[must_use]
    pub const fn connect_to(self, remote: SocketAddr) -> ManagerBuilder<None, Connect> {
        ManagerBuilder {
            host: None,
            connect: Connect(remote),
            ping_enabled: self.ping_enabled,
        }
    }

    /// Configure to be a server hosting on the given address.
    #[inline]
    #[must_use]
    pub const fn host_on(self, local: SocketAddr) -> ManagerBuilder<Host, None> {
        ManagerBuilder {
            host: Host(local),
            connect: None,
            ping_enabled: self.ping_enabled,
        }
    }
}

// Any
impl<H, C> ManagerBuilder<H, C> {
    /// Disable constant ping loop.
    #[inline]
    #[must_use]
    pub fn disable_ping(self) -> Self {
        Self {
            ping_enabled: false,
            ..self
        }
    }
}

// Server
impl ManagerBuilder<Host, None> {
    /// Complete the configuration and boot up the server.
    /// Returns both the server manager and the mangers output receiver.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to listen on.
    #[inline]
    pub fn startup(&self) -> Result<(ServerManager, OutputReceiver<server::Output>)> {
        let (controller, listener) = NetworkController::new();
        let server_addr = controller.listen(self.host.0)?;

        let (sender, receiver) = OutputSender::new();
        let handler = server::Handler::new(controller.clone(), sender, self.ping_enabled);

        let manager = ServerManager::new(CommonManager {
            _lifetime: listener.start(handler),
            controller,
            addr: server_addr,
            server_addr,
        });
        Ok((manager, receiver))
    }
}

// Client
impl ManagerBuilder<None, Connect> {
    /// Complete the configuration and boot up the client.
    /// Returns both the client manager and the mangers output receiver.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to connect to.
    #[inline]
    pub fn startup(&self) -> Result<(ClientManager, OutputReceiver<client::Output>)> {
        let (controller, listener) = NetworkController::new();
        let (conn, addr) = controller.connect(self.connect.0)?;

        let (sender, receiver) = OutputSender::new();
        let handler = client::Handler::new(controller.clone(), sender, conn, self.ping_enabled);

        let manager = ClientManager::new(CommonManager {
            _lifetime: listener.start(handler),
            controller,
            addr,
            server_addr: conn.addr(),
        });
        Ok((manager, receiver))
    }
}
