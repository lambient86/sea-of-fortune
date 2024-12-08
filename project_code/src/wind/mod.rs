use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::components::GameState;
use crate::GameworldState;
use systems::*;

pub struct WindPlugin;

impl Plugin for WindPlugin {
    /// Builds the boat plugin
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_wind).add_systems(
            Update,
            (change_wind_dir)
                .run_if(in_state(GameworldState::Ocean))
                .run_if(in_state(GameState::Running)),
        );
    }
}
