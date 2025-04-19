use bevy::prelude::*;
use crate::shared::components::*;
use crate::shared::entities::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player);
    }
}

fn update_player() {
    // Player update logic will go here
}

// Player creation function
#[allow(dead_code)]
pub fn create_player(username: String) -> Player {
    Player {
        id: rand::random::<u64>(),
        username,
    }
}

// Player movement function
#[allow(dead_code)]
pub fn move_player(_player: &mut Player, _position: &mut Position, _direction: &crate::shared::messages::MovementDirection) {
    // Player movement logic will go here
}
