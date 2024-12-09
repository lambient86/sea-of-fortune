use bevy::prelude::*;
use bevy::render::texture;
use bevy::sprite::TextureAtlas;

use crate::data::gameworld_data::*;
use crate::enemies::*;
use crate::hitbox_system::*;
use crate::player::components::*;
use crate::shop::systems::*;
use crate::skeleton::components::*;

/*   ROTATE_skeleton FUNCTION   */
/// This should be changed to a function called "track_player", which will
/// be how the skeleton knows where to check where the player is for shooting projectiles
///
/// WE DON'T NEED TO ROTATE THE skeleton! I WILL MAKE A BACK FACING SPRITE IF NEEDED
pub fn rotate_skeleton(
    time: Res<Time>,
    mut query: Query<(&Skeleton, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // getting player position
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for (skeleton, mut enemy_transform) in &mut query {
        //getting skeleton's position relative to player position
        let skeleton_position = enemy_transform.translation.xy();
        let distance_to_player = skeleton_position.distance(player_translation);

        //ensuring skeleton is close enough to player to attack
        if distance_to_player > SKELETON_ATTACK_DIST {
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
            rotation_sign * (skeleton.rotation_speed * time.delta_seconds()).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);
    }
}

/*   ANIMATE_SKELETON FUNCTION   */
/// Animates a skeleton entity
pub fn animate_skeleton(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &AnimationFrameCount)>,
) {
    for (mut timer, mut texture_atlas, frame_count) in &mut query {
        timer.tick(time.delta());

        if timer.finished() {
            texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
        }
    }
}

/*  SPAWN_SKELETON FUNCTION  */
/// Spawns a skeleton entity in the gameworld

pub fn spawn_skeleton(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let transform = Transform::from_xyz(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.)
        .with_scale(Vec3::splat(2.0));

    // Spawning skeleton 1
    spawn_enemy(
        &mut commands,
        EnemyT::RSkeleton,
        transform,
        &asset_server,
        &mut texture_atlases,
    );
}

/*   Skeleton_DAMAGED FUNCTION   */
/// Current functionality: Detects when a player is within player attack range (this will later be replaced with
// player weapon/attack collision) and then takes 1 damage (dies)
pub fn skeleton_damaged(
    mut commands: Commands,
    mut skeleton_query: Query<(&mut Skeleton, Entity, &mut Hurtbox, &Transform), With<Skeleton>>,
    mut player_query: Query<&mut Player>,
) {
    for (mut skeleton, entity, mut hurtbox, transform) in skeleton_query.iter_mut() {
        if !hurtbox.colliding {
            continue;
        }

        skeleton.current_hp -= 1.;

        if skeleton.current_hp <= 0. {
            println!("Skeleton was attacked by player, it is dead :(");
            let loot = generate_loot_item(EnemyT::RSkeleton);
            if loot.price > 0 {
                println!("Skeleton dropped: {}", loot.name);
                if let Ok(mut player) = player_query.get_single_mut() {
                    player.inventory.add_item(loot);
                }
            }
            commands.entity(entity).despawn();
        } else {
            println!("Skeleton was attacked by player");
        }

        hurtbox.colliding = false;
    }
}

/*   DESPAWN_ALL_SKELETON FUNCTION   */
/// Despawns a skeleton entity
/// DEBUG: Despwans all skeleton entities
pub fn despawn_all_skeletons(mut commands: Commands, query: Query<Entity, With<Skeleton>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/*   skeleton_ATTACK FUNCTION   */
/// Detects when player is within skeleton attack range and attacks.
/// Things not added:
/// * Attack cooldown timer
/// * Projectile shooting
/// Things currently added:
/// * Distance-to-player checking
/// * Attack cooldown timer
/// * Projectile shooting
pub fn skeleton_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut skeleton_query: Query<(&Transform, &mut AttackCooldown), With<Skeleton>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (skeleton_transform, mut cooldown) in skeleton_query.iter_mut() {
        // Attacks only when cooldown is over
        cooldown.remaining.tick(time.delta());
        if !cooldown.remaining.just_finished() {
            continue;
        }

        cooldown.remaining = Timer::from_seconds(1.5, TimerMode::Once);

        // Gets positions (Vec3) of the entities
        let skeleton_translation = skeleton_transform.translation;
        let player_translation = player_query.single().translation;

        // Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let skeleton_position = skeleton_translation.xy();

        // Gets distance
        let distance_to_player = skeleton_position.distance(player_position);

        if distance_to_player > SKELETON_ATTACK_DIST {
            continue;
        }

        // Gets direction projectile will be going
        let original_direction = (player_translation - skeleton_translation).normalize();
        let angle = original_direction.x.atan2(original_direction.y);
        let angle_direction = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();
        let projectile_start_position = skeleton_translation + angle_direction * 10.0; // skeleton_pos + direction * offset wanted

        // Sets the projectile texture
        let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
            UVec2::splat(64),
            8,
            1,
            None,
            None,
        ));

        // Creates Projectile
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("s_bone.png"), // Use the texture directly
                transform: Transform {
                    translation: projectile_start_position,
                    scale: Vec3::splat(0.5),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: layout,
                index: 0,
            },
            SkeletonProjectile {
                timer: Timer::from_seconds(0.2, TimerMode::Once),
            },
            Lifetime(SKELETON_PROJECTILE_LIFETIME),
            Velocity {
                v: angle_direction.truncate() * SKELETON_PROJECTILE_SPEED, // (direction * speed of projectile)
            },
            Hitbox {
                size: Vec2::splat(16.),
                offset: Vec2::splat(0.),
                lifetime: Some(Timer::from_seconds(3., TimerMode::Once)),
                entity: SKELETON,
                projectile: true,
                enemy: true,
                boat: false,
            },
            AnimationTimer::new(Timer::from_seconds(0.5, TimerMode::Repeating)),
        ));
    }
}
/*   MOVE_SKELETON_PROJECTILE FUNCTION   */
/// Updates the locations of skeleton projectiles
/// Things to add:
/// * Collision handling, dealing damage on collision
pub fn move_skeleton_projectile(
    mut proj_query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut TextureAtlas,
        &mut SkeletonProjectile,
    )>,
    time: Res<Time>,
) {
    for (mut transform, velocity, mut texture_atlas, mut skeleton) in proj_query.iter_mut() {
        // Calculates/moves the projectile
        transform.translation += velocity.to_vec3(0.) * time.delta_seconds();

        if !skeleton.timer.finished() {
            skeleton.timer.tick(time.delta());
            continue;
        }

        skeleton.timer = Timer::from_seconds(0.2, TimerMode::Once);

        // Animates projectile

        if texture_atlas.index < 7 {
            texture_atlas.index += 1;
        } else {
            texture_atlas.index = 0;
        };
    }
}

/*   SKELETON_PROJ_LIFETIME_CHECK FUNCTION   */
/// Checks the lifetime left on a skeleton's projectile, and despawns
/// after the lifetime expires
pub fn skeleton_proj_lifetime_check(
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

/*   MOVE_skeleton FUNCTION   */
/// Moves the skeleton as long as a player is within agro range
pub fn move_skeleton(
    time: Res<Time>,
    mut skeleton_query: Query<&mut Transform, With<Skeleton>>,
    player_query: Query<&Transform, (With<Player>, Without<Skeleton>)>,
) {
    for mut transform in skeleton_query.iter_mut() {
        //Gets positions (Vec3) of the entities
        let skeleton_translation = transform.translation;
        let player_translation = player_query.single().translation;

        //Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let skeleton_position = skeleton_translation.xy();

        //Gets distance
        let distance_to_player = skeleton_position.distance(player_position);

        //Check
        if distance_to_player > SKELETON_AGRO_RANGE || distance_to_player <= SKELETON_AGRO_STOP {
            continue;
        }

        //Gets direction projectile will be going
        let direction = (player_translation - skeleton_translation).normalize();
        let velocity = direction * SKELETON_MOVEMENT_SPEED;

        //Moves skeleton
        transform.translation += velocity * time.delta_seconds();
    }
}

/*   DESPAWN_ALL_SKELETON_PROJ   */
/// Despawns all the skeleton's projectiles
pub fn despawn_all_skeleton_proj(
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
