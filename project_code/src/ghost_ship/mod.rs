use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;

pub struct GhostShipPlugin;

impl Plugin for GhostShipPlugin {
    /// Builds the kraken plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Ocean), spawn_ghostship)
            .add_systems(
                Update,
                (
                    rotate_ghostship,
                    ghostship_attack,
                    ghostship_damaged,
                    move_ghostship_projectile,
                    ghostship_proj_lifetime_check,
                    move_ghostship,
                )
                    .run_if(in_state(GameworldState::Ocean))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                OnExit(GameworldState::Ocean),
                (despawn_all_ghostships, despawn_all_ghostship_proj),
            );
    }
}
