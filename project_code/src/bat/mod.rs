use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;

pub struct BatPlugin;

impl Plugin for BatPlugin {
    /// Builds the bat plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Dungeon), spawn_bat)
            .add_systems(
                Update,
                (
                    animate_bat,
                    rotate_bat,
                    bat_attack,
                    bat_damaged,
                    move_bat_projectile,
                    bat_proj_lifetime_check,
                    move_bat,
                )
                    .run_if(in_state(GameworldState::Dungeon))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                OnExit(GameworldState::Dungeon),
                (despawn_all_bats, despawn_all_bat_proj),
            );
    }
}
