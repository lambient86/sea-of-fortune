use bevy::prelude::*;
use bevy::render::texture;
use bevy::sprite::TextureAtlas;

use crate::data::gameworld_data::*;
use crate::enemies::*;
use crate::hitbox_system::*;
use crate::player::components::*;
use crate::poison_skeleton::components::*;
use crate::poison_skeleton::components::Lifetime as PoisonSkeletonLifetime;


/*   ROTATE_skeleton FUNCTION   */
/// This should be changed to a function called "track_player", which will
/// be how the skeleton knows where to check where the player is for shooting projectiles
///
/// WE DON'T NEED TO ROTATE THE skeleton! I WILL MAKE A BACK FACING SPRITE IF NEEDED
pub fn rotate_pskeleton(
    time: Res<Time>,
    mut query: Query<(&PoisonSkeleton, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // getting player position
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for (pskeleton, mut enemy_transform) in &mut query {
        //getting skeleton's position relative to player position
        let pskeleton_position = enemy_transform.translation.xy();
        let distance_to_player = pskeleton_position.distance(player_translation);

        //ensuring skeleton is close enough to player to attack
        if distance_to_player > PSKELETON_ATTACK_DIST {
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
            rotation_sign * (pskeleton.rotation_speed * time.delta_seconds()).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);
    }
}

/*   ANIMATE_SKELETON FUNCTION   */
/// Animates a skeleton entity
pub fn animate_pskeleton(
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

pub fn spawn_pskeleton(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let transform = Transform::from_xyz(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.)
        .with_scale(Vec3::splat(2.0));

    // Spawning skeleton 1
    spawn_enemy(
        &mut commands,
        EnemyT::PoisonSkeleton,
        transform,
        &asset_server,
        &mut texture_atlases,
    );
}

/*   Skeleton_DAMAGED FUNCTION   */
/// Current functionality: Detects when a player is within player attack range (this will later be replaced with
// player weapon/attack collision) and then takes 1 damage (dies)
pub fn pskeleton_damaged(
    mut commands: Commands,
    mut pskeleton_query: Query<(&mut PoisonSkeleton, Entity, &mut Hurtbox), With<PoisonSkeleton>>,
) {
    for (mut pskeleton, entity, mut hurtbox) in pskeleton_query.iter_mut() {
        if !hurtbox.colliding {
            continue;
        }

        pskeleton.current_hp -= 1.;

        if pskeleton.current_hp <= 0. {
            println!("Poison skeleton was attacked by player, it is dead :(");
            commands.entity(entity).despawn();
        } else {
            println!("Poison skeleton was attacked by player");
        }

        hurtbox.colliding = false;
    }
}

/*   DESPAWN_ALL_SKELETON FUNCTION   */
/// Despawns a skeleton entity
/// DEBUG: Despwans all skeleton entities
pub fn despawn_all_pskeletons(mut commands: Commands, query: Query<Entity, With<PoisonSkeleton>>) {
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
pub fn pskeleton_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut pskeleton_query: Query<(&Transform, &mut AttackCooldown), With<PoisonSkeleton>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (pskeleton_transform, mut cooldown) in pskeleton_query.iter_mut() {
        // Attacks only when cooldown is over
        cooldown.remaining.tick(time.delta());
        if !cooldown.remaining.just_finished() {
            continue;
        }

        cooldown.remaining = Timer::from_seconds(1.5, TimerMode::Once);

        // Gets positions (Vec3) of the entities
        let pskeleton_translation = pskeleton_transform.translation;
        let player_translation = player_query.single().translation;

        // Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let pskeleton_position = pskeleton_translation.xy();

        // Gets distance
        let distance_to_player = pskeleton_position.distance(player_position);

        if distance_to_player > PSKELETON_ATTACK_DIST {
            continue;
        }

        // Gets direction projectile will be going
        let original_direction = (player_translation - pskeleton_translation).normalize();
        let angle = original_direction.x.atan2(original_direction.y);
        let angle_direction = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();
        let projectile_start_position = pskeleton_translation + angle_direction * 10.0; // skeleton_pos + direction * offset wanted

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
            PSkeletonProjectile {
                timer: Timer::from_seconds(0.2, TimerMode::Once),
            },
            PoisonSkeletonLifetime(PSKELETON_PROJECTILE_LIFETIME),
            Velocity {
                v: angle_direction.truncate() * PSKELETON_PROJECTILE_SPEED, // (direction * speed of projectile)
            },
            Hitbox {
                size: Vec2::splat(16.),
                offset: Vec2::splat(0.),
                lifetime: Some(Timer::from_seconds(3., TimerMode::Once)),
                entity: PSKELETON,
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
pub fn move_pskeleton_projectile(
    mut proj_query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut TextureAtlas,
        &mut PSkeletonProjectile,
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

pub fn pmove_skeleton(
    time: Res<Time>,
    mut pskeleton_query: Query<&mut Transform, With<PoisonSkeleton>>,
    player_query: Query<&Transform, (With<Player>, Without<PoisonSkeleton>)>,
) {
    for mut transform in pskeleton_query.iter_mut() {
        //Gets positions (Vec3) of the entities
        let pskeleton_translation = transform.translation;
        let player_translation = player_query.single().translation;

        //Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let pskeleton_position = pskeleton_translation.xy();

        //Gets distance
        let distance_to_player = pskeleton_position.distance(player_position);

        //Check
        if distance_to_player > PSKELETON_AGRO_RANGE || distance_to_player <= PSKELETON_AGRO_STOP {
            continue;
        }

        //Gets direction projectile will be going
        let direction = (player_translation - pskeleton_translation).normalize();
        let velocity = direction * PSKELETON_MOVEMENT_SPEED;

        //Moves skeleton
        transform.translation += velocity * time.delta_seconds();
    }
}


/*   DESPAWN_ALL_SKELETON_PROJ   */
/// Despawns all the skeleton's projectiles
pub fn despawn_all_pskeleton_proj(
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut PoisonSkeletonLifetime)>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
