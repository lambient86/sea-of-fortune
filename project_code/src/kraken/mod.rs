use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;

pub struct KrakenPlugin;

impl Plugin for KrakenPlugin {
    /// Builds the kraken plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Ocean), spawn_kraken)
            .add_systems(
                Update,
                (
                    kraken_attack,
                    kraken_damaged,
                    move_kraken_projectile,
                    kraken_proj_lifetime_check,
                    move_kraken,
                )
                    .run_if(in_state(GameworldState::Ocean))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                OnExit(GameworldState::Ocean),
                (despawn_all_krakens, despawn_all_kraken_proj),
            );
    }
}
