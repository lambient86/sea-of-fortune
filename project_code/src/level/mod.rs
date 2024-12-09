use bevy::prelude::*;
use components::*;
use systems::*;

use crate::components::{Background, GameworldState};
use crate::player::components::Sword;
use crate::{create_env, HostPlayer, Server, UDP};

pub mod components;
pub mod systems;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_level)
            .add_systems(
                OnEnter(GameworldState::Ocean),
                (
                    setup_ocean,
                    got_here_late_packet.after(setup_ocean),
                    despawn_with::<Background>,
                    despawn_with::<Sword>,
                    despawn_with::<Dungeon>,
                    despawn_with::<OceanDoor>,
                ),
            )
            .add_systems(
                OnExit(GameworldState::Ocean),
                (despawn_with::<OceanTile>, despawn_with::<Island>),
            )
            .add_systems(
                OnEnter(GameworldState::Island),
                (setup_island, despawn_with::<Background>),
            )
            .add_systems(OnEnter(GameworldState::Dungeon),
                        (setup_dungeon, despawn_with::<Island>),
            )
            .add_systems(
                OnExit(GameworldState::Island),
                (despawn_with::<SandTile>, despawn_with::<OceanDoor>),
            );
    }
}

pub fn got_here_late_packet(udp: Res<UDP>, host: Res<HostPlayer>, server: Res<Server>) {
    udp.socket
        .send_to(
            create_env("got_here_late".to_string(), host.player.clone()).as_bytes(),
            server.addr.clone(),
        )
        .expect("Failed to send [got_here_late] packet");
}
