mod level;
mod network;

use bevy::prelude::*;
use bevy::window::PresentMode;
use core::panic;
use network::components::*;
use rand::Rng;
use serde::*;
use std::net::*;
use std::sync::{Arc, Mutex};

use crate::level::components::*;
use crate::level::systems::*;
use crate::network::systems::*;

pub fn create_env<T: Serialize>(message: String, object: T) -> String {
    let packet: Packet<T> = Packet { payload: object };

    serde_json::to_string(&Envelope {
        message: message,
        packet: serde_json::to_string(&packet).unwrap(),
    })
    .unwrap()
}

fn main() {
    println!("Starting Client");

    //connect to server
    let udp_addr = "127.0.0.1:4000";
    //let tcp_addr = "127.0.0.1:8000";

    let udp_socket = UdpSocket::bind(udp_addr).unwrap();

    println!(
        "UDP: Client listening on {}",
        udp_socket.local_addr().unwrap()
    );

    let mut buf = [0; 1024];

    //starting tcp connection with server

    /*
    let mut tcp_stream = TcpStream::connect(tcp_addr);

    loop {
        match tcp_stream {
            Ok(ref t) => {
                println!("TCP: Stream connected!");
                break;
            }
            Err(ref e) => {
                eprintln!("Something happened: {}", e);
                tcp_stream = TcpStream::connect(tcp_addr);
            }
        }
    }
     */

    println!("Trying to join world...");

    let mut player = Player::default();
    player.addr = udp_socket.local_addr().unwrap().to_string();
    println!("Player addr = {}", player.addr);

    let packet: Packet<Player> = Packet {
        payload: player.clone(),
    };

    let env = serde_json::to_string(&Envelope {
        message: "new_player".to_string(),
        packet: serde_json::to_string(&packet).unwrap(),
    });

    udp_socket
        .send_to(env.unwrap().as_bytes(), "127.0.0.1:5000")
        .expect("Failed to send [new_player] packet");

    let mut ocean = Vec::new();

    loop {
        let result = udp_socket.recv_from(&mut buf);

        match result {
            Ok((size, src)) => {
                let env: Envelope = serde_json::from_slice(&buf[..size]).unwrap();

                if env.message.eq("joined_lobby") {
                    let packet: Packet<i32> = serde_json::from_str(&env.packet).unwrap();

                    let id = packet.payload;
                    println!("Joined lobby! You are player #{}", id);
                    player.id = id;
                } else if env.message.eq("full_lobby") {
                    panic!("{}", env.packet);
                } else if env.message.eq("load_ocean") {
                    let packet: Packet<OceanTile> = serde_json::from_str(&env.packet).unwrap();

                    ocean.push(packet.payload);
                } else {
                    println!("Recieved invalid packet");
                }

                if ocean.len() >= 15625 {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Something happened: {}", e);
            }
        }
    }

    println!("Ocean map done. Final size: {}", ocean.len());

    if !udp_socket.set_nonblocking(true).is_ok() {
        panic!("Non blocking wasn't successful; terminating");
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sea of Fortune | Build 0.2".into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(TcpConnections {
            streams: Vec::new(),
        })
        .insert_resource(UDP { socket: udp_socket })
        .insert_resource(Ocean { map: ocean })
        .insert_resource(HostPlayer { player: player })
        .add_systems(Startup, setup)
        .add_systems(Last, leave)
        /*.add_systems(Update, listen)*/
        .run();
    //.add_systems(Startup, listener);
}

pub fn listen(
    connections: Res<TcpConnections>,
    udp: Res<UDP>,
    mut ocean: ResMut<Ocean>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut buf = [0; 1024];

    let result = udp.socket.recv_from(&mut buf);

    match result {
        Ok((size, src)) => {
            //println!("Recieved {} bytes from {}", size, src);

            let json_str = String::from_utf8_lossy(&buf[..size]);
            //println!("Received JSON packet: {}", json_str);

            let deserialize: Packet<OceanTile> = serde_json::from_slice(&buf[..size]).unwrap();

            //let result = socket.send_to(&buf[..size], "127.0.0.1:8000");
        }
        Err(e) => {
            eprintln!("Something happened: {}", e);
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ocean: Res<Ocean>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    let bg_ocean_texture_handle: Handle<Image> = asset_server.load("ts_ocean_tiles.png");
    let ocean_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE * 2), 2, 1, None, None);
    let ocean_layout_handle = texture_atlases.add(ocean_layout);

    for tile in &ocean.map {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("ts_ocean_tiles.png"),
                transform: Transform {
                    translation: tile.translation,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: ocean_layout_handle.clone(),
                index: tile.tile_index,
            },
        ));
    }
}

fn leave(
    exit_events: EventReader<AppExit>,
    mut exit_triggered: Local<bool>,
    udp: Res<UDP>,
    player: Res<HostPlayer>,
) {
    if !*exit_triggered && exit_events.len() > 0 {
        *exit_triggered = true;

        udp.socket
            .send_to(
                create_env("player_leave".to_string(), player.player.clone()).as_bytes(),
                "127.0.0.1:5000",
            )
            .expect("Failed to send [player_leave]] packet");

        let mut buf = [0; 1024];

        udp.socket.set_nonblocking(false).expect("fail");

        loop {
            let result = udp.socket.recv_from(&mut buf);

            match result {
                Ok((bytes, src)) => {
                    let env: Envelope = serde_json::from_slice(&buf[..bytes]).unwrap();

                    if env.message.eq("leave_success") {
                        println!("Leave success");
                        break;
                    }
                }
                Err(e) => {}
            }
        }
    }
}
