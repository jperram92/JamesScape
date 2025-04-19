use bevy::prelude::*;
use serde::{Serialize, Deserialize};

// Components that are shared between client and server

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: u32,
    pub maximum: u32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub attack: u32,
    pub defense: u32,
    pub strength: u32,
    pub hitpoints: u32,
    pub ranged: u32,
    pub prayer: u32,
    pub magic: u32,
    pub cooking: u32,
    pub woodcutting: u32,
    pub fletching: u32,
    pub fishing: u32,
    pub firemaking: u32,
    pub crafting: u32,
    pub smithing: u32,
    pub mining: u32,
    pub herblore: u32,
    pub agility: u32,
    pub thieving: u32,
    pub slayer: u32,
    pub farming: u32,
    pub runecrafting: u32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<(u64, u32)>, // (item_id, quantity)
    pub capacity: u32,
}
