use bevy::prelude::*;
use serde::{Serialize, Deserialize};

// Entity definitions that are shared between client and server

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u64,
    pub username: String,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NPC {
    pub id: u64,
    pub name: String,
    pub level: u32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub stackable: bool,
    pub value: u32,
}
