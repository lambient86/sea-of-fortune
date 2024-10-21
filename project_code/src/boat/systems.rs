use crate::boat::components::*;
use crate::controls::*;
use crate::data::gameworld_data::*;
use crate::hitbox_system::*;
use crate::player::components::AttackCooldown;
use bevy::prelude::*;

/*   MOVE_BOAT FUNCTION   */
/// Moves and updates the boats position
pub fn move_boat(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Boat, &mut Transform)>,
) {
    let (mut ship, mut transform) = query.single_mut();

    //initializing rotation and movement variables
    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    //getting rotation factor by checking left and right input and subtracting from one another
    //e.g if left pressed and right no : 1 - 0 = 1
    //will accout for both left and right being pressed in one check
    //e.g 1 - 1 = 0
    rotation_factor += get_player_input(PlayerControl::Left, &keyboard_input, &mouse_input)
        - get_player_input(PlayerControl::Right, &keyboard_input, &mouse_input);

    //checking if player is pressing up
    movement_factor = get_player_input(PlayerControl::Up, &keyboard_input, &mouse_input);

    //increasing acceleration if needed
    if ship.acceleration <= MAX_ACCEL && movement_factor == 1. {
        ship.acceleration += 3.;
    } else if ship.acceleration > 0. {
        ship.acceleration -= 7.;
    } else if ship.acceleration < 0. {
        ship.acceleration = 0.;
    }

    //transforming the players rotation
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    //getting movement information
    let movement_dir = transform.rotation * Vec3::Y;
    let movement_dis = movement_factor * (ship.movement_speed * time.delta_seconds())
        + (0.5 * ship.acceleration * time.delta_seconds());
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

    //getting hurtbox information
    let hurtbox_size = Vec2::new(50., 50.);
    let hurtbox_offset = Vec2::new(0., 0.);

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
        Boat {
            movement_speed: 150.,
            rotation_speed: f32::to_radians(100.0),
            acceleration: 0.,
        },
        AttackCooldown {
            remaining: Timer::from_seconds(1.5, TimerMode::Once),
        },
        Hurtbox {
            size: hurtbox_size,
            offset: hurtbox_offset,
            entity: BOAT,
            colliding: false,
            iframe: Timer::from_seconds(1., TimerMode::Once),
        },
    ));
}

/*   BOAT_ATTACK FUNCTION   */
/// Function that fires the cannonball from the boat as an attack
pub fn boat_attack(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    curr_mouse_pos: ResMut<CurrMousePos>,
    time: Res<Time>,
    mut boat_query: Query<(&Transform, &mut AttackCooldown), With<Boat>>,
    asset_server: Res<AssetServer>,
) {
    for (boat_transform, mut cooldown) in boat_query.iter_mut() {
        // checking cooldown
        if !cooldown.remaining.finished() {
            cooldown.remaining.tick(time.delta());
            break;
        }

        /***   ATTACK   ***/
        if get_player_input(PlayerControl::Attack, &keyboard_input, &mouse_input) == 1. {
            cooldown.remaining = Timer::from_seconds(1.5, TimerMode::Once);

            //getting cannonball sprite
            let cannonball_handler = asset_server.load("s_cannonball.png");

            //getting angle to fire at
            let pos2 = curr_mouse_pos.0;
            let original_direction =
                (Vec3::new(pos2.x, pos2.y, 0.) - boat_transform.translation).normalize();
            let angle = original_direction.x.atan2(original_direction.y);
            let firing_angle = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();

            //getting start position to fire from
            let projectile_start_position = boat_transform.translation.xyz();

            //getting hitbox info
            let hitbox_size = Vec2::new(32., 32.);
            let offset = Vec2::new(0., 0.);

            //spawning cannonball
            commands.spawn((
                SpriteBundle {
                    texture: cannonball_handler,
                    transform: Transform {
                        translation: projectile_start_position,
                        scale: Vec3::splat(1.5),
                        ..default()
                    },
                    ..default()
                },
                Cannonball,
                Lifetime(CANNONBALL_LIFETIME),
                CannonballVelocity {
                    v: firing_angle * CANNONBALL_SPEED, /* (direction * speed of projectile) */
                },
                Hitbox {
                    size: hitbox_size,
                    offset: offset,
                    lifetime: Some(Timer::from_seconds(CANNONBALL_LIFETIME, TimerMode::Once)),
                    entity: BOAT,
                },
            ));
        }
    }
}

/*   MOVE_CANNONBALL FUNCTION   */
/// Updates the locations of boat projectiles
pub fn move_cannonball(
    mut proj_query: Query<(&mut Transform, &mut CannonballVelocity), With<Cannonball>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in proj_query.iter_mut() {
        // Calculates/moves the projectile
        transform.translation += velocity.v * time.delta_seconds();
    }
}

/*   CANNONBALL_LIFETIME_CHECK FUNCTION   */
/// Checks the lifetime of a cannonball
pub fn cannonball_lifetime_check(
    time: Res<Time>,
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        lifetime.0 -= time.delta_seconds();
        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn();

            /* Debug */
            println!("Cannonball despawned");
        }
    }
}

/*   DESPAWN_BOAT FUNCTION   */
/// Despawns the boat
/// DEBUG: Will despawn any and all boats
pub fn despawn_boat(mut commands: Commands, query: Query<Entity, With<Boat>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/*   DESPAWN_CANNONBALLS FUNCTION   */
/// Despawns cannonballs
/// DEBUG: Will despawn any and all cannonballs
pub fn despawn_cannonballs(mut commands: Commands, query: Query<Entity, With<Cannonball>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
