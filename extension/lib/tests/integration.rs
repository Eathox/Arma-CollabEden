use std::{
    collections::BTreeSet,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use crossbeam_channel::Receiver;
use pretty_assertions::{assert_eq, assert_matches};

use coden::{
    ClientManager, ClientOutput, InstanceManager, LocalEntityId, ManagerBuilder, NetEntityId,
    ServerManager, ServerOutput,
};

const LOCAL_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
const TIMEOUT: Duration = Duration::from_millis(100);

#[allow(clippy::type_complexity)]
fn setup_test_network(
    client_count: usize,
) -> (
    (ServerManager, Receiver<ServerOutput>),
    Vec<(ClientManager, Receiver<ClientOutput>)>,
) {
    let (server, server_out) = ManagerBuilder::new()
        .host_on(LOCAL_ADDR)
        .disable_ping()
        .startup()
        .unwrap();

    let mut clients = Vec::with_capacity(client_count);
    let client_builder = ManagerBuilder::new()
        .connect_to(server.server_addr())
        .disable_ping();

    for _ in 0..client_count {
        let (client, client_out) = client_builder.startup().unwrap();
        assert_matches!(
            recv_output(&server_out),
            ServerOutput::ClientConnected(conn) if conn.addr() == client.addr()
        );
        assert_eq!(
            recv_output(&client_out),
            ClientOutput::ServerConnected(true)
        );

        clients.push((client, client_out));
    }

    ((server, server_out), clients)
}

fn recv_output<O>(out: &Receiver<O>) -> O {
    out.recv_timeout(TIMEOUT).unwrap()
}

#[test]
fn reserve_net_id() {
    const CLIENT_COUNT: u32 = 50;

    let ((_server, _), clients) = setup_test_network(CLIENT_COUNT as usize);
    for (client, _) in &clients {
        client.reserve_net_id();
    }

    let mut ids = BTreeSet::new();
    for (client, out) in clients.iter() {
        assert_matches!(recv_output(out), ClientOutput::EntityNetId(id) if ids.insert(id));
        client.stop(); // Speeds up the test
    }

    assert_eq!(ids.last(), Some(&NetEntityId::new(CLIENT_COUNT - 1)));
}

#[test]
fn arma_event() {
    let ((server, server_out), mut clients) = setup_test_network(2);
    let (client_a, client_a_out) = clients.pop().unwrap();
    let (_client_b, client_b_out) = clients.pop().unwrap();

    client_a.arma_event("test", arma_rs::Value::Null);
    assert_eq!(
        recv_output(&client_b_out),
        ClientOutput::ArmaEvent {
            name: "test".to_string(),
            params: arma_rs::Value::Null
        }
    );

    client_a.disconnect();
    assert_matches!(
        recv_output(&server_out),
        ServerOutput::ClientDisconnected(conn) if conn.addr() == client_a.addr()
    );

    server.disconnect();
    assert_matches!(recv_output(&client_b_out), ClientOutput::ServerDisconnected);

    assert_eq!(server_out.len(), 0);
    assert_eq!(client_a_out.len(), 0);
    assert_eq!(client_b_out.len(), 0);
}

#[test]
fn arma_entity_event() {
    let ((_server, server_out), mut clients) = setup_test_network(2);
    let (mut client_a, client_a_out) = clients.pop().unwrap();
    let (mut client_b, client_b_out) = clients.pop().unwrap();

    client_a.reserve_net_id();
    let ClientOutput::EntityNetId(net_id) = recv_output(&client_a_out) else {
        panic!("Event should be an EntityNetId");
    };

    let client_b_local_id = LocalEntityId::new(1);
    assert_eq!(client_a.get_net_id(client_b_local_id), None);

    client_a.add_entity(net_id, client_b_local_id);
    assert_eq!(client_a.get_net_id(client_b_local_id), Some(net_id));

    client_a.arma_entity_event(net_id, "created", arma_rs::Value::Null);
    assert_eq!(
        recv_output(&client_b_out),
        ClientOutput::ArmaEntityEvent {
            id: net_id,
            name: "created".to_string(),
            params: arma_rs::Value::Null
        }
    );

    let client_a_local_id = LocalEntityId::new(99);
    assert_eq!(client_b.get_net_id(client_a_local_id), None);

    client_b.add_entity(net_id, client_a_local_id);
    assert_eq!(client_b.get_net_id(client_a_local_id), Some(net_id));

    assert_eq!(client_a.get_local_id(net_id), Some(client_b_local_id));
    assert_eq!(client_b.get_local_id(net_id), Some(client_a_local_id));

    assert_eq!(server_out.len(), 0);
    assert_eq!(client_a_out.len(), 0);
    assert_eq!(client_b_out.len(), 0);
}
