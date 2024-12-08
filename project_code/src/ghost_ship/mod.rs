use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;

pub struct GhostShipPlugin;

impl Plugin for GhostShipPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize the spawn timer resource
            .init_resource::<GhostSpawnTimer>()
            
            // Setup systems
            .add_systems(OnEnter(GameworldState::Ocean), setup_ghost_timer)
            
            // Main game systems
            .add_systems(
                Update,
                (
                    spawn_ghostship,
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
            
            // Cleanup systems
            .add_systems(
                OnExit(GameworldState::Ocean),
                (despawn_all_ghostships, despawn_all_ghostship_proj),
            );
    }
}
