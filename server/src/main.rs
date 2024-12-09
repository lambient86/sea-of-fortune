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
    let mut new = Enemies { list: Vec::new() };
    let mut update = Enemies { list: Vec::new() };
    let mut cooldowns = Cooldowns { list: Vec::new() };

    new.list.push(Enemy {
        id: 15,
        etype: KRAKEN,
        pos: Vec3::new(0., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.),
        animation_index: 0,
        alive: true,
        hp: KRAKEN_MAX_HP,
        target_id: -1,
    });

    update.list.push(Enemy {
        id: 15,
        etype: KRAKEN,
        pos: Vec3::new(0., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.),
        animation_index: 0,
        alive: true,
        hp: KRAKEN_MAX_HP,
        target_id: -1,
    });

    cooldowns.list.push(CD {
        enemy_id: 15,
        og: 2.5,
        timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
    });

    new.list.push(Enemy {
        id: 16,
        etype: GHOSTSHIP,
        pos: Vec3::new(200., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.),
        animation_index: 0,
        alive: true,
        hp: GHOSTSHIP_MAX_HP,
        target_id: -1,
    });

    update.list.push(Enemy {
        id: 16,
        etype: GHOSTSHIP,
        pos: Vec3::new(200., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.),
        animation_index: 0,
        alive: true,
        hp: GHOSTSHIP_MAX_HP,
        target_id: -1,
    });

    cooldowns.list.push(CD {
        enemy_id: 16,
        og: 2.5,
        timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
    });

    println!("Ocean size: {}", ocean_map.map.len());

    let result = UdpSocket::bind("0.0.0.0:5000");

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
            .insert_resource(EnemyLists {
                new,
                update,
                dead: Enemies { list: Vec::new() },
            })
            .insert_resource(projectiles)
            .insert_resource(UDP { socket: udp_socket })
            .insert_resource(cooldowns)
            .add_systems(Update, handle)
            .add_systems(Update, enemy_movement)
            //.add_systems(Update, enemy_proj_handle)
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
        println!("UDP Socket unsuccessfully bound: {}", result.err().unwrap());
        //3 sec cooldown between attempts
        std::thread::sleep(Duration::new(3, 0));
    }

    //.add_systems(Update);
}

pub fn handle(
    ocean: Res<OceanMap>,
    mut players: ResMut<Players>,
    udp: Res<UDP>,
    mut enemies: ResMut<EnemyLists>,
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
                                    create_env(
                                        "update_enemies".to_string(),
                                        enemies.update.clone(),
                                    )
                                    .as_bytes(),
                                    player.addr.clone(),
                                )
                                .expect("Failed to send [update_enemy] packet");

                            udp.socket
                                .send_to(
                                    create_env("new_enemies".to_string(), enemies.new.clone())
                                        .as_bytes(),
                                    player.addr.clone(),
                                )
                                .expect("Failed to send [update_enemy] packet");

                            /*udp.socket
                                .send_to(
                                    create_env(
                                        "update_projectiles".to_string(),
                                        projectiles.clone(),
                                    )
                                    .as_bytes(),
                                    player.addr.clone(),
                                )
                                .expect("Failed to send [update_projectiles] packet");
                            projectiles.list.clear();*/
                        }
                        enemies.new.list.clear();
                    }
                } else if env.message == "player_update" {
                    let packet: Packet<Player> = serde_json::from_str(&env.packet).unwrap();
                    let player = packet.payload;
                    let id = player.id;

                    players.player_array[id as usize].pos = player.pos;
                    players.player_array[id as usize].rot = player.rot;
                } else if env.message == "enemy_damaged" {
                    let packet: Packet<Damage> = serde_json::from_str(&env.packet).unwrap();
                    let attack = packet.payload;

                    let option = enemies
                        .update
                        .list
                        .iter()
                        .position(|x| x.id == attack.target_id);

                    match option {
                        Some(index) => {
                            enemies.update.list[index].hp -= attack.dmg;

                            println!(
                                "Enemy [{}] hp: [{}]",
                                enemies.update.list[index].id, enemies.update.list[index].hp
                            );

                            if enemies.update.list[index].hp <= 0. {
                                for player in players.player_array.iter() {
                                    if player.used {
                                        println!(
                                            "Sending enemy [{}] dead to player #{}",
                                            enemies.update.list[index].id, player.addr
                                        );
                                        let temp = enemies.update.list[index].clone();
                                        enemies.dead.list.push(temp);
                                    }
                                }

                                enemies.update.list.remove(index);
                            }
                        }
                        None => {}
                    }
                } else if env.message == "got_here_late" {
                    let packet: Packet<Player> = serde_json::from_str(&env.packet).unwrap();
                    let player = packet.payload;
                    println!(
                        "This happened for player #{}: Sending [{}] enemies",
                        player.id,
                        enemies.update.list.len()
                    );
                    udp.socket
                        .send_to(
                            create_env("new_enemies".to_string(), enemies.update.clone())
                                .as_bytes(),
                            player.addr.clone(),
                        )
                        .expect("Failed to send [update_enemy] packet");
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
    enemies: Res<EnemyLists>,
    mut projectiles: ResMut<Projectiles>,
    players: ResMut<Players>,
    mut cooldowns: ResMut<Cooldowns>,
    time: Res<Time>,
) {
    for enemy in enemies.update.list.iter() {
        let index = cooldowns
            .list
            .iter()
            .position(|x| x.enemy_id == enemy.id)
            .unwrap();

        cooldowns.list[index].timer.tick(time.delta());
        if !cooldowns.list[index].timer.finished() {
            continue;
        }

        cooldowns.list[index].timer =
            Timer::from_seconds(cooldowns.list[index].og, TimerMode::Once);

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
            if !player.used {
                continue;
            }
            let player_position = player.pos.xy();
            let enemy_position = enemy.pos.xy();

            let distance_to_player = enemy_position.distance(player_position);

            if distance_to_player > attack_dist {
                continue;
            }

            let original_direction = (player.pos - enemy.pos).normalize();
            let angle = original_direction.x.atan2(original_direction.y);
            let angle_direction = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();

            let mut projectile_start_pos = enemy.pos + angle_direction * 10.0;
            projectile_start_pos.z = 2.;

            let projectile = Projectile {
                owner_id: enemy.id,
                velocity: Velocity {
                    v: angle_direction.truncate() * speed,
                },
                translation: projectile_start_pos,
                lifetime: lifetime,
            };

            println!("Player #{} is in range of entity [{}]", player.id, enemy.id);
            projectiles.list.push(projectile);
            break;
        }
    }
}

pub fn enemy_movement(
    mut enemies: ResMut<EnemyLists>,
    mut projectiles: ResMut<Projectiles>,
    players: ResMut<Players>,
    mut cooldowns: ResMut<Cooldowns>,
    time: Res<Time>,
) {
    for enemy in enemies.update.list.iter_mut() {
        match enemy.etype {
            KRAKEN => {
                let kraken_translation = enemy.pos;

                for player in players.player_array.iter() {
                    let player_translation = player.pos;

                    //Gets positions (Vec2) of the entities
                    let player_position = player_translation.xy();
                    let kraken_position = kraken_translation.xy();

                    //Gets distance
                    let distance_to_player = kraken_position.distance(player_position);

                    //Check
                    if distance_to_player > KRAKEN_AGRO_RANGE
                        || distance_to_player <= KRAKEN_AGRO_STOP
                    {
                        continue;
                    }

                    //Gets direction projectile will be going
                    let direction = (player_translation - kraken_translation).normalize();
                    let velocity = direction * KRAKEN_MOVEMENT_SPEED;

                    //Moves kraken
                    enemy.pos += velocity * time.delta_seconds();
                    break;
                }
            }
            GHOSTSHIP => {
                let ghostship_translation = enemy.pos;

                for player in players.player_array.iter() {
                    let player_translation = player.pos;

                    //Gets positions (Vec2) of the entities
                    let player_position = player_translation.xy();
                    let ghostship_position = ghostship_translation.xy();

                    //Gets distance
                    let distance_to_player = ghostship_position.distance(player_position);

                    //Check
                    if distance_to_player > GHOSTSHIP_AGRO_RANGE
                        || distance_to_player <= GHOSTSHIP_AGRO_STOP
                    {
                        continue;
                    }

                    //Gets direction projectile will be going
                    let direction = (player_translation - ghostship_translation).normalize();
                    let velocity = direction * GHOSTSHIP_MOVEMENT_SPEED;

                    //Moves kraken
                    enemy.pos += velocity * time.delta_seconds();
                    break;
                }
            }
            _ => {
                println!("Undefined enemy type: {}", enemy.etype);
            }
        }
    }
}
