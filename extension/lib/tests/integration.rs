use std::collections::BTreeSet;

use pretty_assertions::{assert_eq, assert_matches};

use coden::{ClientOutput, InstanceManager, ServerOutput};

mod common;

use common::{recv, setup_test_network};

#[test]
fn ping() {
    let ((_server, server_out), mut clients) = setup_test_network(1, true);
    let (client_a, client_a_out) = clients.pop().unwrap();

    assert_matches!(
        recv(&server_out),
        ServerOutput::Ping(conn, _) if conn.addr() == client_a.addr()
    );

    assert_matches!(recv(&client_a_out), ClientOutput::Ping(_));

    assert_eq!(server_out.len(), 0);
    assert_eq!(client_a_out.len(), 0);
}

#[test]
fn reserve_net_id() {
    const CLIENT_COUNT: u32 = 50;

    let ((_server, _), clients) = setup_test_network(CLIENT_COUNT as usize, false);
    for (client, _) in &clients {
        client.reserve_net_id();
    }

    let mut ids = BTreeSet::new();
    for (client, out) in clients.iter() {
        assert_matches!(recv(out), ClientOutput::ReservedNetId(id) if ids.insert(id));
        client.stop(); // Speeds up the test
    }

    assert_eq!(ids.last().map(|&id| id.id()), Some(CLIENT_COUNT - 1));
}

#[test]
fn arma_event() {
    let ((_server, server_out), mut clients) = setup_test_network(3, false);
    let (_client_c, client_c_out) = clients.pop().unwrap();
    let (_client_b, client_b_out) = clients.pop().unwrap();
    let (client_a, client_a_out) = clients.pop().unwrap();

    client_a.arma_event("test", arma_rs::Value::Null);
    for out in [&client_b_out, &client_c_out] {
        assert_eq!(
            recv(out),
            ClientOutput::ArmaEvent {
                name: "test".to_string(),
                params: arma_rs::Value::Null
            }
        );
    }

    assert_eq!(server_out.len(), 0);
    assert_eq!(client_a_out.len(), 0);
    assert_eq!(client_b_out.len(), 0);
}

#[test]
fn arma_entity_event() {
    let ((_server, server_out), mut clients) = setup_test_network(3, false);
    let (_client_c, client_c_out) = clients.pop().unwrap();
    let (_client_b, client_b_out) = clients.pop().unwrap();
    let (client_a, client_a_out) = clients.pop().unwrap();

    client_a.reserve_net_id();
    let ClientOutput::ReservedNetId(net_id) = recv(&client_a_out) else {
        panic!("Event should be an EntityNetId");
    };

    client_a.arma_entity_event(net_id, "created", arma_rs::Value::Null);
    for out in [&client_b_out, &client_c_out] {
        assert_eq!(
            recv(out),
            ClientOutput::ArmaEntityEvent {
                id: net_id,
                name: "created".to_string(),
                params: arma_rs::Value::Null
            }
        );
    }

    assert_eq!(server_out.len(), 0);
    assert_eq!(client_a_out.len(), 0);
    assert_eq!(client_b_out.len(), 0);
    assert_eq!(client_c_out.len(), 0);
}
