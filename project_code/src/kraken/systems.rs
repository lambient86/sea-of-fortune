use bevy::math::{vec2, NormedVectorSpace};
use bevy::prelude::*;
use bevy::render::texture;

use crate::boat::components::Boat;
use crate::data::gameworld_data::*;
use crate::hitbox_system::*;
use crate::kraken::components::*;
use crate::player::components::*;
use crate::{enemies::*, HostPlayer};

/*  SPAWN_KRAKEN FUNCTION  */
/// Spawns a kraken entity in the gameworld
pub fn spawn_kraken(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let transform = Transform::from_xyz(0., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.)
        .with_scale(Vec3::splat(2.0));

    spawn_enemy(
        &mut commands,
        EnemyT::Kraken(0),
        transform,
        &asset_server,
        &mut texture_atlases,
    );
}

/*   KRAKEN_DAMAGED FUNCTION   */
/// Current functionality: Detects when a player is within player attack range (this will later be replaced with
// player weapon/attack collision) and then takes 1 damage (dies)
pub fn kraken_damaged(
    mut commands: Commands,
    mut kraken_query: Query<(&mut Kraken, Entity, &mut Hurtbox), With<Kraken>>,
) {
    for (mut kraken, entity, mut hurtbox) in kraken_query.iter_mut() {
        if !hurtbox.colliding {
            continue;
        }

        kraken.current_hp -= 1.;

        if kraken.current_hp <= 0. {
            println!("Kraken was attacked by player, it is dead :(");
            commands.entity(entity).despawn();
        } else {
            println!("Kraken was attacked by player");
        }

        hurtbox.colliding = false;
    }
}

/*   DESPAWN_ALL_KRAKEN FUNCTION   */
/// Despawns a kraken entity
/// DEBUG: Despwans all kraken entities
pub fn despawn_all_krakens(mut commands: Commands, query: Query<Entity, With<Kraken>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/*   KRAKEN_ATTACK FUNCTION   */
/// Detects when player is within kraken attack range and attacks.
/// Things not added:
/// * Attack cooldown timer
/// * Projectile shooting
/// Things currently added:
/// * Distance-to-player checking
/// * Attack cooldown timer
/// * Projectile shooting
pub fn kraken_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut kraken_query: Query<(&Transform, &mut AttackCooldown), With<Kraken>>,
    player_query: Query<(&Transform, &Boat), With<Boat>>,
    asset_server: Res<AssetServer>,
    host: Res<HostPlayer>,
) {
    for (kraken_transform, mut cooldown) in kraken_query.iter_mut() {
        // Attacks only when cooldown is over
        cooldown.remaining.tick(time.delta());
        if !cooldown.remaining.just_finished() {
            continue;
        }

        cooldown.remaining = Timer::from_seconds(2.5, TimerMode::Once);

        //Gets positions (Vec3) of the entities
        let kraken_translation = kraken_transform.translation;

        for (ptransform, boat) in player_query.iter() {
            if boat.id != host.player.id {
                continue;
            }
            let player_translation = ptransform.translation;

            //Gets positions (Vec2) of the entities
            let player_position = player_translation.xy();
            let kraken_position = kraken_translation.xy();

            //Gets distance
            let distance_to_player = kraken_position.distance(player_position);

            if distance_to_player > KRAKEN_ATTACK_DIST {
                continue;
            }

            //Gets direction projectile will be going
            let original_direction = (player_translation - kraken_translation).normalize();
            let angle = original_direction.x.atan2(original_direction.y);
            let angle_direction = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();

            let projectile_start_position = kraken_translation + angle_direction * 10.0;

            //Sets the projectile texture
            let kraken_projectile_handle = asset_server.load("s_kraken_spit_1.png");

            //Creates Projectile
            commands.spawn((
            SpriteBundle {
                texture: kraken_projectile_handle,
                transform: Transform {
                    translation: projectile_start_position,
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            },
            KrakenProjectile,
            Lifetime(KRAKEN_PROJECTILE_LIFETIME),
            Velocity {
                v: angle_direction.truncate() * KRAKEN_PROJECTILE_SPEED, /* (direction * speed of projectile) */
            },
            Hitbox {
                size: Vec2::splat(60.),
                offset: Vec2::splat(0.),
                lifetime: Some(Timer::from_seconds(5., TimerMode::Once)),
                entity: KRAKEN,
                projectile: true,
                enemy: true,
            },
        ));
        }
    }
}

/*   MOVE_KRAKEN_PROJECTILE FUNCTION   */
/// Updates the locations of kraken projectiles
/// Things to add:
/// * Collision handling, dealing damage on collision
pub fn move_kraken_projectile(
    mut proj_query: Query<(&mut Transform, &mut Velocity), With<KrakenProjectile>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in proj_query.iter_mut() {
        // Calculates/moves the projectile

        transform.translation += velocity.to_vec3(0.) * time.delta_seconds();
    }
}

/*   KRAKEN_PROJ_LIFETIME_CHECK FUNCTION   */
/// Checks the lifetime left on a kraken's projectile, and despawns
/// after the lifetime expires
pub fn kraken_proj_lifetime_check(
    time: Res<Time>,
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        lifetime.0 -= time.delta_seconds();
        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn();

            /* Debug */
            //println!("Projectile despawned");
        }
    }
}

/*   MOVE_KRAKEN FUNCTION   */
/// Moves the kraken as long as a player is within agro range
pub fn move_kraken(
    time: Res<Time>,
    mut kraken_query: Query<&mut Transform, With<Kraken>>,
    player_query: Query<(&Transform, &Boat), (With<Boat>, Without<Kraken>)>,
    host: Res<HostPlayer>,
) {
    for mut transform in kraken_query.iter_mut() {
        //Gets positions (Vec3) of the entities
        let kraken_translation = transform.translation;

        for (ptransform, boat) in player_query.iter() {
            if boat.id != host.player.id {
                continue;
            }
            let player_translation = ptransform.translation;

            //Gets positions (Vec2) of the entities
            let player_position = player_translation.xy();
            let kraken_position = kraken_translation.xy();

            //Gets distance
            let distance_to_player = kraken_position.distance(player_position);

            //Check
            if distance_to_player > KRAKEN_AGRO_RANGE || distance_to_player <= KRAKEN_AGRO_STOP {
                continue;
            }

            //Gets direction projectile will be going
            let direction = (player_translation - kraken_translation).normalize();
            let velocity = direction * KRAKEN_MOVEMENT_SPEED;

            //Moves kraken
            transform.translation += velocity * time.delta_seconds();
        }
    }
}

/*   DESPAWN_ALL_KRAKEN_PROJ   */
/// Despawns all the kraken's projectiles
pub fn despawn_all_kraken_proj(
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
