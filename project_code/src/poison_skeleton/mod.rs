use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;

pub struct PSkeletonPlugin;

impl Plugin for PSkeletonPlugin {
    /// Builds the skeleton plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Dungeon), spawn_pskeleton)
            .add_systems(
                Update,
                (
                    rotate_pskeleton,
                    pskeleton_attack,
                    pskeleton_damaged,
                    pmove_skeleton,
                )
                    .run_if(in_state(GameworldState::Dungeon))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                OnExit(GameworldState::Dungeon),
                (despawn_all_pskeletons, despawn_all_pskeleton_proj),
            );
    }
}
