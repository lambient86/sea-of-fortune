use bevy::prelude::*;

use crate::boat::components::Boat;
use crate::data::gameworld_data::*;
use crate::ghost_ship::components::*;
use crate::hitbox_system::*;
use crate::player::components::*;
use crate::{enemies::*, HostPlayer};

/*   ROTATE_KRAKEN FUNCTION   */
/// This should be changed to a function called "track_player", which will
/// be how the kraken knows where to check where the player is for shooting projectiles
///
/// WE DON'T NEED TO ROTATE THE KRAKEN! I WILL MAKE A BACK FACING SPRITE IF NEEDED
pub fn rotate_ghostship(
    time: Res<Time>,
    mut query: Query<(&GhostShip, &mut Transform), Without<Boat>>,
    player_query: Query<(&Transform, &Boat), With<Boat>>,
    host: Res<HostPlayer>,
) {
    // getting player position

    for (ghostship, mut enemy_transform) in &mut query {
        for (transform, boat) in player_query.iter() {
            if boat.id != host.player.id {
                continue;
            }
            let player_translation = transform.translation.xy();
            //getting kraken's position relative to player position
            let ghostship_position = enemy_transform.translation.xy();
            let distance_to_player = ghostship_position.distance(player_translation);

            //ensuring kraken is close enough to player to attack
            if distance_to_player > GHOSTSHIP_ATTACK_DIST {
                break;
            }

            //getting enemy forward
            let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
            let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
            let forward_dot_player = enemy_forward.dot(to_player);

            if (forward_dot_player - 1.0).abs() < f32::EPSILON {
                continue;
            }

            let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

            let right_dot_player = enemy_right.dot(to_player);

            let rotation_sign = -f32::copysign(1.0, right_dot_player);
            let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();

            let rotation_angle =
                rotation_sign * (ghostship.rotation_speed * time.delta_seconds()).min(max_angle);

            enemy_transform.rotate_z(rotation_angle);
        }
    }
}

/*  SPAWN_KRAKEN FUNCTION  */
/// Spawns a kraken entity in the gameworld
pub fn spawn_ghostship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let transform = Transform::from_xyz(200., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.)
        .with_scale(Vec3::splat(2.0));

    spawn_enemy(
        &mut commands,
        EnemyT::GhostShip(0),
        transform,
        &asset_server,
        &mut texture_atlases,
    );
}

/*   KRAKEN_DAMAGED FUNCTION   */
/// Current functionality: Detects when a player is within player attack range (this will later be replaced with
// player weapon/attack collision) and then takes 1 damage (dies)
pub fn ghostship_damaged(
    mut commands: Commands,
    mut ghostship_query: Query<(&mut GhostShip, Entity, &mut Hurtbox)>,
) {
    for (mut ghostship, entity, mut hurtbox) in ghostship_query.iter_mut() {
        if !hurtbox.colliding {
            continue;
        }

        ghostship.current_hp -= 1.;

        if ghostship.current_hp <= 0. {
            println!("Ghostship was attacked by player, it is dead :(");
            commands.entity(entity).despawn();
        } else {
            println!("Ghostship was attacked by player");
        }

        hurtbox.colliding = false;
    }
}

/*   DESPAWN_ALL_KRAKEN FUNCTION   */
/// Despawns a kraken entity
/// DEBUG: Despwans all kraken entities
pub fn despawn_all_ghostships(mut commands: Commands, query: Query<Entity, With<GhostShip>>) {
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
pub fn ghostship_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut ghostship_query: Query<(&Transform, &mut AttackCooldown), With<GhostShip>>,
    player_query: Query<(&Transform, &Boat), With<Boat>>,
    asset_server: Res<AssetServer>,
    host: Res<HostPlayer>,
) {
    for (ghostship_transform, mut cooldown) in ghostship_query.iter_mut() {
        // Attacks only when cooldown is over
        cooldown.remaining.tick(time.delta());
        if !cooldown.remaining.just_finished() {
            continue;
        }

        cooldown.remaining = Timer::from_seconds(2.5, TimerMode::Once);

        //Gets positions (Vec3) of the entities
        let ghostship_translation = ghostship_transform.translation;

        for (ptransform, boat) in player_query.iter() {
            if boat.id != host.player.id {
                continue;
            }
            let player_translation = ptransform.translation;

            //Gets positions (Vec2) of the entities
            let player_position = player_translation.xy();
            let ghostship_position = ghostship_translation.xy();

            //Gets distance
            let distance_to_player = ghostship_position.distance(player_position);

            if distance_to_player > GHOSTSHIP_ATTACK_DIST {
                continue;
            }

            //Gets direction projectile will be going
            let original_direction = (player_translation - ghostship_translation).normalize();
            let angle = original_direction.x.atan2(original_direction.y);
            let angle_direction = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();

            let projectile_start_position = ghostship_translation + angle_direction * 10.0;

            //Sets the projectile texture
            let ghostship_projectile_handle = asset_server.load("s_cannonball.png");

            //Creates Projectile
            commands.spawn((
            SpriteBundle {
                texture: ghostship_projectile_handle,
                transform: Transform {
                    translation: projectile_start_position,
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            },
            GhostShipProjectile,
            Lifetime(GHOSTSHIP_PROJECTILE_LIFETIME),
            Velocity {
                v: angle_direction.truncate() * GHOSTSHIP_PROJECTILE_SPEED, /* (direction * speed of projectile) */
            },
            Hitbox {
                size: Vec2::splat(60.),
                offset: Vec2::splat(0.),
                lifetime: Some(Timer::from_seconds(5., TimerMode::Once)),
                entity: GHOSTSHIP,
                projectile: true,
                enemy: true,
            },));
        }
    }
}

/*   MOVE_KRAKEN_PROJECTILE FUNCTION   */
/// Updates the locations of kraken projectiles
/// Things to add:
/// * Collision handling, dealing damage on collision
pub fn move_ghostship_projectile(
    mut proj_query: Query<(&mut Transform, &mut Velocity), With<GhostShipProjectile>>,
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
pub fn ghostship_proj_lifetime_check(
    time: Res<Time>,
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Lifetime), With<GhostShipProjectile>>,
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
pub fn move_ghostship(
    time: Res<Time>,
    mut ghostship_query: Query<&mut Transform, With<GhostShip>>,
    player_query: Query<(&Transform, &Boat), (With<Boat>, Without<GhostShip>)>,
    host: Res<HostPlayer>,
) {
    for mut transform in ghostship_query.iter_mut() {
        //Gets positions (Vec3) of the entities
        let ghostship_translation = transform.translation;

        for (ptransform, boat) in player_query.iter() {
            if boat.id != host.player.id {
                continue;
            }
            let player_translation = ptransform.translation;

            //Gets positions (Vec2) of the entities
            let player_position = player_translation.xy();
            let ghostship_position = ghostship_translation.xy();

            //Gets distance
            let distance_to_player = ghostship_position.distance(player_position);

            //Check
            if distance_to_player > GHOSTSHIP_AGRO_RANGE
                || distance_to_player <= GHOSTSHIP_AGRO_STOP
            {
                continue;
            }

            //Gets direction projectile will be going
            let direction = (player_translation - ghostship_translation).normalize();
            let velocity = direction * GHOSTSHIP_MOVEMENT_SPEED;

            //Moves kraken
            transform.translation += velocity * time.delta_seconds();
        }
    }
}

/*   DESPAWN_ALL_KRAKEN_PROJ   */
/// Despawns all the kraken's projectiles
pub fn despawn_all_ghostship_proj(
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Lifetime), With<GhostShipProjectile>>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
