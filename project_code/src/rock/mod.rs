pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use bevy::app::Plugin;
use bevy::prelude::*;
use systems::*;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    /// Builds the rock plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Dungeon), spawn_rock)
            .add_systems(
                Update,
                (rock_damaged, move_rock)
                    .run_if(in_state(GameworldState::Dungeon))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(GameworldState::Dungeon), (despawn_all_rocks));
    }
}
