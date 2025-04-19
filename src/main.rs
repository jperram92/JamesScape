mod client;
mod server;
mod shared;
mod systems;

use bevy::prelude::*;
use client::ClientPlugin;
use server::ServerPlugin;
use shared::SharedPlugin;
use systems::GameSystemsPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
#[allow(dead_code)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JamesScape".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugins(ClientPlugin)
        .add_plugins(ServerPlugin)
        .add_plugins(SharedPlugin)
        .add_plugins(GameSystemsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    println!("JamesScape is starting up!");
}
