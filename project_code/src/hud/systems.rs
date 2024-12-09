use bevy::prelude::*;

use crate::{
    boat::components::Boat,
    player::components::Player,
    wind::components::{CountdownTimer, Wind},
};

use super::components::*;

pub fn init_player_hud(mut commands: Commands, asset_server: Res<AssetServer>, _wind: Res<Wind>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            PlayerHUD,
        ))
        .with_children(|parent| {
            // Health Display
            parent.spawn((
                TextBundle::from_section(
                    "Health: 3/3",
                    TextStyle {
                        font: asset_server.load("pixel_pirate.ttf"),
                        font_size: 32.0,
                        color: Color::srgb(242.0, 231.0, 218.0),
                    },
                ),
                PlayerHPText,
            ));

            // Gold Display
            parent.spawn((
                TextBundle::from_section(
                    "Gold: 100",
                    TextStyle {
                        font: asset_server.load("pixel_pirate.ttf"),
                        font_size: 32.0,
                        color: Color::srgb(242.0, 231.0, 218.0),
                    },
                ),
                GoldText,
            ));
        });
}

pub fn init_ship_hud(mut commands: Commands, asset_server: Res<AssetServer>, _wind: Res<Wind>) {
    commands.insert_resource(ShipStats {
        hp: 100.0,
        gold: 50,
        wind_dir: _wind.direction,
    });

    let font_handle: Handle<Font> = asset_server.load("pixel_pirate.ttf");
    let arrow_handle: Handle<Image> = asset_server.load("s_arrow.png");
    commands.insert_resource(ArrowTS(arrow_handle.clone()));

    let dot = Vec2::new(0., 1.).dot(_wind.direction);
    let mag_w = _wind.direction.length();
    let mag_b = Vec2::new(0., 1.).length();
    let cs = dot / (mag_b * mag_w);
    let angle = cs.acos();

    // money and hp container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ShipHUD,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|stats_parent| {
                    stats_parent.spawn((
                        TextBundle::from_section(
                            format!("Health: {}/{}", 10, 10),
                            TextStyle {
                                font: font_handle.clone(),
                                font_size: 32.0,
                                color: Color::srgb(242.0, 231.0, 218.0),
                            },
                        ),
                        PlayerHPText,
                    ));
                    stats_parent.spawn((
                        TextBundle::from_section(
                            format!("Gold: {}", 100),
                            TextStyle {
                                font: font_handle.clone(),
                                font_size: 32.0,
                                color: Color::srgb(242.0, 231.0, 218.0),
                            },
                        ),
                        GoldText,
                    ));
                    stats_parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(500.),
                                height: Val::Px(500.),
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Start,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|arrow_parent| {
                            arrow_parent.spawn((
                                TextBundle::from_section(
                                    format!("Wind:"),
                                    TextStyle {
                                        font: font_handle.clone(),
                                        font_size: 32.0,
                                        color: Color::srgb(242.0, 231.0, 218.0),
                                    },
                                ),
                                WindText,
                            ));
                            arrow_parent.spawn((
                                ImageBundle {
                                    image: UiImage::new(arrow_handle),
                                    transform: Transform {
                                        rotation: Quat::from_rotation_z(angle), // Rotate around Z-axis
                                        ..default() // Default other transform values (position, scale)
                                    },
                                    ..default()
                                },
                                Arrow,
                            ));
                        });
                });
        });
}

pub fn update_player_hud(
    player_query: Query<(&Player)>,
    mut text_query: Query<(&mut Text, Option<&PlayerHPText>, Option<&GoldText>)>,
) {
    if let Ok(player) = player_query.get_single() {
        for (mut text, health, gold) in text_query.iter_mut() {
            if health.is_some() {
                text.sections[0].value = format!("Health: {}/{}", player.health, player.max_health);
            } else if gold.is_some() {
                text.sections[0].value = format!("Gold: {}", player.inventory.money);
            }
        }
    }
}

pub fn update_ship_hud(
    player_query: Query<&Player>,
    ship_query: Query<&Boat>,
    _wind: Res<Wind>,
    mut query: Query<&mut CountdownTimer>,
    mut text_query: Query<(&mut Text, Option<&ShipHPText>, Option<&GoldText>)>,
    mut arrow_q: Query<&mut Transform, With<Arrow>>,
) {
    if let Ok(ship) = ship_query.get_single() {
        if let Ok(player) = player_query.get_single() {
            for (mut text, health, gold) in text_query.iter_mut() {
                if health.is_some() {
                    text.sections[0].value = format!("Health: {}/{}", ship.health, ship.max_health);
                }
                if gold.is_some() {
                    text.sections[0].value = format!("Gold: {}", player.inventory.money);
                }

                if let Ok(mut arrow) = arrow_q.get_single_mut() {
                    if let Ok(timer) = query.get_single() {
                        if timer.timer.just_finished() {
                            let dot = Vec2::new(0., 1.).dot(_wind.direction);
                            let mag_w = _wind.direction.length();
                            let mag_b = Vec2::new(0., 1.).length();
                            let cs = dot / (mag_b * mag_w);
                            let angle = cs.acos();

                            arrow.rotation = Quat::from_rotation_z(0.0);
                            arrow.rotate_z(angle);
                        }
                    }
                }
            }
        }
    }
}
