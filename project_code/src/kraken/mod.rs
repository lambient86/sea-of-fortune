use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;

pub struct KrakenPlugin;

impl Plugin for KrakenPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize the spawn timer resource
            .init_resource::<KrakenSpawnTimer>()
            
            // Setup systems
            .add_systems(OnEnter(GameworldState::Ocean), setup_kraken_timer)
            
            // Main game systems
            .add_systems(
                Update,
                (
                    spawn_kraken,
                    kraken_attack,
                    kraken_damaged,
                    move_kraken_projectile,
                    kraken_proj_lifetime_check,
                    move_kraken,
                )
                    .run_if(in_state(GameworldState::Ocean))
                    .run_if(in_state(GameState::Running)),
            )
            
            // Cleanup systems
            .add_systems(
                OnExit(GameworldState::Ocean),
                (despawn_all_krakens, despawn_all_kraken_proj),
            );
    }
}
