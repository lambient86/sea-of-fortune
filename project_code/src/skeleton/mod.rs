use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;

pub struct SkeletonPlugin;

impl Plugin for SkeletonPlugin {
    /// Builds the skeleton plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Dungeon), spawn_skeleton)
            .add_systems(
                Update,
                (
                    rotate_skeleton,
                    skeleton_attack,
                    skeleton_damaged,
                    move_skeleton_projectile,
                    skeleton_proj_lifetime_check,
                    move_skeleton,
                )
                    .run_if(in_state(GameworldState::Dungeon))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                OnExit(GameworldState::Dungeon),
                (despawn_all_skeletons, despawn_all_skeleton_proj),
            );
    }
}

/*
app.add_systems(OnEnter(GameworldState::Dungeon), spawn_skeleton)
            .add_systems(
                Update,
                (
                    animate_skeleton,
                    rotate_skeleton,
                    skeleton_attack,
                    skeleton_damaged,
                    move_skeleton_projectile,
                    skeleton_proj_lifetime_check,
                    move_skeleton,
                )
                    .run_if(in_state(GameworldState::Dungeon))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                OnExit(GameworldState::Dungeon),
                (despawn_all_skeletons, despawn_all_skeleton_proj),
            );
*/
