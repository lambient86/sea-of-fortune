<<<<<<< Updated upstream
=======
mod player;
mod bat;
mod boat;
mod data;
mod systems;
mod components;
// hit/hurt boxes
mod hitbox_system;
use hitbox_system::HitboxPlugin;

use data::gameworld_data::*;
>>>>>>> Stashed changes
use bevy::{prelude::*, window::PresentMode};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Component, Clone)]
struct ZIndex {
    z_index: f32,
}
fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sea of Fortune".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
<<<<<<< Updated upstream
        .add_systems(Startup, setup)
        .add_systems(Update, show_popup)
=======
        .add_systems(Startup, setup_gameworld)
        .add_plugins(PlayerPlugin)
        .add_systems(Update, move_camera)
        .add_plugins(HitboxPlugin)
>>>>>>> Stashed changes
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("Theo.png"),
        ..default()
    });
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Zac.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(3., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Tim.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(6., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Anna.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(9., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Mark.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(12., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Andrew.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(15., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("end.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(18., TimerMode::Once)));

    commands.spawn(ZIndex{z_index: 0.});
}

fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>, mut index: Query<&mut ZIndex>) {

    let mut _z = index.single_mut();

    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = _z.z_index;
            _z.z_index +=1.;
            print!("z level: {}\n", _z.z_index);       // debug
        }
    }    
}
