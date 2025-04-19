use bevy::prelude::*;
use rand::Rng;
use crate::shared::components::{Health, Skills};
use crate::client::input::Player;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatSettings>()
           .add_event::<CombatEvent>()
           .add_event::<DamageEvent>()
           .add_systems(Update, (
               handle_combat_input,
               process_combat_events,
               process_damage_events,
               update_enemy_ai,
           ));
    }
}

// Combat settings
#[derive(Resource)]
pub struct CombatSettings {
    pub melee_range: f32,
    pub ranged_range: f32,
    pub magic_range: f32,
    #[allow(dead_code)]
    pub attack_cooldown: f32,
}

impl Default for CombatSettings {
    fn default() -> Self {
        Self {
            melee_range: 2.0,
            ranged_range: 7.0,
            magic_range: 10.0,
            attack_cooldown: 1.5,
        }
    }
}

// Combat style
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum CombatStyle {
    Melee,
    Ranged,
    Magic,
}

// Combat state
#[derive(Component)]
pub struct CombatState {
    pub current_style: CombatStyle,
    pub attack_timer: Timer,
    pub target: Option<Entity>,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            current_style: CombatStyle::Melee,
            attack_timer: Timer::from_seconds(1.5, TimerMode::Once),
            target: None,
        }
    }
}

// Enemy component
#[derive(Component)]
pub struct Enemy {
    pub level: u32,
    pub aggression_range: f32,
    pub attack_range: f32,
}

// Combat events
#[derive(Event)]
pub struct CombatEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub style: CombatStyle,
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: u32,
    #[allow(dead_code)]
    pub is_player_source: bool,
}

// Handle player combat input
fn handle_combat_input(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    settings: Res<CombatSettings>,
    time: Res<Time>,
    mut player_query: Query<(Entity, &Transform, &mut CombatState), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    if let Ok((_, player_transform, mut combat_state)) = player_query.get_single_mut() {
        // Change combat style with number keys
        if keyboard_input.just_pressed(KeyCode::Key1) {
            combat_state.current_style = CombatStyle::Melee;
            println!("Switched to Melee combat style");
        } else if keyboard_input.just_pressed(KeyCode::Key2) {
            combat_state.current_style = CombatStyle::Ranged;
            println!("Switched to Ranged combat style");
        } else if keyboard_input.just_pressed(KeyCode::Key3) {
            combat_state.current_style = CombatStyle::Magic;
            println!("Switched to Magic combat style");
        }

        // Update attack timer
        combat_state.attack_timer.tick(time.delta());

        // Attack with left mouse button
        if mouse_button_input.just_pressed(MouseButton::Left) && combat_state.attack_timer.finished() {
            // Find the closest enemy within range
            let attack_range = match combat_state.current_style {
                CombatStyle::Melee => settings.melee_range,
                CombatStyle::Ranged => settings.ranged_range,
                CombatStyle::Magic => settings.magic_range,
            };

            let mut closest_enemy = None;
            let mut closest_distance = f32::MAX;

            for (enemy_entity, enemy_transform) in enemy_query.iter() {
                let distance = player_transform.translation.distance(enemy_transform.translation);

                if distance < attack_range && distance < closest_distance {
                    closest_distance = distance;
                    closest_enemy = Some(enemy_entity);
                }
            }

            // If an enemy is in range, attack it
            if let Some(enemy_entity) = closest_enemy {
                combat_state.target = Some(enemy_entity);
                combat_state.attack_timer.reset();

                // Send combat event
                // events.send(CombatEvent {
                //     attacker: player_entity,
                //     target: enemy_entity,
                //     style: combat_state.current_style,
                // });

                println!("Attacked enemy with {:?} style", combat_state.current_style);
            } else {
                println!("No enemies in range");
            }
        }
    }
}

// Process combat events
fn process_combat_events(
    mut events: EventReader<CombatEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    player_query: Query<&Skills, With<Player>>,
    enemy_query: Query<&Enemy>,
) {
    let mut rng = rand::thread_rng();

    for event in events.read() {
        // Determine if attacker is player
        let is_player_attacker = player_query.contains(event.attacker);

        if is_player_attacker {
            // Player attacking enemy
            if let Ok(skills) = player_query.get(event.attacker) {
                // Calculate damage based on combat style and skills
                let base_damage = match event.style {
                    CombatStyle::Melee => {
                        let attack_level = calculate_level(skills.attack);
                        let strength_level = calculate_level(skills.strength);
                        (attack_level + strength_level) / 8 + 1
                    },
                    CombatStyle::Ranged => {
                        let ranged_level = calculate_level(skills.ranged);
                        ranged_level / 4 + 1
                    },
                    CombatStyle::Magic => {
                        let magic_level = calculate_level(skills.magic);
                        magic_level / 4 + 1
                    },
                };

                // Add some randomness
                let damage = (base_damage as f32 * rng.gen_range(0.8..1.2)) as u32;

                // Send damage event
                damage_events.send(DamageEvent {
                    target: event.target,
                    amount: damage,
                    is_player_source: true,
                });
            }
        } else {
            // Enemy attacking player
            if let Ok(enemy) = enemy_query.get(event.attacker) {
                // Calculate damage based on enemy level
                let base_damage = enemy.level / 4 + 1;

                // Add some randomness
                let damage = (base_damage as f32 * rng.gen_range(0.8..1.2)) as u32;

                // Send damage event
                damage_events.send(DamageEvent {
                    target: event.target,
                    amount: damage,
                    is_player_source: false,
                });
            }
        }
    }
}

// Process damage events
fn process_damage_events(
    mut events: EventReader<DamageEvent>,
    mut player_query: Query<&mut Health, With<Player>>,
    mut enemy_query: Query<&mut Health, Without<Player>>,
) {
    for event in events.read() {
        if player_query.contains(event.target) {
            // Player taking damage
            if let Ok(mut health) = player_query.get_mut(event.target) {
                health.current = health.current.saturating_sub(event.amount);
                println!("Player took {} damage. Health: {}/{}", event.amount, health.current, health.maximum);
            }
        } else {
            // Enemy taking damage
            if let Ok(mut health) = enemy_query.get_mut(event.target) {
                health.current = health.current.saturating_sub(event.amount);
                println!("Enemy took {} damage. Health: {}/{}", event.amount, health.current, health.maximum);
            }
        }
    }
}

// Update enemy AI
fn update_enemy_ai(
    time: Res<Time>,
    mut enemy_query: Query<(Entity, &Transform, &Enemy, &mut CombatState)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut combat_events: EventWriter<CombatEvent>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for (enemy_entity, enemy_transform, enemy, mut combat_state) in enemy_query.iter_mut() {
            // Update attack timer
            combat_state.attack_timer.tick(time.delta());

            // Calculate distance to player
            let distance = enemy_transform.translation.distance(player_transform.translation);

            // Check if player is in aggression range
            if distance < enemy.aggression_range {
                // Set player as target
                combat_state.target = Some(player_entity);

                // Attack if in range and cooldown is finished
                if distance < enemy.attack_range && combat_state.attack_timer.finished() {
                    combat_state.attack_timer.reset();

                    // Send combat event
                    combat_events.send(CombatEvent {
                        attacker: enemy_entity,
                        target: player_entity,
                        style: CombatStyle::Melee, // Enemies use melee by default
                    });
                }
            } else {
                // Clear target if player is out of range
                combat_state.target = None;
            }
        }
    }
}

// Helper function to calculate level from experience
fn calculate_level(experience: u32) -> u32 {
    // Simple level calculation for now
    (experience as f32 / 100.0).sqrt() as u32 + 1
}
