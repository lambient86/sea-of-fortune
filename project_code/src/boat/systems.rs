use bevy::prelude::*;
use crate::controls::*;
use crate::boat::components::*;
use crate::data::gameworld_data::*;

/*   MOVE_BOAT FUNCTION   */
/// Moves and updates the boats position
pub fn move_boat(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&Boat, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();

    //initializing rotation and movement variables
    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    //getting rotation factor by checking left and right input and subtracting from one another
    //e.g if left pressed and right no : 1 - 0 = 1
    //will accout for both left and right being pressed in one check
    //e.g 1 - 1 = 0
    rotation_factor += get_player_input(PlayerControl::Left, &keyboard_input, &mouse_input) - get_player_input(PlayerControl::Right, &keyboard_input, &mouse_input); 

    //checking if player is pressing up
    movement_factor = get_player_input(PlayerControl::Up, &keyboard_input, &mouse_input);

    //transforming the players rotation
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    //getting movement information
    let movement_dir = transform.rotation * Vec3::Y;
    let movement_dis = movement_factor * ship.movement_speed * time.delta_seconds();
    let translation_delta = movement_dir * movement_dis;

    //moving the boat
    transform.translation += translation_delta;

    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

/*  SPAWN_BOAT FUNCTION */
/// Spawns a boat entity for the player to control
pub fn spawn_boat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    //getting boat sprite info
    let boat_sheet_handle = asset_server.load("s_basic_ship.png");
    let boat_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let boat_layout_handle = texture_atlases.add(boat_layout);

    //spawning boat
    commands.spawn((
        SpriteBundle {
            texture: boat_sheet_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: boat_layout_handle.clone(),
            index: 0,
        },
        Boat{
            movement_speed: 250.0,
            rotation_speed: f32::to_radians(180.0),
        },
    ));
}