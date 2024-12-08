use bevy::prelude::*;

pub(crate) mod components;
mod systems;

use crate::GameState;
use crate::GameworldState;
use systems::*;

pub struct WhirlpoolPlugin;

pub fn setup_whirlpool_cooldown(mut commands: Commands) {
    commands.insert_resource(WhirlpoolCooldownTimer {
        timer: Timer::from_seconds(2.0, TimerMode::Once), // 2 second cooldown
    });
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
            );
    }
}