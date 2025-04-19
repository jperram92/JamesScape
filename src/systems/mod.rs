pub mod player;
pub mod combat;
pub mod skills;
pub mod economy;
pub mod quests;
pub mod skills_system;
pub mod combat_system;

use bevy::prelude::*;
use skills_system::SkillsPlugin;
use combat_system::CombatPlugin;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkillsPlugin)
           .add_plugins(CombatPlugin)
           .add_systems(Startup, systems_setup);
    }
}

fn systems_setup() {
    println!("Game systems initialized");
}
