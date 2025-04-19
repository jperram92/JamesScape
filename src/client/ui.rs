use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::client::input::Player;
use crate::shared::components::{Skills, Health};
use crate::systems::skills_system::{GatheringInProgress, SkillsSettings};
use crate::systems::combat_system::CombatState;
use crate::client::terrain::ResourceNodeType;
use crate::systems::inventory_system::{Inventory, ItemDatabase};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
           .add_systems(Update, ui_system);
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    player_query: Query<(&Transform, Option<&Skills>, Option<&Health>, Option<&GatheringInProgress>, Option<&CombatState>, Option<&Inventory>), With<Player>>,
    settings: Res<SkillsSettings>,
    item_database: Res<ItemDatabase>,
) {
    let ctx = contexts.ctx_mut();

    // Set up the RuneScape-style UI with bottom action bar
    setup_action_bar(ctx, &player_query, &item_database);
    // Game info window
    egui::Window::new("JamesScape")
        .resizable(false)
        .default_width(240.0)
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(30, 30, 30, 240),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
            rounding: egui::Rounding::same(2.0),
            inner_margin: egui::style::Margin::same(6.0),
            outer_margin: egui::style::Margin::same(0.0),
            ..Default::default()
        })
        .show(contexts.ctx_mut(), |ui| {
        ui.heading(egui::RichText::new("Welcome to JamesScape!").size(18.0));

        // Controls section in a collapsible header
        ui.collapsing("Controls Guide", |ui| {
            // Movement controls
            ui.label(egui::RichText::new("🎮 Movement Controls:").strong());
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("• W - Move forward");
                    ui.label("• S - Move backward");
                    ui.label("• A - Move left");
                    ui.label("• D - Move right");
                });
                ui.vertical(|ui| {
                    ui.label("• SPACE - Jump");
                    ui.label("• Q/E - Rotate camera");
                    ui.label("• Mouse Wheel - Zoom");
                    ui.label("• +/- Keys - Zoom");
                });
            });
            ui.separator();

            // Gathering controls
            ui.label(egui::RichText::new("🌲 Gathering Resources:").strong());
            ui.label("• Approach a resource (tree, rock, ore)");
            ui.label("• Press F when close to start gathering");
            ui.label("• Wait for the progress bar to complete");
            ui.separator();

            // Combat controls
            ui.label(egui::RichText::new("⚔️ Combat Controls:").strong());
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("• 1 - Switch to Melee");
                    ui.label("• 2 - Switch to Ranged");
                });
                ui.vertical(|ui| {
                    ui.label("• 3 - Switch to Magic");
                    ui.label("• LMB - Attack enemy");
                });
            });
        });

        // Display player information if available
        if let Ok((transform, _, health, gathering, combat_state, _inventory)) = player_query.get_single() {
            ui.separator();

            // Player position in a more compact format
            ui.collapsing("Player Position", |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("X:").strong());
                    ui.label(format!("{:.2}", transform.translation.x));
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Y:").strong());
                    ui.label(format!("{:.2}", transform.translation.y));
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Z:").strong());
                    ui.label(format!("{:.2}", transform.translation.z));
                });
            });

            // Display health if available
            if let Some(health) = health {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Health:").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let health_percent = (health.current as f32 / health.maximum as f32) * 100.0;
                        ui.label(format!("{}/{} ({:.1}%)", health.current, health.maximum, health_percent));
                    });
                });

                // Health bar with color based on health percentage
                let health_ratio = health.current as f32 / health.maximum as f32;
                let health_color = if health_ratio < 0.3 {
                    egui::Color32::RED
                } else if health_ratio < 0.6 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::GREEN
                };

                ui.add(egui::ProgressBar::new(health_ratio)
                    .fill(health_color)
                    .show_percentage());
            }

            // Display combat state if available
            if let Some(combat_state) = combat_state {
                ui.separator();
                ui.collapsing("Combat Status", |ui| {
                    // Combat style with icon
                    let (style_icon, style_name) = match combat_state.current_style {
                        crate::systems::combat_system::CombatStyle::Melee => ("⚔️", "Melee"),
                        crate::systems::combat_system::CombatStyle::Ranged => ("🏹", "Ranged"),
                        crate::systems::combat_system::CombatStyle::Magic => ("🔮", "Magic"),
                    };

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Style:").strong());
                        ui.label(format!("{} {}", style_icon, style_name));
                    });

                    // Target information
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Target:").strong());
                        if let Some(target) = combat_state.target {
                            ui.label(format!("Entity {:?}", target));
                        } else {
                            ui.label("None");
                        }
                    });

                    // Attack cooldown with progress bar
                    let cooldown_percent = combat_state.attack_timer.percent_left();
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Attack:").strong());
                        if cooldown_percent > 0.0 {
                            ui.label(format!("Ready in {:.1}s", combat_state.attack_timer.remaining_secs()));
                        } else {
                            ui.label(egui::RichText::new("Ready!").color(egui::Color32::GREEN));
                        }
                    });

                    if cooldown_percent > 0.0 {
                        ui.add(egui::ProgressBar::new(1.0 - cooldown_percent)
                            .fill(egui::Color32::from_rgb(100, 150, 255))
                            .show_percentage());
                    }
                });
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
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Progress:").strong());
                    ui.label(format!("{:.1}%", progress_percent));
                });

                // Add a colorful progress bar
                let progress_bar = egui::ProgressBar::new(gathering.progress / gathering.total_time)
                    .fill(egui::Color32::from_rgb(100, 200, 100))
                    .show_percentage()
                    .animate(true);

                ui.add(progress_bar);

                // Add estimated time remaining
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Time remaining:").strong());
                    let time_remaining = gathering.total_time - gathering.progress;
                    ui.label(format!("{:.1} seconds", time_remaining));
                });
            }
        }
    });

    // Skills panel
    egui::Window::new("Skills")
        .resizable(false)
        .default_width(220.0)
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(30, 30, 30, 240),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
            rounding: egui::Rounding::same(2.0),
            inner_margin: egui::style::Margin::same(6.0),
            outer_margin: egui::style::Margin::same(0.0),
            ..Default::default()
        })
        .show(contexts.ctx_mut(), |ui| {
        if let Ok((_, skills_opt, _, _, _, _)) = player_query.get_single() {
            if let Some(skills) = skills_opt {
                // Helper function to calculate and display skill level in a formatted way
                let display_skill = |ui: &mut egui::Ui, name: &str, xp: u32| {
                    let level = crate::systems::skills_system::calculate_level(xp, &settings.experience_curve);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("{}", name)).strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(format!("({} XP)", xp)).weak().small());
                            ui.add_space(5.0);
                            ui.label(egui::RichText::new(format!("{}", level)).strong());
                        });
                    });
                };

                // Combat Skills Section
                ui.collapsing("Combat Skills", |ui| {
                    display_skill(ui, "Attack", skills.attack);
                    display_skill(ui, "Strength", skills.strength);
                    display_skill(ui, "Defense", skills.defense);
                    display_skill(ui, "Hitpoints", skills.hitpoints);
                    display_skill(ui, "Ranged", skills.ranged);
                    display_skill(ui, "Magic", skills.magic);
                    display_skill(ui, "Prayer", skills.prayer);
                });

                ui.add_space(4.0);

                // Gathering Skills Section
                ui.collapsing("Gathering Skills", |ui| {
                    display_skill(ui, "Mining", skills.mining);
                    display_skill(ui, "Fishing", skills.fishing);
                    display_skill(ui, "Woodcutting", skills.woodcutting);
                });

                ui.add_space(4.0);

                // Artisan Skills Section
                ui.collapsing("Artisan Skills", |ui| {
                    display_skill(ui, "Cooking", skills.cooking);
                    display_skill(ui, "Smithing", skills.smithing);
                    display_skill(ui, "Crafting", skills.crafting);
                    display_skill(ui, "Fletching", skills.fletching);
                    display_skill(ui, "Herblore", skills.herblore);
                    display_skill(ui, "Runecrafting", skills.runecrafting);
                    display_skill(ui, "Firemaking", skills.firemaking);
                });

                ui.add_space(4.0);

                // Support Skills Section
                ui.collapsing("Support Skills", |ui| {
                    display_skill(ui, "Agility", skills.agility);
                    display_skill(ui, "Thieving", skills.thieving);
                    display_skill(ui, "Slayer", skills.slayer);
                    display_skill(ui, "Farming", skills.farming);
                });
            } else {
                ui.label("Skills data not available");
            }
        } else {
            ui.label("Player not found");
        }
    });

    // Inventory panel - now a floating window that can be toggled
    show_inventory_window(contexts.ctx_mut(), &player_query, &item_database);
}

// Function to set up the RuneScape-style action bar at the bottom of the screen
fn setup_action_bar(
    ctx: &mut egui::Context,
    player_query: &Query<(&Transform, Option<&Skills>, Option<&Health>, Option<&GatheringInProgress>, Option<&CombatState>, Option<&Inventory>), With<Player>>,
    _item_database: &Res<ItemDatabase>,
) {
    // Create a panel at the bottom of the screen
    egui::TopBottomPanel::bottom("action_bar")
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(40, 40, 40, 230),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
            rounding: egui::Rounding::same(4.0),
            inner_margin: egui::style::Margin::same(8.0),
            outer_margin: egui::style::Margin::same(0.0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // Combat style buttons
                let button_size = egui::Vec2::new(40.0, 40.0);

                // Melee button
                let melee_button = ui.add(egui::Button::new("⚔️")
                    .min_size(button_size)
                    .frame(true));
                if melee_button.clicked() {
                    // Handle melee style selection
                }
                if melee_button.hovered() {
                    melee_button.on_hover_text("Switch to Melee combat (1)");
                }

                // Ranged button
                let ranged_button = ui.add(egui::Button::new("🏹")
                    .min_size(button_size)
                    .frame(true));
                if ranged_button.clicked() {
                    // Handle ranged style selection
                }
                if ranged_button.hovered() {
                    ranged_button.on_hover_text("Switch to Ranged combat (2)");
                }

                // Magic button
                let magic_button = ui.add(egui::Button::new("🔮")
                    .min_size(button_size)
                    .frame(true));
                if magic_button.clicked() {
                    // Handle magic style selection
                }
                if magic_button.hovered() {
                    magic_button.on_hover_text("Switch to Magic combat (3)");
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Inventory button
                let inventory_button = ui.add(egui::Button::new("🎒")
                    .min_size(button_size)
                    .frame(true));
                if inventory_button.clicked() {
                    // Toggle inventory visibility
                }
                if inventory_button.hovered() {
                    inventory_button.on_hover_text("Open Inventory");
                }

                // Skills button
                let skills_button = ui.add(egui::Button::new("📊")
                    .min_size(button_size)
                    .frame(true));
                if skills_button.clicked() {
                    // Toggle skills visibility
                }
                if skills_button.hovered() {
                    skills_button.on_hover_text("Open Skills");
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Health and status indicators
                if let Ok((_, _, health, gathering, combat_state, _)) = player_query.get_single() {
                    // Health indicator
                    if let Some(health) = health {
                        let health_ratio = health.current as f32 / health.maximum as f32;
                        let health_color = if health_ratio < 0.3 {
                            egui::Color32::RED
                        } else if health_ratio < 0.6 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::GREEN
                        };

                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("❤️").size(20.0));
                            ui.add(egui::ProgressBar::new(health_ratio)
                                .desired_width(80.0)
                                .fill(health_color));
                        });
                    }

                    // Combat style indicator
                    if let Some(combat) = combat_state {
                        ui.vertical(|ui| {
                            let style_text = match combat.current_style {
                                crate::systems::combat_system::CombatStyle::Melee => "⚔️ Melee",
                                crate::systems::combat_system::CombatStyle::Ranged => "🏹 Ranged",
                                crate::systems::combat_system::CombatStyle::Magic => "🔮 Magic",
                            };
                            ui.label(egui::RichText::new(style_text).strong());
                        });
                    }

                    // Gathering indicator
                    if let Some(gathering) = gathering {
                        let progress = gathering.progress / gathering.total_time;
                        ui.vertical(|ui| {
                            let action = match gathering.resource_type {
                                crate::client::terrain::ResourceNodeType::Tree => "Woodcutting",
                                crate::client::terrain::ResourceNodeType::Rock => "Mining",
                                crate::client::terrain::ResourceNodeType::OreDeposit => "Mining",
                                crate::client::terrain::ResourceNodeType::FishingSpot => "Fishing",
                            };
                            ui.label(egui::RichText::new(action).strong());
                            ui.add(egui::ProgressBar::new(progress)
                                .desired_width(80.0)
                                .animate(true));
                        });
                    }
                }

                // Gold display on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Ok((_, _, _, _, _, inventory_opt)) = player_query.get_single() {
                        if let Some(inventory) = inventory_opt {
                            ui.label(egui::RichText::new(format!("{}", inventory.gold))
                                .strong()
                                .color(egui::Color32::from_rgb(255, 215, 0)));
                            ui.label(egui::RichText::new("💰").size(20.0));
                        }
                    }
                });
            });
        });
}

// Function to show the RuneScape-style inventory window
fn show_inventory_window(
    ctx: &mut egui::Context,
    player_query: &Query<(&Transform, Option<&Skills>, Option<&Health>, Option<&GatheringInProgress>, Option<&CombatState>, Option<&Inventory>), With<Player>>,
    item_database: &Res<ItemDatabase>,
) {
    egui::Window::new("Inventory")
        .resizable(false)
        .collapsible(true)
        .default_width(176.0) // Exact width for 4 slots
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(30, 30, 30, 240),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
            rounding: egui::Rounding::same(2.0),
            inner_margin: egui::style::Margin::same(4.0),
            outer_margin: egui::style::Margin::same(0.0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            if let Ok((_, _, _, _, _, inventory_opt)) = player_query.get_single() {
                if let Some(inventory) = inventory_opt {
                    // Header with gold display
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Backpack").heading());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(format!("{}", inventory.gold))
                                .strong()
                                .color(egui::Color32::from_rgb(255, 215, 0)));
                            ui.label(egui::RichText::new("💰").size(16.0));
                        });
                    });

                    ui.separator();

                    // RuneScape-style inventory grid (4x7 grid)
                    let slot_size = 40.0;
                    let slot_padding = 2.0;
                    let slots_per_row = 4;
                    let rows = 7;

                    // Sort items by ID for consistent display
                    let mut items: Vec<(&u64, &u32)> = inventory.items.iter().collect();
                    items.sort_by_key(|&(id, _)| *id);

                    // Create the grid with fixed size
                    ui.spacing_mut().item_spacing = egui::vec2(slot_padding, slot_padding);

                    // Use a table for more precise control
                    egui::Grid::new("inventory_grid")
                        .spacing([slot_padding, slot_padding])
                        .min_col_width(slot_size)
                        .max_col_width(slot_size)
                        .show(ui, |ui| {
                            for row in 0..rows {
                                for col in 0..slots_per_row {
                                    let slot_index = row * slots_per_row + col;
                                    let total_slots = inventory.capacity as usize;

                                    if slot_index < total_slots {
                                        // Create a framed slot with RuneScape style
                                        let frame = egui::Frame::none()
                                            .fill(egui::Color32::from_gray(50))
                                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(80)))
                                            .rounding(egui::Rounding::same(0.0))
                                            .inner_margin(egui::style::Margin::same(1.0));

                                        frame.show(ui, |ui| {
                                            // Fixed size for each slot
                                            ui.set_min_size(egui::Vec2::new(slot_size, slot_size));
                                            ui.set_max_size(egui::Vec2::new(slot_size, slot_size));

                                            // Find if there's an item for this slot
                                            let item_for_slot = if slot_index < items.len() {
                                                Some(items[slot_index])
                                            } else {
                                                None
                                            };

                                            if let Some((item_id, quantity)) = item_for_slot {
                                                if let Some(item_def) = item_database.items.get(item_id) {
                                                    // Item icon (emoji based on type)
                                                    let icon = match item_def.item_type {
                                                        crate::systems::inventory_system::ItemType::Resource => {
                                                            match item_def.name.as_str() {
                                                                "Logs" => "🪵",
                                                                "Stone" => "🪨",
                                                                "Ore" => "💎",
                                                                "Fish" => "🐟",
                                                                _ => "📦",
                                                            }
                                                        },
                                                        _ => "📦",
                                                    };

                                                    // Center the icon
                                                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                                        ui.label(egui::RichText::new(icon).size(20.0));
                                                    });

                                                    // Add quantity in bottom-right corner
                                                    if *quantity > 1 {
                                                        let quantity_text = if *quantity >= 100_000 {
                                                            format!("{:.1}M", *quantity as f32 / 1_000_000.0)
                                                        } else if *quantity >= 10_000 {
                                                            format!("{:.1}K", *quantity as f32 / 1_000.0)
                                                        } else {
                                                            format!("{}", quantity)
                                                        };

                                                        ui.put(
                                                            egui::Rect::from_min_size(
                                                                ui.min_rect().min + egui::vec2(slot_size - 20.0, slot_size - 16.0),
                                                                egui::Vec2::new(18.0, 14.0),
                                                            ),
                                                            egui::Label::new(
                                                                egui::RichText::new(quantity_text)
                                                                    .color(egui::Color32::from_rgb(255, 255, 100))
                                                                    .small()
                                                            ),
                                                        );
                                                    }

                                                    // Tooltip on hover
                                                    let tooltip_text = format!("{}: {}\nValue: {} gold",
                                                        item_def.name, item_def.description, item_def.value);
                                                    ui.with_layout(egui::Layout::default(), |ui| {
                                                        ui.add(egui::Label::new("").sense(egui::Sense::hover()))
                                                            .on_hover_text(tooltip_text);
                                                    });
                                                }
                                            }
                                        });
                                    }
                                }
                                ui.end_row();
                            }
                        });

                    // Display capacity at the bottom
                    ui.separator();
                    ui.horizontal(|ui| {
                        let used = inventory.items.len();
                        let capacity = inventory.capacity as usize;
                        let color = if used >= capacity {
                            egui::Color32::RED
                        } else if used > capacity * 3 / 4 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::GREEN
                        };

                        ui.label("Items:");
                        ui.label(egui::RichText::new(format!("{} / {}", used, capacity)).color(color));
                    });
                } else {
                    ui.label("Inventory data not available");
                }
            } else {
                ui.label("Player not found");
            }
        });
}
