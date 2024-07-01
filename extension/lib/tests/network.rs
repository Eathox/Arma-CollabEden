use pretty_assertions::{assert_eq, assert_matches};

use coden::{ClientOutput, InstanceManager, ManagerBuilder, ServerOutput};

mod common;
use common::{recv, LOCAL_ADDR};

#[test]
fn connect() {
    let (server, server_out) = ManagerBuilder::default()
        .host_on(LOCAL_ADDR)
        .with_ping(None)
        .startup()
        .unwrap();

    let (client, client_out) = ManagerBuilder::default()
        .connect_to(server.addr())
        .with_ping(None)
        .startup()
        .unwrap();

    assert_matches!(
        recv(&server_out),
        ServerOutput::ClientConnected(conn) if conn.addr() == client.addr()
    );
    assert_eq!(recv(&client_out), ClientOutput::ServerConnected(true));
}

#[test]
fn disconnect() {
    let (server, server_out) = ManagerBuilder::default()
        .host_on(LOCAL_ADDR)
        .with_ping(None)
        .startup()
        .unwrap();

    let (client, client_out) = ManagerBuilder::default()
        .connect_to(server.addr())
        .with_ping(None)
        .startup()
        .unwrap();

    assert_matches!(
        recv(&server_out),
        ServerOutput::ClientConnected(conn) if conn.addr() == client.addr()
    );
    assert_eq!(recv(&client_out), ClientOutput::ServerConnected(true));
}
