pub(crate) mod components;
pub mod systems;

use crate::GameState;
use crate::GameworldState;
use bevy::app::Plugin;
use bevy::prelude::*;
use crate::boss::systems::*;
use crate::boss::components::*;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    /// Builds the boss plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Dungeon), spawn_boss)
            .add_systems(
                Update,
                (boss_damaged, move_boss)
                    .run_if(in_state(GameworldState::Dungeon))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(GameworldState::Dungeon), despawn_all_bosses);
    }
}