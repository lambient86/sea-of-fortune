use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;
use crate::whirlpool::components::Whirlpool;

pub struct WhirlpoolPlugin;

pub fn setup_whirlpool_cooldown(mut commands: Commands) {
    commands.insert_resource(WhirlpoolCooldownTimer {
        timer: Timer::from_seconds(0.032, TimerMode::Once), 
    });
}

fn cleanup_whirlpools(
    mut commands: Commands,
    whirlpool_query: Query<Entity, With<Whirlpool>>,
) {
    for entity in whirlpool_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for WhirlpoolPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WhirlpoolSpawnTimer>()
            .init_resource::<WhirlpoolCooldownTimer>() // Add this line
            .add_systems(OnEnter(GameworldState::Ocean), 
                (
                    setup_whirlpool_timer,
                    setup_whirlpool_cooldown, // Add this line
                )
            )
            .add_systems(
                Update, 
                (
                    spawn_whirlpool,
                    despawn_whirlpool_system,
                    check_whirlpool_collisions,
                )
                    .run_if(in_state(GameworldState::Ocean))
            )
            .add_systems(OnExit(GameworldState::Ocean), cleanup_whirlpools);
    }
}