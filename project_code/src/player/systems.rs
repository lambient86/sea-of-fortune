use bevy::{prelude::*};
use crate::controls::*;
use crate::player::components::*;
use crate::data::gameworld_data::*;
use crate::hitbox_system::*;
use bevy::input::mouse::{self, MouseButtonInput};




/// The speed at which the player accelerates
pub const ACCELERATION: f32 = 5000.;
pub const SPEED: f32 = 500.;
pub const SIZE: f32 = 32.;
pub const ANIMATION_TIME: f32 = 0.1;

/*   MOVE_PLAYER FUNCTION */
/// Moves the player, updating its position depending on
/// button pressed and players current velocity
pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut player_transform, mut player_velocity) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    //checking left/right input and deciding movement
    //if left pressed and right not : 0 - 1 = -1
    deltav.x = get_player_input( PlayerControl::Right, &keyboard_input, &mouse_input) - get_player_input( PlayerControl::Left, &keyboard_input, &mouse_input);

    //checking up/down input and deciding movement
    //if up pressed and down not; 0 - 1 = -1
    deltav.y = get_player_input(PlayerControl::Up, &keyboard_input,&mouse_input) - get_player_input( PlayerControl::Down, &keyboard_input, &mouse_input);

    //getting acceleration
    let delta_t = time.delta_seconds();
    let acc = ACCELERATION * delta_t;

    //deciding player velocity
    player_velocity.v = if deltav.length() > 0. {
        (player_velocity.v + (deltav.normalize_or_zero() * acc)).clamp_length_max(SPEED)
    } else if player_velocity.v.length() > acc {
        player_velocity.v + (player_velocity.v.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };

    //getting change in location
    let change = player_velocity.v * delta_t;

    //setting new player x position if within bounds
    let new_position = player_transform.translation + Vec3::new(change.x, 0., 0.);
    if new_position.x >= -(LEVEL_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_position.x <= LEVEL_W / 2. - (TILE_SIZE as f32) / 2.
    {
        player_transform.translation = new_position;
    }

    //setting new player y position if within bounds
    let new_pos = player_transform.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(LEVEL_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= LEVEL_H / 2. - (TILE_SIZE as f32) / 2.
    {
        player_transform.translation = new_pos;
    }
}

/*   PLAYER_ANIMATION FUNCTION   */
/// Animates the player sprite depending on the movement of
/// the player
pub fn player_animation(
    time: Res<Time>,
    mut player_query: Query<
        (
            &Velocity,
            &mut TextureAtlas,
            &mut AnimationTimer,
            &AnimationFrameCount,
            &mut Player
        ),
        With<Player>,
    >,
) {
    let (velocity, mut texture_atlas, mut timer, frame_count, mut player) = player_query.single_mut();
        //deciding what animation to use
        let new_state = if velocity.v.cmpeq(Vec2::ZERO).all() {
            SpriteState::Idle
        } else if velocity.v.x < 0. {
            SpriteState::LeftRun         
        } else if velocity.v.x > 0. {
            SpriteState::RightRun
        } else if velocity.v.y < 0. {
            SpriteState::ForwardRun
        } else if velocity.v.y > 0. {
            SpriteState::BackwardRun
        } else {
            SpriteState::Idle
        };

        //changing player animation state if needed
        if new_state != player.animation_state {
            player.animation_state = new_state;
            player.timer = Timer::from_seconds(
                player.animation_state.animation_speed(),
                TimerMode::Repeating,
            );
            
            //setting animation at start
            let start = player.animation_state.animation_indices();
            texture_atlas.index = start.start;
        }

        //passing time
        player.timer.tick(time.delta());

        //going to next frame
        if player.timer.just_finished() {
            let indices = player.animation_state.animation_indices();
            texture_atlas.index = if texture_atlas.index + 1 >= indices.end {
                indices.start
            } else {
                texture_atlas.index + 1
            };
        }
}

/*   SPAWN_PLAYER FUNCTION */
/// Spawns the player in the gameworld
pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    //getting sprite info
    let master_handle: Handle<Image> = asset_server.load("s_pirate.png");
    let master_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 8, 5, None, None);
    let master_layout_length = master_layout.textures.len();
    let master_layout_handle = texture_atlases.add(master_layout);

    //setting up player for spawning
    commands.spawn((
      SpriteBundle {
        texture: master_handle,
        transform: Transform {
            scale: Vec3::splat(2.0),
            ..default()
        },
        ..default()
      },
      TextureAtlas {
        layout: master_layout_handle,
        index: 0,
      },
      AnimationTimer::new(Timer::from_seconds(ANIMATION_TIME, TimerMode::Repeating)),
      AnimationFrameCount::new(master_layout_length),
      Velocity::new(),
      AttackCooldown {remaining: 0.0},
      Player {
        animation_state: SpriteState::Idle,
        timer: Timer::from_seconds(SpriteState::Idle.animation_speed(), TimerMode::Repeating),
        health: 3,
        max_health: 3,
      },
      TestTimer::new(Timer::from_seconds(1., TimerMode::Repeating)),
    ));

}



/*   PLAYER_ATTACK FUNCTION   */
/// Checks if player pressed attack input. If the player has attacked, the
/// current weapons attack is then used
/*   PLAYER_ATTACK FUNCTION   */
/// Checks if player pressed attack input. If the player has attacked, the
/// current weapon's attack is then used
pub fn player_attack(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>, 
    mut cursor: EventReader<CursorMoved>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut AttackCooldown), With<Player>>,
) {
    for (entity, transform, mut cooldown) in player_query.iter_mut() {
        // Attacking only when cooldown is over
        if cooldown.remaining > 0.0 {
            cooldown.remaining -= time.delta_seconds();
        }

        for ev in cursor.read() {
            let cursor_direction = ev.position.trunc();
            println!("Cursor direction: X: {}, Y: {}", cursor_direction.x, cursor_direction.y);
        }
        
        /*// Check if the left mouse button is pressed
        if get_player_input(PlayerControl::Attack, &keyboard_input, &mouse_input) == 1. && cooldown.remaining <= 0. {
            println!("Player attacked!");
            
            // Player position
            let player_position = transform.translation.truncate();

            // Calculate hitbox position
            let hitbox_position = transform.translation.truncate(); 

            // Define the size of the hitbox
            let hitbox_size = Vec2::new(50.0, 50.0); // Example size

            // Create the hitbox 
            create_hitbox(
                &mut commands,
                entity,
                hitbox_size,
                hitbox_position,
                Some(30.0), 
            );

            cooldown.remaining = 1.0; 
        
        }*/
    }
    
}

/*
 Function checks the current state of the player's health
 if current health == 0 --> panic and close program

*/
pub fn check_player_health(
    time: Res<Time>,
    mut player_query: Query<(&mut Player,&mut TestTimer), With<Player>>,
) {

    for (mut player, mut timer) in player_query.iter_mut() {
        if player.health <= 0 {
            panic!("Health reached {}...You died :(", player.health);
        }
    
        timer.tick(time.delta());
        print!("health: {}\n", player.health);
    
        if timer.just_finished() {
            player.health -=1;
        }
    }
}