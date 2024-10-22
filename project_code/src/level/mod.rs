use bevy::prelude::*;
use systems::*;
use components::*;

use crate::components::GameworldState;

pub mod components;
pub mod systems;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_level)
            .add_systems(OnEnter(GameworldState::Ocean), setup_level)
            .add_systems(
                OnExit(GameworldState::Ocean),
                despawn_with::<OceanTile>,
            )
            .add_systems(OnEnter(GameworldState::Island), setup_level)
            .add_systems(
                OnExit(GameworldState::Island),
                despawn_with::<SandTile>,
            )
            .add_systems(OnEnter(GameworldState::Dungeon), setup_level)
            .add_systems(
                OnExit(GameworldState::Dungeon),
                despawn_with::<DungeonTile>,
            );
    }
}