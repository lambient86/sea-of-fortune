use bevy::prelude::*;
use components::*;
use systems::*;

use crate::components::{Background, GameworldState};
use crate::player::components::Sword;

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
