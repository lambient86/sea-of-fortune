use bevy::prelude::*;
pub mod components;
pub mod systems;
use crate::storm::components::Storm;
use crate::GameworldState;

use systems::*;

pub struct StormPlugin;

fn cleanup_storms(mut commands: Commands, storm_query: Query<Entity, With<Storm>>) {
    for entity in storm_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for StormPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<StormSpawnTimer>()
            .init_resource::<StormDamageCooldownTimer>()
            // Setup systems on enter
            .add_systems(
                OnEnter(GameworldState::Ocean),
                (setup_storm_timer, setup_storm_damage_cooldown),
            )
            // Update systems
            .add_systems(
                Update,
                (spawn_storm, storm_damage_system).run_if(in_state(GameworldState::Ocean)),
            )
            // Cleanup on exit
            .add_systems(OnExit(GameworldState::Ocean), cleanup_storms);
    }
}
