use bevy::{prelude::*, window::PresentMode};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

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
        .add_systems(Startup, setup)
        .add_systems(Update, show_popup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("1.png"),
        ..default()
    });
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("2.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(3., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("3.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(6., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("4.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(9., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("5.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(12., TimerMode::Once)));
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("6.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(15., TimerMode::Once)));
}

fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = 0.;
        }
    }    
}