use bevy::prelude::*;
use systems::*;
use components::*;

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
                (setup_level, despawn_with::<Background>, despawn_with::<Sword>)
            )
            .add_systems(
                OnExit(GameworldState::Ocean),
                despawn_with::<OceanTile>,
            )
            .add_systems(
                OnEnter(GameworldState::Island), 
                (setup_level, despawn_with::<Background>,)
            )
            .add_systems(
                OnExit(GameworldState::Island),
                despawn_with::<SandTile>,
            );
    }
}