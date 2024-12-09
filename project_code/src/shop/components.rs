// In src/shop/components.rs

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemType {
    Sword(f32),
    Dagger(f32),
    Musket(f32),
    Pistol(f32),
    Boat,
    Loot, // For sellable items from enemies
}

#[derive(Clone, Debug, Component)]
pub struct Item {
    pub item_type: ItemType,
    pub name: String,
    pub price: u32,
    pub level: u32,
}

impl Item {
    pub fn new(item_type: ItemType, name: String, price: u32) -> Self {
        Self {
            item_type,
            name,
            price,
            level: 0,
        }
    }

    pub fn upgrade(&mut self) {
        if self.level < 5 {
            self.level += 1;
            self.price += self.price / 2; // Increase price by 50% for each upgrade
        }
    }
}

#[derive(Resource)]
pub struct Shop {
    pub items: Vec<Item>,
}

impl Default for Shop {
    fn default() -> Self {
        Self {
            items: vec![
                Item::new(ItemType::Dagger(0.5), "Dagger".to_string(), 75),
                Item::new(ItemType::Sword(1.), "Sword".to_string(), 100),
                Item::new(ItemType::Pistol(0.5), "Pistol".to_string(), 150),
                Item::new(ItemType::Musket(1.), "Musket".to_string(), 200),
                Item::new(ItemType::Boat, "Wooden Boat".to_string(), 500),
            ],
        }
    }
}

#[derive(Resource)]
pub struct ShopCooldown(pub Timer);

#[derive(Component, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub money: u32,
}

impl Inventory {
    pub fn new(initial_money: u32) -> Self {
        let mut inventory = Self {
            items: Vec::new(),
            money: initial_money,
        };

        // Add default items
        inventory.add_item(Item::new(ItemType::Dagger, "Dagger".to_string(), 75));
        inventory.add_item(Item::new(ItemType::Sword, "Sword".to_string(), 100));
        inventory.add_item(Item::new(ItemType::Pistol, "Pistol".to_string(), 150));
        inventory.add_item(Item::new(ItemType::Musket, "Musket".to_string(), 200));

        inventory
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn find_and_upgrade_item(&mut self, item_type: ItemType) -> bool {
        if let Some(item) = self.items.iter_mut().find(|i| i.item_type == item_type) {
            if item.level < 5 {
                item.upgrade();
                return true;
            }
        }
        false
    }
}

#[derive(Component)]
pub struct ShopUI;

#[derive(Component, Debug)]
pub enum ShopButton {
    Buy,
    Upgrade,
    Sell,
    UpgradeItem(usize),
    SellItem(usize),
}

#[derive(Event)]
pub enum ShopEvent {
    Upgrade(usize),
    Sell(usize),
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopPage {
    BuyUpgrade,
    Sell,
}

impl Default for ShopPage {
    fn default() -> Self {
        ShopPage::BuyUpgrade
    }
}
