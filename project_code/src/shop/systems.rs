use super::components::*;
use crate::enemies::*;
use crate::player::components::Player;
use crate::player::components::Sword;
use crate::boat::components::Boat;
use crate::player;
use bevy::prelude::*;

pub fn setup_shop_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            ShopUI,
        ));
}

pub fn update_shop_text(
    mut text_query: Query<(&mut Text, &Parent)>,
    button_query: Query<(&ShopButton, &Parent)>,
    player: Query<&Player>,
    shop: Res<Shop>,
) {
    let player = player.single();

    for (mut text, parent) in text_query.iter_mut() {
        // Update gold display
        if text.sections[0].value.starts_with("Gold:") {
            text.sections[0].value = format!("Gold: {}", player.inventory.money);
            continue;
        }

        // Update button texts
        if let Ok((button, _)) = button_query.get(parent.get()) {
            match button {
                ShopButton::UpgradeItem(index) => {
                    if let Some(shop_item) = shop.items.get(*index) {
                        if let Some(owned_item) = player
                            .inventory
                            .items
                            .iter()
                            .find(|i| i.item_type == shop_item.item_type)
                        {
                            text.sections[0].value =
                                format!("Upgrade {} (Lvl {})", shop_item.name, owned_item.level);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn rebuild_shop_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    shop: Res<Shop>,
    player: Query<&Player>,
    shop_page: Res<ShopPage>,
    shop_ui_query: Query<Entity, With<ShopUI>>,
) {
    if !shop_page.is_changed() {
        return;
    }

    let player = player.single();
    let shop_ui_entity = shop_ui_query.single();
    let font = asset_server.load("pixel_pirate.ttf");

    commands.entity(shop_ui_entity).despawn_descendants();

    commands.entity(shop_ui_entity).with_children(|parent| {
        // Background Panel
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(600.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            })
            .with_children(|panel| {
                // Title
                panel.spawn(TextBundle::from_section(
                    "Shop",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ));

                // Player's money
                panel.spawn(TextBundle::from_section(
                    format!("Gold: {}", player.inventory.money),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::srgb(1.0, 0.84, 0.0),
                    },
                ));

                // Content based on current page
                match *shop_page {
                    ShopPage::PlayerUpgrades => {
                        // Weapon upgrades
                        for (index, item) in player.inventory.items.iter().enumerate() {
                            if matches!(
                                item.item_type,
                                ItemType::Sword | ItemType::Pistol | ItemType::Dagger | ItemType::Musket
                            ) {
                                panel.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(250.0),
                                            height: Val::Px(65.0),
                                            margin: UiRect::all(Val::Px(10.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border: UiRect::all(Val::Px(2.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                                        border_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    ShopButton::UpgradeItem(index),
                                ))
                                .with_children(|button| {
                                    button.spawn(TextBundle::from_section(
                                        format!(
                                            "Upgrade {} (Lvl {})\n{} gold",
                                            item.name, item.level, item.price
                                        ),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 18.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                            }
                        }

                        // Player HP upgrade button
                        panel.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(250.0),
                                    height: Val::Px(65.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                                border_color: Color::WHITE.into(),
                                ..default()
                            },
                            ShopButton::UpgradePlayerHealth,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                format!("Upgrade Player Health\n{} gold", PLAYER_HEALTH_UPGRADE_COST),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    }
                    ShopPage::ShipUpgrades => {
                        for (text, button_type, cost) in [
                            ("Boat Speed", ShopButton::UpgradeBoatSpeed, BOAT_SPEED_UPGRADE_COST),
                            ("Boat Health", ShopButton::UpgradeBoatHealth, BOAT_HEALTH_UPGRADE_COST),
                            (
                                "Boat Control",
                                ShopButton::UpgradeBoatRotation,
                                BOAT_ROTATION_UPGRADE_COST,
                            ),
                            (
                                "Cannon Damage",
                                ShopButton::UpgradeBoatCannon,
                                BOAT_CANNON_UPGRADE_COST,
                            ),
                        ] {
                            panel.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(250.0),
                                        height: Val::Px(65.0),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                                    border_color: Color::WHITE.into(),
                                    ..default()
                                },
                                button_type,
                            ))
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    format!("Upgrade {}\n{} gold", text, cost),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 18.0,
                                        color: Color::WHITE,
                                    },
                                ));
                            });
                        }
                    }
                    ShopPage::Sell => {
                        for (index, item) in player.inventory.items.iter().enumerate() {
                            if item.item_type == ItemType::Loot {
                                panel.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(250.0),
                                            height: Val::Px(65.0),
                                            margin: UiRect::all(Val::Px(10.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border: UiRect::all(Val::Px(2.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                                        border_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    ShopButton::SellItem(index),
                                ))
                                .with_children(|button| {
                                    button.spawn(TextBundle::from_section(
                                        format!("Sell {} for {} gold", item.name, item.price / 2),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 18.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                            }
                        }
                    }
                }

                // Navigation buttons at bottom
                for (text, button_type) in [
                    ("Player Upgrades", ShopButton::PlayerUpgrades),
                    ("Ship Upgrades", ShopButton::ShipUpgrades),
                    ("Sell Items", ShopButton::Sell),
                ] {
                    panel.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                            border_color: Color::WHITE.into(),
                            ..default()
                        },
                        button_type,
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            text,
                            TextStyle {
                                font: font.clone(),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
                }
            });
    });
}

pub fn cleanup_shop_ui(mut commands: Commands, query: Query<Entity, With<ShopUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_button_interactions(
    mut interaction_query: Query<(&Interaction, &ShopButton, &mut BackgroundColor), With<Button>>,
    mut shop_events: EventWriter<ShopEvent>,
    mut shop_page: ResMut<ShopPage>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.35, 0.35).into();
                match button {
                    ShopButton::PlayerUpgrades => {
                        *shop_page = ShopPage::PlayerUpgrades;
                    }
                    ShopButton::ShipUpgrades => {
                        *shop_page = ShopPage::ShipUpgrades;
                    }
                    ShopButton::Sell => {
                        *shop_page = ShopPage::Sell;
                    }
                    ShopButton::UpgradeItem(index) => {
                        shop_events.send(ShopEvent::Upgrade(*index));
                    }
                    ShopButton::SellItem(index) => {
                        shop_events.send(ShopEvent::Sell(*index));
                    }
                    ShopButton::UpgradePlayerHealth => {
                        shop_events.send(ShopEvent::UpgradePlayerHealth);
                    }
                    ShopButton::UpgradeBoatSpeed => {
                        shop_events.send(ShopEvent::UpgradeBoatSpeed);
                    }  
                    ShopButton::UpgradeBoatHealth => {
                        shop_events.send(ShopEvent::UpgradeBoatHealth);
                    }
                    ShopButton::UpgradeBoatRotation => {
                        shop_events.send(ShopEvent::UpgradeBoatRotation);
                    }
                    ShopButton::UpgradeBoatCannon => {
                        shop_events.send(ShopEvent::UpgradeBoatCannon);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

// Sword Damage and Upgrade Mechanics
pub fn update_sword_damage(
    mut player_query: Query<(&Player, &Children)>,
    mut sword_query: Query<&mut Sword>,
) {
    for (player, children) in player_query.iter_mut() {
        // Find the sword in player's children
        for &child in children.iter() {
            if let Ok(mut sword) = sword_query.get_mut(child) {
                // Find the sword item in inventory to get its level
                if let Some(sword_item) = player
                    .inventory
                    .items
                    .iter()
                    .find(|item| item.item_type == ItemType::Sword)
                {
                    sword.upgrade_sword(sword_item.level);
                }
            }
        }
    }
}

pub fn handle_shop_events(
    mut shop_events: EventReader<ShopEvent>,
    mut player_query: Query<(&mut Player, &Children)>,
    mut sword_query: Query<&mut Sword>,
    mut boat_query: Query<&mut Boat>,
    time: Res<Time>,
    mut cooldown: ResMut<ShopCooldown>,
) {
    if !cooldown.0.finished() {
        cooldown.0.tick(time.delta());
        return;
    }

    for event in shop_events.read() {
        cooldown.0.reset();

        let (mut player, children) = player_query.single_mut();
        let mut boat = boat_query.single_mut();

        match event {
            ShopEvent::Upgrade(index) => {
                let item_type = if let Some(item) = player.inventory.items.get(*index) {
                    let cost = item.price;
                    if player.inventory.money >= cost {
                        Some((item.item_type, cost))
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some((item_type, cost)) = item_type {
                    if player.inventory.find_and_upgrade_item(item_type) {
                        player.inventory.money -= cost;

                        if item_type == ItemType::Sword {
                            update_sword_for_player(&mut player, children, &mut sword_query);
                        }
                    }
                }
            }
            ShopEvent::Sell(index) => {
                if let Some(item) = player.inventory.remove_item(*index) {
                    player.inventory.money += item.price / 2;
                }
            }
            ShopEvent::UpgradePlayerHealth => {
                let health_upgrade_cost = PLAYER_HEALTH_UPGRADE_COST;
                if player.inventory.money >= health_upgrade_cost {
                    player.max_health += 1.0;
                    player.health = player.max_health;
                    player.inventory.money -= health_upgrade_cost;
                }
            },
            ShopEvent::UpgradeBoatSpeed => {
                let boat_speed_cost = BOAT_SPEED_UPGRADE_COST;
                if player.inventory.money >= boat_speed_cost {
                    boat.movement_speed *= 1.10; // 10% increase
                    player.inventory.money -= boat_speed_cost;
                }
            },
            ShopEvent::UpgradeBoatHealth => {
                let boat_health_cost = BOAT_HEALTH_UPGRADE_COST;
                if player.inventory.money >= boat_health_cost {
                    boat.max_health += 25.0;
                    boat.health = boat.max_health;
                    player.inventory.money -= boat_health_cost;
                }
            },
            ShopEvent::UpgradeBoatRotation => {
                let boat_rotation_cost = BOAT_ROTATION_UPGRADE_COST;
                if player.inventory.money >= boat_rotation_cost {
                    boat.rotation_speed *= 1.05; // 5% increase
                    player.inventory.money -= boat_rotation_cost;
                }
            },
            ShopEvent::UpgradeBoatCannon => {
                let boat_cannon_cost = BOAT_CANNON_UPGRADE_COST;
                if player.inventory.money >= boat_cannon_cost {
                    boat.cannon_damage *= 1.25; // 25% increase
                    player.inventory.money -= boat_cannon_cost;
                }
            },
        }
    }
}

fn update_sword_for_player(
    player: &mut Player,
    children: &Children,
    sword_query: &mut Query<&mut Sword>,
) {
    if let Some(sword_item) = player
        .inventory
        .items
        .iter()
        .find(|item| item.item_type == ItemType::Sword)
    {
        // Find and update the sword entity
        for &child in children.iter() {
            if let Ok(mut sword) = sword_query.get_mut(child) {
                sword.upgrade_sword(sword_item.level);
            }
        }
    }
}

pub fn generate_loot_item(enemy_type: EnemyT) -> Item {
    match enemy_type {
        EnemyT::RSkeleton => Item::new(ItemType::Loot, "Bone".to_string(), 100),
        EnemyT::Bat => Item::new(ItemType::Loot, "Bat Wing".to_string(), 100),
        EnemyT::Rock => Item::new(ItemType::Loot, "Rock Shard".to_string(), 150),
        _ => Item::new(ItemType::Loot, "Nothing".to_string(), 0),
    }
}
