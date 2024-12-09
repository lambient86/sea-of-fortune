use crate::boat::components::*;
use crate::data::gameworld_data::*;
use crate::enemies::*;
use crate::hitbox_system::Hurtbox;
use crate::whirlpool::components::Lifetime;
use crate::whirlpool::components::*;
use bevy::math::{vec2, NormedVectorSpace};
use bevy::prelude::*;
use bevy::render::texture;
use bevy::time::Time;
use rand::random;
use rand::Rng;

#[derive(Resource, Default)]
pub struct WhirlpoolSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct WhirlpoolCooldownTimer {
    pub timer: Timer,
}

pub fn setup_whirlpool_timer(mut commands: Commands) {
    let initial_duration = rand::thread_rng().gen_range(25.0..35.0);
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
            let mut rng = rand::thread_rng();

            // Generate random coordinates within the ocean bounds
            let spawn_x = rng.gen_range(-(OCEAN_LEVEL_W / 2.0)..(OCEAN_LEVEL_W / 2.0));
            let spawn_y = rng.gen_range(-(OCEAN_LEVEL_H / 2.0)..(OCEAN_LEVEL_H / 2.0));

            // Ensure minimum distance from boat
            let min_distance = 300.0; // Minimum distance from boat
            let boat_pos = Vec2::new(boat_transform.translation.x, boat_transform.translation.y);
            let spawn_pos = Vec2::new(spawn_x, spawn_y);

            // If too close to boat, adjust the position
            let spawn_pos = if (spawn_pos - boat_pos).length() < min_distance {
                let direction = (spawn_pos - boat_pos).normalize();
                let adjusted_pos = boat_pos + direction * min_distance;
                Vec3::new(adjusted_pos.x, adjusted_pos.y, 0.0)
            } else {
                Vec3::new(spawn_x, spawn_y, 0.0)
            };

            spawn_enemy(
                &mut commands,
                EnemyT::Whirlpool(0),
                Transform::from_translation(spawn_pos),
                &asset_server,
                &mut texture_atlases,
            );

            let new_duration = rng.gen_range(25.0..35.0);
            spawn_timer
                .timer
                .set_duration(std::time::Duration::from_secs_f32(new_duration));
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
    mut query_set: ParamSet<(
        Query<(&mut Transform, &Hurtbox), With<Boat>>,
        Query<(&Transform, &Hurtbox), With<Whirlpool>>,
    )>,
    time: Res<Time>,
    mut cooldown_timer: ResMut<WhirlpoolCooldownTimer>,
) {
    // Tick the cooldown timer
    cooldown_timer.timer.tick(time.delta());

    // Only check collisions if we're not in cooldown
    if !cooldown_timer.timer.finished() {
        return;
    }

    let whirlpool_positions: Vec<(Vec3, f32)> = query_set
        .p1()
        .iter()
        .map(|(transform, hurtbox)| (transform.translation, hurtbox.size.x))
        .collect();

    if let Ok((mut boat_transform, boat_hurtbox)) = query_set.p0().get_single_mut() {
        for (whirlpool_pos, whirlpool_size) in whirlpool_positions {
            let distance = boat_transform.translation.distance(whirlpool_pos);
            let collision_distance = (boat_hurtbox.size.x + whirlpool_size) / 2.0;

            // Check if boat is within whirlpool's influence
            if distance < collision_distance {
                println!("Boat caught in whirlpool!");

                // Calculate direction from boat to whirlpool center
                let direction = (whirlpool_pos - boat_transform.translation).normalize();

                // Pull strength increases as boat gets closer to center
                let pull_strength = 5.0 * (1.0 - distance / collision_distance);

                // Apply the pull force
                boat_transform.translation += direction * pull_strength;

                // Reset the cooldown timer when a collision occurs
                cooldown_timer.timer.reset();
            }
        }
    }
}
