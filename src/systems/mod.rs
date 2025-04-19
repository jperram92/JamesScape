pub mod player;
pub mod combat;
pub mod skills;
pub mod economy;
pub mod quests;
pub mod skills_system;
pub mod combat_system;
pub mod inventory_system;

use bevy::prelude::*;
use skills_system::SkillsPlugin;
use combat_system::CombatPlugin;
use inventory_system::InventoryPlugin;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkillsPlugin)
           .add_plugins(CombatPlugin)
           .add_plugins(InventoryPlugin)
           .add_systems(Startup, systems_setup);
    }
}

fn systems_setup() {
    println!("Game systems initialized");
}
