mod data;
mod level;
mod network;

use bevy::prelude::*;
use bevy::window::PresentMode;
use data::gameworld_data::*;
use level::components::*;
use serde::Serialize;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::*;

use crate::level::systems::*;
use crate::network::components::*;
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
    println!("Starting Server");

    // Creating UDP socket connecion

    // Creating ocean level
    let ocean_map = OceanMap { map: build_ocean() };
    let projectiles = Projectiles { list: Vec::new() };
    let mut enemies = Enemies { list: Vec::new() };

    enemies.list.push(Enemy {
        id: 15,
        etype: KRAKEN,
        pos: Vec3::new(0., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.),
        animation_index: 0,
        alive: true,
    });

    println!("Ocean size: {}", ocean_map.map.len());

    let result = UdpSocket::bind("127.0.0.1:5000");

    if result.is_ok() {
        let udp_socket = result.unwrap();

        udp_socket.set_nonblocking(true).expect("Fail");

        println!(
            "UDP Socket listening to {}",
            udp_socket.local_addr().unwrap()
        );

        App::new()
            .insert_resource(ocean_map)
            .insert_resource(Counter::init())
            .insert_resource(Players::init())
            .insert_resource(enemies)
            .insert_resource(projectiles)
            .insert_resource(UDP { socket: udp_socket })
            .add_systems(Update, handle)
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Sea of Fortune | Build 0.2".into(),
                    resolution: (1280., 720.).into(),
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            }))
            .run();
    } else {
        println!("UDP Socket unsuccessfully bound");
        //3 sec cooldown between attempts
        std::thread::sleep(Duration::new(3, 0));
    }

    //.add_systems(Update);
}

pub fn handle(
    ocean: Res<OceanMap>,
    mut players: ResMut<Players>,
    udp: Res<UDP>,
    enemies: Res<Enemies>,
    mut projectiles: ResMut<Projectiles>,
) {
    loop {
        let mut buf = [0; 1024];

        let result = udp.socket.recv_from(&mut buf);

        match result {
            Ok((bytes, src)) => {
                let env: Envelope = serde_json::from_slice(&buf[..bytes]).unwrap();

                if env.message == "new_player" {
                    let packet: Packet<Player> = serde_json::from_str(&env.packet).unwrap();
                    let mut new_player = packet.payload;

                    println!("Player join request from [{}]", new_player.addr);

                    let mut index = 0;
                    let mut full = true;

                    for player in players.player_array.iter() {
                        if !player.used {
                            new_player.id = index;
                            new_player.used = true;
                            players.player_array[index as usize] = new_player.clone();
                            full = false;
                            break;
                        }
                        index += 1;
                    }

                    if full {
                        udp.socket
                            .send_to(
                                create_env(
                                    "full_lobby".to_string(),
                                    "Lobby is full, cannot join right now. Try again later!"
                                        .to_string(),
                                )
                                .as_bytes(),
                                new_player.addr,
                            )
                            .expect("Failed to send [full_lobby] packet");
                    } else {
                        //If lobby isn't full
                        udp.socket
                            .send_to(
                                create_env("joined_lobby".to_string(), new_player.id).as_bytes(),
                                new_player.addr.clone(),
                            )
                            .expect("Failed to send [id] response packet");

                        println!("Sending ocean overworld...");
                        let mut size = 0;
                        for tile in ocean.map.iter() {
                            size += 1;

                            let expect_msg = "Failed to send ocean tile packet #".to_string()
                                + &size.to_string();

                            udp.socket
                                .send_to(
                                    create_env("load_ocean".to_string(), tile.clone()).as_bytes(),
                                    new_player.addr.clone(),
                                )
                                .expect(&expect_msg);
                        }
                        println!("Done. Total ocean packets sent: {}", size);
                    }
                } else if env.message == "player_leave" {
                    let packet: Packet<Player> = serde_json::from_str(&env.packet).unwrap();

                    let player = packet.payload;
                    let id = player.id;
                    let addr = player.addr;

                    players.player_array[id as usize].used = false;

                    udp.socket
                        .send_to(
                            create_env("leave_success".to_string(), "null".to_string()).as_bytes(),
                            addr.clone(),
                        )
                        .expect("Failed to send [leave_success] packet");

                    println!("Logged out player");
                } else if env.message == "update" {
                    for player in players.player_array.iter() {
                        if player.used {
                            udp.socket
                                .send_to(
                                    create_env("update_players".to_string(), players.clone())
                                        .as_bytes(),
                                    player.addr.clone(),
                                )
                                .expect("Failed to send [update_player] packet");

                            udp.socket
                                .send_to(
                                    create_env("update_enemies".to_string(), enemies.clone())
                                        .as_bytes(),
                                    player.addr.clone(),
                                )
                                .expect("Failed to send [update_enemy] packet");

                            udp.socket
                                .send_to(
                                    create_env(
                                        "update_projectiles".to_string(),
                                        projectiles.clone(),
                                    )
                                    .as_bytes(),
                                    player.addr.clone(),
                                )
                                .expect("Failed to send [update_projectiles] packet");
                            projectiles.list.clear();
                        }
                    }
                } else if env.message == "player_update" {
                    let packet: Packet<Player> = serde_json::from_str(&env.packet).unwrap();
                    let player = packet.payload;
                    let id = player.id;

                    players.player_array[id as usize].pos = player.pos;
                    players.player_array[id as usize].rot = player.rot;
                } else {
                    println!(
                        "Recieved invalid packet from [{}]: {}",
                        src.ip(),
                        env.message
                    );
                }
            }
            Err(e) => {
                //println!("Listen: Something happened: {}", e);
                break;
            }
        }
    }
}

pub fn enemy_proj_handle(
    enemies: Res<Enemies>,
    mut projectiles: ResMut<Projectiles>,
    players: ResMut<Players>,
    udp: Res<UDP>,
    mut counter: ResMut<Counter>,
) {
    for enemy in enemies.list.iter() {
        let (attack_dist, lifetime, speed) = match enemy.etype {
            KRAKEN => (
                KRAKEN_ATTACK_DIST,
                KRAKEN_PROJECTILE_LIFETIME,
                KRAKEN_PROJECTILE_SPEED,
            ),
            GHOSTSHIP => (
                GHOSTSHIP_ATTACK_DIST,
                GHOSTSHIP_PROJECTILE_LIFETIME,
                GHOSTSHIP_PROJECTILE_SPEED,
            ),
            _ => {
                println!("Undefined enemy type for enemy_proj_hande()");
                (0., 0., 0.)
            }
        };

        for player in players.player_array.iter() {
            let player_position = player.pos.xy();
            let enemy_position = enemy.pos.xy();

            let distance_to_player = enemy_position.distance(player_position);

            if distance_to_player > attack_dist {
                continue;
            }

            let original_direction = (player.pos - enemy.pos).normalize();
            let angle = original_direction.x.atan2(original_direction.y);
            let angle_direction = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();

            let projectile_start_pos = enemy.pos + angle_direction * 10.0;

            let projectile = Projectile {
                owner_id: enemy.id,
                velocity: Velocity {
                    v: angle_direction.truncate() * speed,
                },
                translation: projectile_start_pos,
                lifetime: lifetime,
            };

            projectiles.list.push(projectile);
            break;
        }
    }
}
