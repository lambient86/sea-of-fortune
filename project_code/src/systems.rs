use crate::player::components::*;
use crate::player::systems::*;
use bevy::{prelude::*, window::PresentMode};
use crate::data::gameworld_data::*;
use crate::components::*;


/// Updates the cameras position to center the current player entity
/// and tracks the player wherever they go
pub fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;
    ct.translation.x = pt.translation.x.clamp(-x_bound, x_bound);
    ct.translation.y = pt.translation.y.clamp(-y_bound, y_bound);
}

pub fn setup_gameworld(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>) {
    commands.spawn(Camera2dBundle::default());

    let bg_texture_handle = asset_server.load("bg_sand_demo.png");

    commands
        .spawn(SpriteBundle {
            texture: bg_texture_handle.clone(),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(Background);
}