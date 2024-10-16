use bevy::prelude::*;

use crate::bat::components::*;
use crate::data::gameworld_data::*;
use crate::hitbox_system::*;
use crate::player::components::Player;

pub fn move_bat(
    time: Res<Time>,
    mut bat_query: Query<&mut Transform, With<Bat>>,
    player_query: Query<&Transform, (With<Player>, Without<Bat>)>,
) {
    for mut transform in bat_query.iter_mut() {
        //Gets positions (Vec3) of the entities
        let bat_translation = transform.translation;
        let player_translation = player_query.single().translation;

        //Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let bat_position = bat_translation.xy();

        //Gets distance
        let distance_to_player = bat_position.distance(player_position);

        if distance_to_player > BAT_AGRO_RANGE || distance_to_player <= BAT_AGRO_STOP_RADIUS {
            continue;
        }

        /* Debug */
        //println!("Bat can attack player! :O");

        //Gets direction projectile will be going
        let direction = (player_translation - bat_translation).normalize();
        let velocity = direction * BAT_MOVEMENT_SPEED;

        transform.translation += velocity * time.delta_seconds();
    }
}
