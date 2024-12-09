use bevy::prelude::*;
use bevy::render::texture;

use crate::data::gameworld_data::*;
use crate::enemies::*;
use crate::hitbox_system::*;
use crate::player::components::*;
use crate::rock::components::*;
use crate::shop::systems::generate_loot_item;

/*  Spawn Rock FUNCTION  */
/// Spawns a kraken entity in the gameworld
pub fn spawn_rock(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let transform = Transform::from_xyz(0., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.);

    spawn_enemy(
        &mut commands,
        EnemyT::Rock,
        transform,
        &asset_server,
        &mut texture_atlases,
    );
}

/*   ROCK_DAMAGED FUNCTION   */
/// Current functionality: Detects when a player is within player attack range (this will later be replaced with
// player weapon/attack collision) and then takes 1 damage (dies)
pub fn rock_damaged(
    mut commands: Commands,
    mut rock_query: Query<(&mut Rock, Entity, &mut Hurtbox, &Transform), With<Rock>>,
    mut player_query: Query<&mut Player>,
) {
    for (mut rock, entity, mut hurtbox, transform) in rock_query.iter_mut() {
        if !hurtbox.colliding.is {
            continue;
        }

        rock.current_hp -= hurtbox.colliding.dmg;
        hurtbox.colliding.dmg = 0.;

        if rock.current_hp <= 0. {
            println!("Rock was attacked by player, it is dead :(");
            let loot = generate_loot_item(EnemyT::Rock);
            if loot.price > 0 {
                println!("Rock dropped: {}", loot.name);
                if let Ok(mut player) = player_query.get_single_mut() {
                    player.inventory.add_item(loot);
                }
            }
            commands.entity(entity).despawn();
        } else {
            println!("Rock was attacked by player");
        }

        hurtbox.colliding.is = false;
    }
}

/*   DESPAWN_ALL_ROCK FUNCTION   */
/// Despawns a kraken entity
/// DEBUG: Despwans all kraken entities
pub fn despawn_all_rocks(mut commands: Commands, query: Query<Entity, With<Rock>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/*   Move rock FUNCTION   */
/// Moves the rock as long as a player is within agro range
pub fn move_rock(
    time: Res<Time>,
    mut kraken_query: Query<&mut Transform, With<Rock>>,
    player_query: Query<&Transform, (With<Player>, Without<Rock>)>,
) {
    for mut transform in kraken_query.iter_mut() {
        //Gets positions (Vec3) of the entities
        let kraken_translation = transform.translation;
        let player_translation = player_query.single().translation;

        //Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let kraken_position = kraken_translation.xy();

        //Gets distance
        let distance_to_player = kraken_position.distance(player_position);

        //Check
        if distance_to_player > ROCK_AGRO_RANGE {
            continue;
        }

        //Gets direction projectile will be going
        let direction = (player_translation - kraken_translation).normalize();
        let velocity = direction * ROCK_MOVEMENT_SPEED;

        //Moves rock
        transform.translation += velocity * time.delta_seconds();
    }
}
