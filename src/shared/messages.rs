use serde::{Serialize, Deserialize};
use super::components::*;
use super::entities::*;

// Network message definitions

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientMessage {
    PlayerMovement {
        direction: MovementDirection,
    },
    ChatMessage {
        content: String,
        channel: ChatChannel,
    },
    InteractWithEntity {
        entity_id: u64,
    },
    UseItem {
        item_id: u64,
        target_item_id: Option<u64>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerMessage {
    PlayerJoined {
        player: Player,
        position: Position,
    },
    PlayerLeft {
        player_id: u64,
    },
    EntityMoved {
        entity_id: u64,
        position: Position,
    },
    ChatReceived {
        sender_id: u64,
        sender_name: String,
        content: String,
        channel: ChatChannel,
    },
    InventoryUpdate {
        inventory: Inventory,
    },
    SkillsUpdate {
        skills: Skills,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MovementDirection {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChatChannel {
    Global,
    Local,
    Private,
    Clan,
    Trade,
}
