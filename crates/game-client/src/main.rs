use std::net::UdpSocket;
use std::time::{Duration, SystemTime};

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_renet::client_connected;
use bevy_renet::netcode::{
    ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeErrorEvent,
};
use bevy_renet::renet::ConnectionConfig;
use bevy_renet::{RenetClient, RenetClientPlugin};
use shared::net::protocol_id_from_version;

const SERVER_ADDR: &str = "127.0.0.1:5000";
const TICK_RATE: f64 = 60.0;

fn main() {
    let (client, transport) = new_client();

    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / TICK_RATE,
            ))),
        )
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .insert_resource(client)
        .insert_resource(transport)
        .insert_resource(ClientBootState::default())
        .add_systems(Update, log_client_started)
        .add_systems(Update, log_connected.run_if(client_connected))
        .add_systems(Update, log_disconnected)
        .add_observer(handle_netcode_error)
        .run();
}

#[derive(Resource, Default)]
struct ClientBootState {
    announced: bool,
    connected_once: bool,
}

fn new_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = SERVER_ADDR.parse().expect("invalid server address");
    let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind client UDP socket");
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system clock before unix epoch");
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: protocol_id_from_version(env!("CARGO_PKG_VERSION")),
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket)
        .expect("failed to create netcode client transport");
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

fn log_client_started(mut boot_state: ResMut<ClientBootState>) {
    if boot_state.announced {
        return;
    }

    println!("game-client connecting to udp://{SERVER_ADDR}");
    boot_state.announced = true;
}

fn log_connected(mut boot_state: ResMut<ClientBootState>) {
    if boot_state.connected_once {
        return;
    }

    boot_state.connected_once = true;
    println!("game-client connected");
}

fn log_disconnected(
    mut exit: MessageWriter<AppExit>,
    boot_state: Res<ClientBootState>,
    client: Option<Res<RenetClient>>,
) {
    if !boot_state.connected_once {
        return;
    }

    let Some(client) = client else {
        return;
    };

    if client.is_disconnected() {
        println!("game-client disconnected");
        exit.write(AppExit::Success);
    }
}

fn handle_netcode_error(netcode_error: On<NetcodeErrorEvent>) {
    eprintln!("game-client netcode error: {}", **netcode_error);
}
