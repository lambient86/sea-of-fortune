use bevy::math::{vec2, NormedVectorSpace};
use bevy::prelude::*;
use bevy::render::texture;
use bevy::time::Time;
use rand::random;
use crate::enemies::*;
use crate::data::gameworld_data::*;
use crate::boat::components::*;
use crate::whirlpool::components::*;
use crate::whirlpool::components::Lifetime;
use rand::Rng;
use crate::hitbox_system::Hurtbox;

#[derive(Resource, Default)]
pub struct WhirlpoolSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct WhirlpoolCooldownTimer {
    pub timer: Timer,
}

pub fn setup_whirlpool_timer(mut commands: Commands) {
    let initial_duration = rand::thread_rng().gen_range(25.0..45.0);
    commands.insert_resource(WhirlpoolSpawnTimer {
        timer: Timer::from_seconds(initial_duration, TimerMode::Once), 
    });
}

pub fn spawn_whirlpool(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<WhirlpoolSpawnTimer>,
    boat_query: Query<&Transform, With<Boat>>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        if let Ok(boat_transform) = boat_query.get_single() {
            // Screen dimensions 
            let screen_width = WIN_W; 
            let screen_height = WIN_H; 
            
            // Calculate minimum spawn distance to be just outside screen
            let min_spawn_distance = (screen_width.max(screen_height) / 2.0) + 25.0; 
            let max_spawn_distance = min_spawn_distance + 50.0; 
            
            // Generate random angle
            let angle = random::<f32>() * std::f32::consts::TAU;
            
            // Generate random distance between min and max
            let distance = rand::thread_rng().gen_range(min_spawn_distance..max_spawn_distance);
            
            let offset_x = angle.cos() * distance;
            let offset_y = angle.sin() * distance;
            
            let spawn_pos = Vec3::new(
                boat_transform.translation.x + offset_x,
                boat_transform.translation.y + offset_y,
                0.0
            );

            spawn_enemy(
                &mut commands,
                EnemyT::Whirlpool(0),
                Transform::from_translation(spawn_pos),
                &asset_server,
                &mut texture_atlases,
            );

            let new_duration = rand::thread_rng().gen_range(25.0..45.0);
            spawn_timer.timer.set_duration(std::time::Duration::from_secs_f32(new_duration));
            spawn_timer.timer.reset();
        }
    }
}

pub fn despawn_whirlpool_system(
    mut commands: Commands,
    time: Res<Time>,
    mut lifetime_query: Query<(Entity, &mut Lifetime), With<Whirlpool>>,
) {
    // Update lifetimes
    for (entity, mut lifetime) in lifetime_query.iter_mut() {
        lifetime.0 -= time.delta_seconds();
        // If lifetime expired, despawn individually
        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn check_whirlpool_collisions(
    boat_query: Query<(&Transform, &Hurtbox), With<Boat>>,
    whirlpool_query: Query<(&Transform, &Hurtbox), With<Whirlpool>>,
    time: Res<Time>,
    mut cooldown_timer: ResMut<WhirlpoolCooldownTimer>,
) {
    // Tick the cooldown timer
    cooldown_timer.timer.tick(time.delta());

    // Only check collisions if we're not in cooldown
    if !cooldown_timer.timer.finished() {
        return;
    }

    if let Ok((boat_transform, boat_hurtbox)) = boat_query.get_single() {
        for (whirlpool_transform, whirlpool_hurtbox) in whirlpool_query.iter() {
            let distance = boat_transform.translation.distance(whirlpool_transform.translation);
            let collision_distance: f32 = (boat_hurtbox.size.x + whirlpool_hurtbox.size.x) / 2.0;

            if distance < collision_distance {
                println!("Boat hit whirlpool!");
                // Reset the cooldown timer when a collision occurs
                cooldown_timer.timer.reset();
            }
        }
    }
}

