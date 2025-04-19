use bevy::prelude::*;
use crate::shared::components::Skills;
use crate::client::input::Player;
use crate::client::terrain::ResourceNodeType;

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkillsSettings>()
           .add_event::<SkillExperienceEvent>()
           .add_event::<ResourceGatheringEvent>()
           .add_systems(Update, (
               handle_skill_experience,
               handle_resource_gathering,
               check_resource_interaction,
           ));
    }
}

// Skill settings
#[derive(Resource)]
pub struct SkillsSettings {
    pub experience_curve: Vec<u32>,
    pub gathering_base_experience: f32,
    pub gathering_time_base: f32,
}

impl Default for SkillsSettings {
    fn default() -> Self {
        // Create RuneScape-like experience curve
        let mut experience_curve = Vec::with_capacity(100);
        let mut total_xp = 0;

        for level in 1..100 {
            let points = ((level as f64 - 1.0) + 300.0 * 2.0_f64.powf((level as f64 - 1.0) / 7.0)) / 4.0;
            total_xp += points.floor() as u32;
            experience_curve.push(total_xp);
        }

        Self {
            experience_curve,
            gathering_base_experience: 10.0,
            gathering_time_base: 3.0,
        }
    }
}

// Skill events
#[derive(Event)]
pub struct SkillExperienceEvent {
    pub skill_name: String,
    pub experience: u32,
}

#[derive(Event)]
pub struct ResourceGatheringEvent {
    pub resource_type: ResourceNodeType,
    pub entity: Entity,
}

// Resource gathering state
#[derive(Component)]
pub struct GatheringInProgress {
    pub resource_type: ResourceNodeType,
    #[allow(dead_code)]
    pub target_entity: Entity,
    pub progress: f32,
    pub total_time: f32,
}

// Handle skill experience gain
fn handle_skill_experience(
    mut events: EventReader<SkillExperienceEvent>,
    settings: Res<SkillsSettings>,
    mut query: Query<&mut Skills, With<Player>>,
) {
    if let Ok(mut skills) = query.get_single_mut() {
        for event in events.read() {
            match event.skill_name.as_str() {
                "attack" => skills.attack += event.experience,
                "defense" => skills.defense += event.experience,
                "strength" => skills.strength += event.experience,
                "hitpoints" => skills.hitpoints += event.experience,
                "ranged" => skills.ranged += event.experience,
                "prayer" => skills.prayer += event.experience,
                "magic" => skills.magic += event.experience,
                "cooking" => skills.cooking += event.experience,
                "woodcutting" => skills.woodcutting += event.experience,
                "fletching" => skills.fletching += event.experience,
                "fishing" => skills.fishing += event.experience,
                "firemaking" => skills.firemaking += event.experience,
                "crafting" => skills.crafting += event.experience,
                "smithing" => skills.smithing += event.experience,
                "mining" => skills.mining += event.experience,
                "herblore" => skills.herblore += event.experience,
                "agility" => skills.agility += event.experience,
                "thieving" => skills.thieving += event.experience,
                "slayer" => skills.slayer += event.experience,
                "farming" => skills.farming += event.experience,
                "runecrafting" => skills.runecrafting += event.experience,
                _ => println!("Unknown skill: {}", event.skill_name),
            }

            // Calculate and print new level
            let skill_xp = match event.skill_name.as_str() {
                "attack" => skills.attack,
                "defense" => skills.defense,
                "strength" => skills.strength,
                "hitpoints" => skills.hitpoints,
                "ranged" => skills.ranged,
                "prayer" => skills.prayer,
                "magic" => skills.magic,
                "cooking" => skills.cooking,
                "woodcutting" => skills.woodcutting,
                "fletching" => skills.fletching,
                "fishing" => skills.fishing,
                "firemaking" => skills.firemaking,
                "crafting" => skills.crafting,
                "smithing" => skills.smithing,
                "mining" => skills.mining,
                "herblore" => skills.herblore,
                "agility" => skills.agility,
                "thieving" => skills.thieving,
                "slayer" => skills.slayer,
                "farming" => skills.farming,
                "runecrafting" => skills.runecrafting,
                _ => 0,
            };

            let level = calculate_level(skill_xp, &settings.experience_curve);
            println!("Gained {} experience in {}. New level: {}", event.experience, event.skill_name, level);
        }
    }
}

// Handle resource gathering
fn handle_resource_gathering(
    mut commands: Commands,
    mut events: EventReader<ResourceGatheringEvent>,
    settings: Res<SkillsSettings>,
    mut query: Query<(Entity, &mut Skills), With<Player>>,
    mut gathering_query: Query<(Entity, &mut GatheringInProgress)>,
    time: Res<Time>,
    mut skill_events: EventWriter<SkillExperienceEvent>,
) {
    // First, process ongoing gathering
    for (entity, mut gathering) in gathering_query.iter_mut() {
        gathering.progress += time.delta_seconds();

        // Check if gathering is complete
        if gathering.progress >= gathering.total_time {
            // Determine which skill to give experience to
            let skill_name = match gathering.resource_type {
                ResourceNodeType::Tree => "woodcutting",
                ResourceNodeType::Rock => "mining",
                ResourceNodeType::OreDeposit => "mining",
                ResourceNodeType::FishingSpot => "fishing",
            };

            // Calculate experience based on resource type
            let base_xp = settings.gathering_base_experience;
            let experience = match gathering.resource_type {
                ResourceNodeType::Tree => (base_xp * 1.0) as u32,
                ResourceNodeType::Rock => (base_xp * 1.2) as u32,
                ResourceNodeType::OreDeposit => (base_xp * 1.5) as u32,
                ResourceNodeType::FishingSpot => (base_xp * 1.1) as u32,
            };

            // Send skill experience event
            skill_events.send(SkillExperienceEvent {
                skill_name: skill_name.to_string(),
                experience,
            });

            // Remove gathering component
            commands.entity(entity).remove::<GatheringInProgress>();

            println!("Gathered resource: {:?}", gathering.resource_type);
        }
    }

    // Then, process new gathering events
    if let Ok((player_entity, _)) = query.get_single_mut() {
        for event in events.read() {
            // Check if player is already gathering
            if gathering_query.contains(player_entity) {
                println!("Already gathering a resource!");
                continue;
            }

            // Start gathering
            let gathering_time = settings.gathering_time_base;

            commands.entity(player_entity).insert(GatheringInProgress {
                resource_type: event.resource_type.clone(),
                target_entity: event.entity,
                progress: 0.0,
                total_time: gathering_time,
            });

            println!("Started gathering: {:?}", event.resource_type);
        }
    }
}

// Check for player interaction with resources
fn check_resource_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    resource_query: Query<(Entity, &Transform, &ResourceNodeType)>,
    mut gathering_events: EventWriter<ResourceGatheringEvent>,
) {
    // Only check when the E key is pressed
    if keyboard_input.just_pressed(KeyCode::F) {
        if let Ok(player_transform) = player_query.get_single() {
            // Find the closest resource within interaction range
            let interaction_range = 2.0;
            let mut closest_resource = None;
            let mut closest_distance = f32::MAX;

            for (entity, transform, resource_type) in resource_query.iter() {
                let distance = player_transform.translation.distance(transform.translation);

                if distance < interaction_range && distance < closest_distance {
                    closest_distance = distance;
                    closest_resource = Some((entity, resource_type.clone()));
                }
            }

            // If a resource is in range, send a gathering event
            if let Some((entity, resource_type)) = closest_resource {
                gathering_events.send(ResourceGatheringEvent {
                    resource_type,
                    entity,
                });
            } else {
                println!("No resources in range to gather.");
            }
        }
    }
}

// Calculate level from experience
pub fn calculate_level(experience: u32, experience_curve: &[u32]) -> u32 {
    for (level, &required_xp) in experience_curve.iter().enumerate() {
        if experience < required_xp {
            return (level + 1) as u32;
        }
    }

    // If experience is beyond the highest level in the curve
    experience_curve.len() as u32 + 1
}
