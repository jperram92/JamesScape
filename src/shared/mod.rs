pub mod entities;
pub mod components;
pub mod messages;

use bevy::prelude::*;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, shared_setup);
    }
}

fn shared_setup() {
    println!("Shared components initialized");
}
