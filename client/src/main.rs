mod level;
mod network;

use bevy::prelude::*;
use bevy::window::PresentMode;
use network::components::*;
use rand::Rng;
use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};

use crate::level::components::*;
use crate::level::systems::*;
use crate::network::systems::*;

fn main() {
    println!("Starting Client");

    //connect to server
    let udp_addr = "127.0.0.1:4000";
    let tcp_addr = "127.0.0.1:8000";

    let udp_socket = UdpSocket::bind(udp_addr).unwrap();

    println!(
        "UDP: Client listening on {}",
        udp_socket.local_addr().unwrap()
    );

    let mut buf = [0; 1024];

    //starting tcp connection with server
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

    let mut ocean = Vec::new();

    loop {
        let result = udp_socket.recv_from(&mut buf);

        match result {
            Ok((size, src)) => {
                let deserialize: Packet<OceanTile> = serde_json::from_slice(&buf[..size]).unwrap();

                ocean.push(deserialize.payload);

                //let result = socket.send_to(&buf[..size], "127.0.0.1:8000");
                if ocean.len() >= 15625 {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Something happened: {}", e);
            }
        }
    }

    println!("Ocean map done");

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
        .add_systems(Startup, setup)
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

    if ocean.map.len() >= 15625 {
        println!("Packets done: {} packets", ocean.map.len());
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

/*

    if (ocean.map.len() >= 100000) {
        println!("Hit");
        let mut rng = rand::thread_rng();
        let mut tile_index;

        let mut w = 0;
        let mut h = 0;
        let mut t = Vec3::new(
            -OCEAN_W_CENTER + TILE_SIZE as f32 / 2.,
            -OCEAN_H_CENTER + TILE_SIZE as f32 / 2.,
            -1.0,
        );

        let bg_ocean_texture_handle: Handle<Image> = asset_server.load("ts_ocean_tiles.png");
        let ocean_layout =
            TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE * 2), 2, 1, None, None);
        let ocean_layout_handle = texture_atlases.add(ocean_layout);

        while (h as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_H {
            while (w as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_W {
                // weigh it so that its mostly dark blue just for aesthetic reasons
                let rand = rng.gen_range(0..=10);
                if rand < 9 {
                    tile_index = 0
                } else {
                    tile_index = 1
                }

                commands.spawn((
                    SpriteBundle {
                        texture: bg_ocean_texture_handle.clone(),
                        transform: Transform {
                            translation: t,
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: ocean_layout_handle.clone(),
                        index: tile_index,
                    },
                ));
                w += 1;
                t += Vec3::new((TILE_SIZE * 2) as f32, 0., 0.);
            }

            w = 0;
            t += Vec3::new(0., (TILE_SIZE * 2) as f32, 0.);
            t.x = -OCEAN_W_CENTER + (TILE_SIZE * 2) as f32 / 2.0;
            h += 1;
        }
    }
*/
