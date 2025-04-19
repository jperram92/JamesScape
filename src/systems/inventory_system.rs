use bevy::prelude::*;
use crate::client::input::Player;
use crate::client::terrain::ResourceNodeType;
use std::collections::HashMap;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemDatabase>()
           .add_event::<InventoryUpdateEvent>()
           .add_systems(Update, (
               handle_inventory_updates,
               update_inventory_ui,
           ));
    }
}

// Component for player inventory
#[derive(Component, Default)]
pub struct Inventory {
    pub items: HashMap<u64, u32>, // item_id -> quantity
    pub capacity: u32,
    pub gold: u32,
}

// Event for inventory updates
#[derive(Event)]
pub struct InventoryUpdateEvent {
    pub item_id: u64,
    pub quantity: i32, // Positive for add, negative for remove
}

// Resource for item database
#[derive(Resource)]
pub struct ItemDatabase {
    pub items: HashMap<u64, ItemDefinition>,
}

impl Default for ItemDatabase {
    fn default() -> Self {
        let mut items = HashMap::new();

        // Add basic resource items
        items.insert(1, ItemDefinition {
            id: 1,
            name: "Logs".to_string(),
            description: "Wood from a tree.".to_string(),
            stackable: true,
            value: 10,
            item_type: ItemType::Resource,
        });

        items.insert(2, ItemDefinition {
            id: 2,
            name: "Stone".to_string(),
            description: "A piece of rock.".to_string(),
            stackable: true,
            value: 5,
            item_type: ItemType::Resource,
        });

        items.insert(3, ItemDefinition {
            id: 3,
            name: "Ore".to_string(),
            description: "Metal ore that can be smelted.".to_string(),
            stackable: true,
            value: 20,
            item_type: ItemType::Resource,
        });

        items.insert(4, ItemDefinition {
            id: 4,
            name: "Fish".to_string(),
            description: "A fresh fish.".to_string(),
            stackable: true,
            value: 15,
            item_type: ItemType::Food,
        });

        Self { items }
    }
}

// Item definition
#[derive(Clone, Debug)]
pub struct ItemDefinition {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub stackable: bool,
    pub value: u32,
    pub item_type: ItemType,
}

// Item types
#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    Resource,
    Equipment,
    Consumable,
    Food,
    Quest,
}

// System to handle inventory updates
fn handle_inventory_updates(
    mut events: EventReader<InventoryUpdateEvent>,
    mut query: Query<&mut Inventory, With<Player>>,
    item_database: Res<ItemDatabase>,
) {
    if let Ok(mut inventory) = query.get_single_mut() {
        for event in events.read() {
            // Check if item exists in database
            if !item_database.items.contains_key(&event.item_id) {
                println!("Warning: Tried to add unknown item ID: {}", event.item_id);
                continue;
            }

            // Update inventory
            if event.quantity > 0 {
                // Add items
                let current = inventory.items.entry(event.item_id).or_insert(0);
                *current += event.quantity as u32;
                println!("Added {} x {} to inventory", event.quantity,
                    item_database.items.get(&event.item_id).unwrap().name);
            } else if event.quantity < 0 {
                // Remove items
                let quantity_to_remove = (-event.quantity) as u32;
                if let Some(current) = inventory.items.get_mut(&event.item_id) {
                    if *current >= quantity_to_remove {
                        *current -= quantity_to_remove;
                        println!("Removed {} x {} from inventory", quantity_to_remove,
                            item_database.items.get(&event.item_id).unwrap().name);

                        // Remove entry if quantity is 0
                        if *current == 0 {
                            inventory.items.remove(&event.item_id);
                        }
                    } else {
                        println!("Not enough items to remove!");
                    }
                } else {
                    println!("Item not in inventory!");
                }
            }
        }
    }
}

// System to update inventory UI
fn update_inventory_ui(
    inventory_query: Query<&Inventory, With<Player>>,
    item_database: Res<ItemDatabase>,
) {
    if let Ok(inventory) = inventory_query.get_single() {
        // This will be used by the UI system to display the inventory
        // We'll implement the actual UI in the UI module

        // For debugging, print inventory contents
        if !inventory.items.is_empty() && rand::random::<f32>() < 0.01 {  // Only print occasionally
            println!("Current Inventory:");
            for (item_id, quantity) in &inventory.items {
                if let Some(item_def) = item_database.items.get(item_id) {
                    println!("  {} x {}", quantity, item_def.name);
                }
            }
            println!("Gold: {}", inventory.gold);
        }
    }
}

// Helper function to get item ID from resource type
pub fn get_item_id_for_resource(resource_type: &ResourceNodeType) -> u64 {
    match resource_type {
        ResourceNodeType::Tree => 1,       // Logs
        ResourceNodeType::Rock => 2,       // Stone
        ResourceNodeType::OreDeposit => 3, // Ore
        ResourceNodeType::FishingSpot => 4, // Fish
    }
}
