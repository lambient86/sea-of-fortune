use bevy::prelude::*;

mod components;
mod systems;

use systems::*;
use crate::GameworldState;

pub struct BatPlugin;

impl Plugin for BatPlugin {
    /// Builds the bat plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Island), spawn_bat)
            .add_systems(Update, (
                animate_bat,
                rotate_bat,
                bat_attack,
                bat_damaged,
            )
            .run_if(in_state(GameworldState::Island)))
            .add_systems(OnExit(GameworldState::Island), despawn_all_bats);
    }
}
