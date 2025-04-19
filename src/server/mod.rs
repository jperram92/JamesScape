pub mod world;
pub mod network;
pub mod database;

use bevy::prelude::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, server_setup);
    }
}

fn server_setup() {
    println!("Server initialized");
}
