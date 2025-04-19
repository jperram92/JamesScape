pub mod rendering;
pub mod input;
pub mod ui;
pub mod network;
pub mod physics;
pub mod terrain;
pub mod indicators;
pub mod character;
pub mod camera;
pub mod effects;

use bevy::prelude::*;
use rendering::RenderingPlugin;
use input::InputPlugin;
use ui::UiPlugin;
use network::NetworkClientPlugin;
use physics::PhysicsPlugin;
use terrain::TerrainPlugin;
use indicators::IndicatorsPlugin;
use character::CharacterPlugin;
use camera::CameraPlugin;
use effects::EffectsPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderingPlugin)
           .add_plugins(PhysicsPlugin)
           .add_plugins(TerrainPlugin)
           .add_plugins(InputPlugin)
           .add_plugins(UiPlugin)
           .add_plugins(NetworkClientPlugin)
           .add_plugins(IndicatorsPlugin)
           .add_plugins(CharacterPlugin)
           .add_plugins(CameraPlugin)
           .add_plugins(EffectsPlugin)
           .add_systems(Startup, client_setup);
    }
}

fn client_setup() {
    println!("Client initialized");
}
