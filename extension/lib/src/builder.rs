use std::{net::SocketAddr, time::Duration};

use crate::{client, networking::NetworkController, server, OutputReceiver, OutputSender, Result};

pub struct Connect(SocketAddr);
pub struct Host(SocketAddr);
pub struct None;

/// Builder for [`InstanceManager`].
///
/// [`InstanceManager`]: crate::InstanceManager
pub struct ManagerBuilder<H, C> {
    host: H,
    connect: C,
    ping: Option<Duration>,
}

impl Default for ManagerBuilder<None, None> {
    /// Start building a new network manager.
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            host: None,
            connect: None,
            ping: Some(Duration::from_millis(500)),
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
            ping: self.ping,
        }
    }

    /// Configure to be a server hosting on the given address.
    #[inline]
    #[must_use]
    pub const fn host_on(self, local: SocketAddr) -> ManagerBuilder<Host, None> {
        ManagerBuilder {
            host: Host(local),
            connect: None,
            ping: self.ping,
        }
    }
}

// Any
impl<H, C> ManagerBuilder<H, C> {
    /// Change the ping interval. Set to `None` to disable the ping loop. Defaults to 500ms.
    #[inline]
    #[must_use]
    pub fn with_ping(self, interval: Option<Duration>) -> Self {
        Self {
            ping: interval,
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
    pub fn startup(&self) -> Result<(server::Manager, OutputReceiver<server::Output>)> {
        let (controller, listener) = NetworkController::new();
        let server_addr = controller.listen(self.host.0)?;

        let (sender, receiver) = OutputSender::new();
        let handler = server::Handler::new(controller.clone(), sender, self.ping);

        let lifetime = listener.start(handler);
        let controller = server::Manager::new(lifetime, controller, server_addr);
        Ok((controller, receiver))
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
    pub fn startup(&self) -> Result<(client::Manager, OutputReceiver<client::Output>)> {
        let (controller, listener) = NetworkController::new();
        let (conn, addr) = controller.connect(self.connect.0)?;

        let (sender, receiver) = OutputSender::new();
        let handler = client::Handler::new(controller.clone(), sender, conn, self.ping);

        let lifetime = listener.start(handler);
        let controller = client::Manager::new(lifetime, controller, addr, conn.addr());
        Ok((controller, receiver))
    }
}
