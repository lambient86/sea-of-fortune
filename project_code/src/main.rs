mod bat;
mod boat;
mod components;
mod controls;
mod data;
mod enemies;
mod ghost_ship;
mod hitbox_system;
mod kraken;
mod level;
mod network;
mod player;
mod shop;
mod skeleton;
mod systems;
mod transition_box;
mod wfc;

use bat::BatPlugin;
use bevy::asset;
use bevy::{prelude::*, window::PresentMode};
use boat::components::Boat;
use boat::systems::*;
use boat::BoatPlugin;
use components::*;
use controls::*;
use data::gameworld_data::*;
use enemies::*;
use ghost_ship::components::*;
use ghost_ship::GhostShipPlugin;
use hitbox_system::*;
use kraken::components::*;
use kraken::KrakenPlugin;
use level::components::*;
use level::LevelPlugin;
use player::components::AttackCooldown;
use player::systems::*;
use player::PlayerPlugin;
use shop::ShopPlugin;
use skeleton::SkeletonPlugin;
use systems::*;
use wfc::WFCPlugin;

use std::net::*;
use std::time::Duration;

use network::components::*;
use network::systems::*;

fn main() {
    println!("Starting Client");

    //connect to server
    let udp_addr = "127.0.0.1:0";
    //let tcp_addr = "127.0.0.1:8000";

    let udp_socket = UdpSocket::bind(udp_addr).unwrap();

    println!(
        "UDP: Client listening on {}",
        udp_socket.local_addr().unwrap()
    );

    let mut ocean = Vec::new();
    let mut player = Player::default();

    let mut joined = false;
    loop {
        let mut buf = [0; 1024];

        if !joined {
            println!("Trying to join world...");

            player.addr = udp_socket.local_addr().unwrap().to_string();
            println!("Player addr = {}", player.addr);

            udp_socket
                .send_to(
                    create_env("new_player".to_string(), player.clone()).as_bytes(),
                    "127.0.0.1:5000",
                )
                .expect("Failed to send [new_player] packet");
        }

        let result = udp_socket.recv_from(&mut buf);

        match result {
            Ok((size, src)) => {
                let env: Envelope = serde_json::from_slice(&buf[..size]).unwrap();

                if env.message.eq("joined_lobby") {
                    let packet: Packet<i32> = serde_json::from_str(&env.packet).unwrap();

                    let id = packet.payload;
                    println!("Joined lobby! You are player #{}", id);
                    player.id = id;
                    joined = true;
                } else if env.message.eq("full_lobby") {
                    panic!("{}", env.packet);
                } else if env.message.eq("load_ocean") {
                    let packet: Packet<OceanT> = serde_json::from_str(&env.packet).unwrap();

                    ocean.push(packet.payload);
                } else {
                    println!("Recieved invalid packet");
                }

                if ocean.len() >= OCEAN_LENGTH as usize {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Something happened: {}", e);
            }
        }
        if !joined {
            std::thread::sleep(Duration::from_secs(3));
        }
    }

    println!("Ocean map done. Final size: {}", ocean.len());

    if !udp_socket.set_nonblocking(true).is_ok() {
        panic!("Non blocking wasn't successful; terminating");
    }

    App::new()
        .insert_resource(UDP { socket: udp_socket })
        .insert_resource(Ocean { map: ocean })
        .insert_resource(HostPlayer { player: player })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sea of Fortune | Build 0.2".into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CurrMousePos>()
        .add_systems(Startup, setup_gameworld)
        .add_plugins(PlayerPlugin)
        .add_plugins(BoatPlugin)
        .add_plugins(BatPlugin)
        .add_plugins(KrakenPlugin)
        .add_plugins(SkeletonPlugin)
        .add_plugins(HitboxPlugin)
        .add_plugins(ShopPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(WFCPlugin)
        .add_plugins(GhostShipPlugin)
        .add_systems(
            Update,
            move_player_camera.after(move_player).run_if(
                in_state(GameworldState::Island).or_else(in_state(GameworldState::Dungeon)),
            ),
        )
        .add_systems(
            Update,
            move_boat_camera
                .after(move_boat)
                .run_if(in_state(GameworldState::Ocean)),
        )
        .add_systems(Update, change_gameworld_state)
        .add_systems(Update, change_game_state)
        .add_systems(Update, update_mouse_pos)
        .insert_state(GameworldState::MainMenu)
        .insert_state(GameState::Running)
        .add_systems(Update, update.run_if(in_state(GameworldState::Ocean)))
        .add_systems(Last, leave)
        .run();
}

pub fn update(
    udp: Res<UDP>,
    host: Res<HostPlayer>,
    mut player_query: Query<(&mut Transform, &Boat), With<Boat>>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy, Entity), (With<EnemyTag>, Without<Boat>)>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
) {
    udp.socket
        .send_to(
            create_env("update".to_string(), "null".to_string()).as_bytes(),
            "127.0.0.1:5000",
        )
        .expect("Failed to send [update] packet");

    let mut buf = [0; 1024];

    let result = udp.socket.recv_from(&mut buf);

    match result {
        Ok((bytes, src)) => {
            let env: Envelope = serde_json::from_slice(&buf[..bytes]).unwrap();

            if env.message == "update_players" {
                let packet: Packet<Players> = serde_json::from_str(&env.packet).unwrap();
                let players = packet.payload;

                for p in players.player_array.iter() {
                    if p.id == host.player.id || !p.used {
                        continue;
                    }

                    let mut boat_found = false;

                    for (mut transform, player) in player_query.iter_mut() {
                        if player.id == host.player.id {
                            continue;
                        }
                        boat_found = true;
                        transform.translation = players.player_array[player.id as usize].pos;
                        transform.rotation = players.player_array[player.id as usize].rot;
                    }

                    if !boat_found {
                        //getting boat sprite info
                        let boat_sheet_handle = asset_server.load("s_basic_ship.png");
                        let boat_layout =
                            TextureAtlasLayout::from_grid(UVec2::splat(100), 2, 2, None, None);
                        let boat_layout_handle = texture_atlases.add(boat_layout);

                        //spawning boat
                        commands.spawn((
                            SpriteBundle {
                                texture: boat_sheet_handle,
                                transform: Transform {
                                    translation: Vec3::new(0., 0., 900.),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                layout: boat_layout_handle.clone(),
                                index: 0,
                            },
                            Boat {
                                id: p.id,
                                movement_speed: 150.,
                                rotation_speed: f32::to_radians(100.0),
                                acceleration: 0.,
                                aabb: BoundingBox::new(Vec2::splat(0.), Vec2::splat(16.)),
                            },
                        ));
                    }
                }
            } else if env.message == "update_enemies" {
                let packet: Packet<Enemies> = serde_json::from_str(&env.packet).unwrap();
                let enemies = packet.payload;

                for e in enemies.list.iter() {
                    let mut found = false;
                    for (mut transform, enemy, entity) in enemy_query.iter_mut() {
                        if e.id != enemy.id || !enemy.alive {
                            continue;
                        }
                        //transform.translation = e.pos;
                        found = true;
                    }
                }
            } else if env.message == "update_projectiles" {
                let packet: Packet<Projectiles> = serde_json::from_str(&env.packet).unwrap();
                let projectiles = packet.payload;

                for proj in projectiles.list.iter() {
                    for (transform, enemy, entity) in enemy_query.iter() {
                        if proj.owner_id == enemy.id {
                            match enemy.etype {
                                KRAKEN => {
                                    commands.spawn((
                                    SpriteBundle {
                                        texture: asset_server.load("s_kraken_spit_1.png"),
                                        transform: Transform {
                                            translation: proj.translation,
                                            scale: Vec3::splat(2.0),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    KrakenProjectile,
                                    Lifetime(proj.lifetime),
                                    Velocity {
                                        v: proj.velocity.v, /* (direction * speed of projectile) */
                                    },
                                    Hitbox {
                                        size: Vec2::splat(60.),
                                        offset: Vec2::splat(0.),
                                        lifetime: Some(Timer::from_seconds(5., TimerMode::Once)),
                                        entity: KRAKEN,
                                        projectile: true,
                                        enemy: true,
                                    },
                                ));
                                }
                                GHOSTSHIP => {
                                    commands.spawn((
                                    SpriteBundle {
                                        texture: asset_server.load("s_cannonball.png"),
                                        transform: Transform {
                                            translation: proj.translation,
                                            scale: Vec3::splat(2.0),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    GhostShipProjectile,
                                    Lifetime(proj.lifetime),
                                    Velocity {
                                        v: proj.velocity.v, /* (direction * speed of projectile) */
                                    },
                                    Hitbox {
                                        size: Vec2::splat(60.),
                                        offset: Vec2::splat(0.),
                                        lifetime: Some(Timer::from_seconds(5., TimerMode::Once)),
                                        entity: GHOSTSHIP,
                                        projectile: true,
                                        enemy: true,
                                    },));
                                }
                                _ => {
                                    println!("Undefined enemy type");
                                }
                            }
                            break;
                        }
                    }
                }
            } else if env.message == "enemy_dead" {
                let packet: Packet<Enemy> = serde_json::from_str(&env.packet).unwrap();
                let enemy = packet.payload;
                println!("Received [enemy_dead]");

                for (transform, mut e, entity) in enemy_query.iter_mut() {
                    println!("E1: {}, E2: {}", e.id, enemy.id);
                    if e.id != enemy.id {
                        continue;
                    }
                    e.alive = false;
                    commands.entity(entity).despawn();

                    println!("Enemy [{}] dead", e.id);
                    break;
                }
            } else if env.message == "new_enemies" {
                let packet: Packet<Enemies> = serde_json::from_str(&env.packet).unwrap();
                let enemies = packet.payload;

                for e in enemies.list.iter() {
                    match e.etype {
                        KRAKEN => {
                            let transform =
                                Transform::from_translation(e.pos).with_scale(Vec3::splat(2.0));
                            spawn_enemy(
                                &mut commands,
                                EnemyT::Kraken(e.id),
                                transform,
                                &asset_server,
                                &mut texture_atlases,
                            )
                        }
                        GHOSTSHIP => {
                            let transform =
                                Transform::from_translation(e.pos).with_scale(Vec3::splat(2.0));
                            spawn_enemy(
                                &mut commands,
                                EnemyT::GhostShip(e.id),
                                transform,
                                &asset_server,
                                &mut texture_atlases,
                            )
                        }
                        _ => {
                            println!("Undefined enemy in [update_enemies]");
                        }
                    }
                }
            } else {
                println!(
                    "Recieved invalid packet from [{}]: {}",
                    src.ip(),
                    env.message
                );
            }
        }
        Err(e) => {
            //println!("Update: Something happened: {}", e);\
        }
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
    }
}
