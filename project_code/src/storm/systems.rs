use crate::boat::components::*;
use crate::data::gameworld_data::*;
use crate::enemies::*;
use bevy::prelude::*;
use rand::Rng;
use crate::storm::components::Storm;
use crate::Hurtbox;
use crate::player::components::Player;

#[derive(Resource, Default)]
pub struct StormSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct StormDamageCooldownTimer {
    pub timer: Timer,
}

pub fn setup_storm_timer(mut commands: Commands) {
    let initial_duration = rand::thread_rng().gen_range(30.0..35.0);
    commands.insert_resource(StormSpawnTimer {
        timer: Timer::from_seconds(initial_duration, TimerMode::Once),
    });
}

pub fn setup_storm_damage_cooldown(mut commands: Commands) {
    commands.insert_resource(StormDamageCooldownTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
}

pub fn spawn_storm(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<StormSpawnTimer>,
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
            let min_distance = 500.0; // Larger minimum distance since storms are bigger
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
                EnemyT::Storm(0),
                Transform::from_translation(spawn_pos),
                &asset_server,
                &mut texture_atlases,
            );

            // Set next spawn timer
            let new_duration = rng.gen_range(30.0..35.0);
            spawn_timer
                .timer
                .set_duration(std::time::Duration::from_secs_f32(new_duration));
            spawn_timer.timer.reset();
        }
    }
}

pub fn storm_damage_system(
    mut query_set: ParamSet<(
        Query<(&mut Transform, &Hurtbox, &mut Boat), With<Boat>>,
        Query<(&Transform, &Hurtbox), With<Storm>>,
    )>,
    time: Res<Time>,
    mut cooldown_timer: ResMut<StormDamageCooldownTimer>,
) {
    // Tick the cooldown timer
    cooldown_timer.timer.tick(time.delta());

    // Only check collisions if we're not in cooldown
    if !cooldown_timer.timer.finished() {
        return;
    }

    let storm_positions: Vec<(Vec3, f32)> = query_set
        .p1()
        .iter()
        .map(|(transform, hurtbox)| (transform.translation, hurtbox.size.x))
        .collect();

    if let Ok((mut boat_transform, boat_hurtbox, mut boat)) = query_set.p0().get_single_mut() {
        for (storm_pos, storm_size) in storm_positions {
            let distance = boat_transform.translation.distance(storm_pos);
            let collision_distance = (boat_hurtbox.size.x + storm_size) / 2.0;

            // Check if boat is within storm's influence
            if distance < collision_distance {
                println!("Boat caught in storm!");

                // Calculate direction from boat to storm center
                let direction = (storm_pos - boat_transform.translation).normalize();

                // Apply damage to the boat
                boat.health -= 5.0;
                println!("Storm damaged boat! Current health: {}", boat.health);

                // Check if boat is destroyed
                if boat.health <= 0.0 {
                    println!("Boat destroyed by storm!");
                }

                // Reset the cooldown timer when a collision occurs
                cooldown_timer.timer.reset();
            }
        }
    }
}