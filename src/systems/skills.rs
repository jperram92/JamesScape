use bevy::prelude::*;
use crate::shared::components::Skills;

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_skills);
    }
}

fn update_skills() {
    // Skills update logic will go here
}

// Skill functions
#[allow(dead_code)]
pub fn gain_experience(_skills: &mut Skills, _skill_name: &str, _experience: u32) {
    // Experience gain logic will go here
}

#[allow(dead_code)]
pub fn calculate_level(_experience: u32) -> u32 {
    // Level calculation logic will go here
    // This should follow RuneScape's exponential formula
    1
}

// Initialize default skills
#[allow(dead_code)]
pub fn new_skills() -> Skills {
    Skills {
        attack: 1,
        defense: 1,
        strength: 1,
        hitpoints: 10,
        ranged: 1,
        prayer: 1,
        magic: 1,
        cooking: 1,
        woodcutting: 1,
        fletching: 1,
        fishing: 1,
        firemaking: 1,
        crafting: 1,
        smithing: 1,
        mining: 1,
        herblore: 1,
        agility: 1,
        thieving: 1,
        slayer: 1,
        farming: 1,
        runecrafting: 1,
    }
}
