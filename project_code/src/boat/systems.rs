use bevy::prelude::*;
use crate::controls::*;
use crate::boat::components::*;
use crate::data::gameworld_data::*;
use crate::player::components::AttackCooldown;

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
    let boat_layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 2, 2, None, None);
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

/*   BOAT_ATTACK FUNCTION   */
/// Function that fires the cannonball from the boat as an attack
pub fn boat_attack(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut boat_query: Query<(&Transform, &mut AttackCooldown), With<Boat>>,
    asset_server: Res<AssetServer>,
) {
    for (boat_transform, mut cooldown) in boat_query.iter_mut() {
        // Attacks only when cooldown is over
        if !cooldown.remaining.just_finished() {
            cooldown.remaining.tick(time.delta());
            break;
        }

        if get_player_input(PlayerControl::Attack, &keyboard_input, &mouse_input) == 1. {    
            println!("Boat attacked");
            cooldown.remaining = Timer::from_seconds(1.5, TimerMode::Once);

            
            //getting cannonball sprite
            let cannonball_handler = asset_server.load("s_cannonball.png");

            //getting angle to fire at
            let firing_angle = Vec2::new(boat_transform.rotation.x, boat_transform.rotation.y);

            //getting start position to fire from
            let projectile_start_position = boat_transform.translation.xyz();

            //spawning cannonball
            commands.spawn((
                SpriteBundle {
                    texture: cannonball_handler,
                    transform: Transform::from_translation(projectile_start_position),
                    ..default()
                },
                Cannonball,
                Lifetime,
                Velocity {
                    v: firing_angle * CANNONBALL_SPEED, /* (direction * speed of projectile) */
                },
            ));
        }
    }
}

/*   MOVE_CANNONBALL FUNCTION   */
/// Updates the locations of bat projectiles
/// Things to add:
/// * Collision handling, dealing damage on collision
pub fn move_cannonball(
    mut proj_query: Query<(&mut Transform, &mut Velocity), With<Cannonball>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in proj_query.iter_mut() {
        // Calculates/moves the projectile
        transform.translation += velocity.v * time.delta_seconds();
    }
}

/*   DESPAWN_BOAT FUNCTION   */
/// Despawns the boat
/// DEBUG: Will despawn any and all boats
pub fn despawn_boat(
    mut commands: Commands,
    query: Query<Entity, With<Boat>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}