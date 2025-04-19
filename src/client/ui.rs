use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::client::input::Player;
use crate::shared::components::{Skills, Health};
use crate::systems::skills_system::{GatheringInProgress, SkillsSettings};
use crate::systems::combat_system::CombatState;
use crate::client::terrain::ResourceNodeType;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
           .add_systems(Update, ui_system);
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    player_query: Query<(&Transform, Option<&Skills>, Option<&Health>, Option<&GatheringInProgress>, Option<&CombatState>), With<Player>>,
    settings: Res<SkillsSettings>,
) {
    // Game info window
    egui::Window::new("JamesScape").show(contexts.ctx_mut(), |ui| {
        ui.heading("Welcome to JamesScape!");

        ui.collapsing("Controls Guide", |ui| {
            ui.label("🎮 Movement Controls:");
            ui.label("• W - Move forward");
            ui.label("• S - Move backward");
            ui.label("• A - Move left");
            ui.label("• D - Move right");
            ui.label("• SPACE - Jump");
            ui.label("• Q/E - Rotate camera");
            ui.separator();

            ui.label("🌲 Gathering Resources:");
            ui.label("• Approach a resource (tree, rock, ore)");
            ui.label("• Press F when close to start gathering");
            ui.label("• Wait for the progress bar to complete");
            ui.separator();

            ui.label("⚔️ Combat Controls:");
            ui.label("• 1 - Switch to Melee combat");
            ui.label("• 2 - Switch to Ranged combat");
            ui.label("• 3 - Switch to Magic combat");
            ui.label("• Left Mouse Button - Attack nearby enemy");
        });

        // Display player position if available
        if let Ok((transform, _, health, gathering, combat_state)) = player_query.get_single() {
            ui.separator();
            ui.label("Player Position:");
            ui.label(format!("X: {:.2}", transform.translation.x));
            ui.label(format!("Y: {:.2}", transform.translation.y));
            ui.label(format!("Z: {:.2}", transform.translation.z));

            // Display health if available
            if let Some(health) = health {
                ui.separator();
                ui.label("Health:");
                let health_percent = (health.current as f32 / health.maximum as f32) * 100.0;
                ui.label(format!("{}/{} ({:.1}%)", health.current, health.maximum, health_percent));
                let health_ratio = health.current as f32 / health.maximum as f32;
                ui.add(egui::ProgressBar::new(health_ratio).show_percentage());
            }

            // Display combat state if available
            if let Some(combat_state) = combat_state {
                ui.separator();
                ui.label("Combat:");
                ui.label(format!("Style: {:?}", combat_state.current_style));
                if let Some(target) = combat_state.target {
                    ui.label(format!("Target: Entity {:?}", target));
                } else {
                    ui.label("No target");
                }

                // Display attack cooldown
                let cooldown_percent = combat_state.attack_timer.percent_left();
                if cooldown_percent > 0.0 {
                    ui.label(format!("Attack ready in: {:.1}s", combat_state.attack_timer.remaining_secs()));
                    ui.add(egui::ProgressBar::new(1.0 - cooldown_percent).show_percentage());
                } else {
                    ui.label("Ready to attack!");
                }
            }

            // Display gathering progress if applicable
            if let Some(gathering) = gathering {
                ui.separator();

                // Show gathering activity with emoji and formatted text
                let (emoji, action, skill) = match gathering.resource_type {
                    ResourceNodeType::Tree => ("🌲", "Chopping", "Woodcutting"),
                    ResourceNodeType::Rock => ("🔩", "Mining", "Mining"),
                    ResourceNodeType::OreDeposit => ("💎", "Mining", "Mining"),
                    ResourceNodeType::FishingSpot => ("🎣", "Fishing", "Fishing"),
                };

                ui.heading(format!("{} {} ({})", emoji, action, skill));

                // Show progress with percentage
                let progress_percent = (gathering.progress / gathering.total_time) * 100.0;
                ui.label(format!("Progress: {:.1}%", progress_percent));

                // Add a colorful progress bar
                let progress_bar = egui::ProgressBar::new(gathering.progress / gathering.total_time)
                    .show_percentage()
                    .animate(true);

                ui.add(progress_bar);

                // Add estimated time remaining
                let time_remaining = gathering.total_time - gathering.progress;
                ui.label(format!("Time remaining: {:.1} seconds", time_remaining));
            }
        }
    });

    // Skills panel
    egui::Window::new("Skills").show(contexts.ctx_mut(), |ui| {
        if let Ok((_, skills_opt, _, _, _)) = player_query.get_single() {
            if let Some(skills) = skills_opt {
                // Helper function to calculate and display skill level
                let display_skill = |ui: &mut egui::Ui, name: &str, xp: u32| {
                    let level = crate::systems::skills_system::calculate_level(xp, &settings.experience_curve);
                    ui.label(format!("{}: {} ({} XP)", name, level, xp));
                };

                ui.label("Combat Skills:");
                display_skill(ui, "Attack", skills.attack);
                display_skill(ui, "Strength", skills.strength);
                display_skill(ui, "Defense", skills.defense);
                display_skill(ui, "Hitpoints", skills.hitpoints);
                display_skill(ui, "Ranged", skills.ranged);
                display_skill(ui, "Magic", skills.magic);
                display_skill(ui, "Prayer", skills.prayer);

                ui.separator();

                ui.label("Gathering Skills:");
                display_skill(ui, "Mining", skills.mining);
                display_skill(ui, "Fishing", skills.fishing);
                display_skill(ui, "Woodcutting", skills.woodcutting);

                ui.separator();

                ui.label("Artisan Skills:");
                display_skill(ui, "Cooking", skills.cooking);
                display_skill(ui, "Smithing", skills.smithing);
                display_skill(ui, "Crafting", skills.crafting);
                display_skill(ui, "Fletching", skills.fletching);
                display_skill(ui, "Herblore", skills.herblore);
                display_skill(ui, "Runecrafting", skills.runecrafting);
                display_skill(ui, "Firemaking", skills.firemaking);

                ui.separator();

                ui.label("Support Skills:");
                display_skill(ui, "Agility", skills.agility);
                display_skill(ui, "Thieving", skills.thieving);
                display_skill(ui, "Slayer", skills.slayer);
                display_skill(ui, "Farming", skills.farming);
            } else {
                ui.label("Skills data not available");
            }
        } else {
            ui.label("Player not found");
        }
    });
}
