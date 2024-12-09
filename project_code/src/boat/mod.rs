use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::components::GameState;
use crate::GameworldState;
use systems::*;
use crate::player::systems::*;

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    /// Builds the boat plugin
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameworldState::Ocean),
            spawn_boat.after(despawn_player),
        )
        .add_systems(
            Update,
            (
                move_boat,
                boat_attack.after(move_boat),
                move_cannonball,
                cannonball_lifetime_check,
            )
                .run_if(in_state(GameworldState::Ocean))
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            OnExit(GameworldState::Ocean),
            (despawn_boat, despawn_cannonballs),
        );
    }
}
