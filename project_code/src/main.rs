mod bat;
mod boat;
mod boss;
mod components;
mod controls;
mod data;
mod enemies;
mod ghost_ship;
mod hitbox_system;
mod hud;
mod kraken;
mod level;
mod network;
mod player;
mod poison_skeleton;
mod rock;
mod shop;
mod skeleton;
mod storm;
mod systems;
mod transition_box;
mod wfc;
mod whirlpool;
mod wind;

use bat::BatPlugin;
use bevy::asset;
use bevy::{prelude::*, window::PresentMode};
use boat::components::Boat;
use boat::systems::*;
use boat::BoatPlugin;
use boss::BossPlugin;
use components::*;
use controls::*;
use data::gameworld_data::*;
use enemies::*;
use enemies::*;
use ghost_ship::components::*;
use ghost_ship::GhostShipPlugin;
use hitbox_system::*;
use hud::HUDPlugin;
use kraken::components::*;
use kraken::KrakenPlugin;
use level::components::*;
use level::LevelPlugin;
use player::components::AttackCooldown;
use player::systems::*;
use player::PlayerPlugin;
use poison_skeleton::PSkeletonPlugin;
use rock::RockPlugin;
use shop::ShopPlugin;
use skeleton::SkeletonPlugin;
use storm::StormPlugin;
use systems::*;
use wfc::WFCPlugin;
use whirlpool::WhirlpoolPlugin;
use wind::WindPlugin;

use std::io::ErrorKind;
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

    let server = Server {
        addr: "127.0.0.1:5000".to_string(),
    };

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
                    server.addr.clone(),
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
            std::thread::sleep(Duration::from_secs(2));
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
        .insert_resource(server)
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
        .add_plugins(RockPlugin)
        .add_plugins(WindPlugin)
        .add_plugins(WhirlpoolPlugin)
        .add_plugins(BossPlugin)
        .add_plugins(HUDPlugin)
        .add_plugins(PSkeletonPlugin)
        .add_plugins(StormPlugin)
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
        .add_systems(Update, check_wall_collisions.after(move_player))
        .add_systems(Update, handle_transition_immunity)
        .add_systems(
            OnEnter(GameworldState::Dungeon),
            handle_dungeon_entry.after(initial_spawn_player),
        )
        .add_systems(OnEnter(GameworldState::Dungeon), handle_door_translation)
        .add_systems(OnEnter(GameworldState::Island), handle_door_translation)
        .add_systems(Update, update_dungeon_collision)
        .insert_state(GameworldState::MainMenu)
        .insert_state(GameState::Running)
        .insert_resource(SpawnLocations::default())
        .insert_resource(PlayerEntities::default())
        .insert_resource(CurrentIslandType::default())
        .insert_resource(StateTransitionCooldown::default())
        .add_systems(Last, leave)
        .run();
}

fn leave(
    exit_events: EventReader<AppExit>,
    mut exit_triggered: Local<bool>,
    udp: Res<UDP>,
    player: Res<HostPlayer>,
    server: Res<Server>,
) {
    if !*exit_triggered && exit_events.len() > 0 {
        *exit_triggered = true;

        udp.socket
            .send_to(
                create_env("player_leave".to_string(), player.player.clone()).as_bytes(),
                server.addr.clone(),
            )
            .expect("Failed to send [player_leave]] packet");
    }
}
