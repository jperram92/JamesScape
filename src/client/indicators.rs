use bevy::prelude::*;
use crate::client::input::Player;
use crate::client::terrain::ResourceNodeType;

pub struct IndicatorsPlugin;

impl Plugin for IndicatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_resource_indicators,
            update_interaction_prompts,
        ));
    }
}

// Component to mark entities as indicators
#[derive(Component)]
pub struct ResourceIndicator {
    pub target: Entity,
}

// Component for floating text prompts
#[derive(Component)]
pub struct InteractionPrompt {
    pub target: Entity,
    pub offset: Vec3,
    pub timer: Timer,
}

// System to create and update resource indicators
fn update_resource_indicators(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    resource_query: Query<(Entity, &Transform, &ResourceNodeType), Without<Player>>,
    indicator_query: Query<(Entity, &ResourceIndicator)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Only proceed if we have a player
    if let Ok(player_transform) = player_query.get_single() {
        // Define interaction range
        let interaction_range = 3.0;

        // Check each resource
        for (resource_entity, resource_transform, resource_type) in resource_query.iter() {
            let distance = player_transform.translation.distance(resource_transform.translation);

            // Check if this resource already has an indicator
            let has_indicator = indicator_query.iter().any(|(_, indicator)| indicator.target == resource_entity);

            // If within range and no indicator exists, create one
            if distance < interaction_range && !has_indicator {
                // Create a glowing ring indicator
                let indicator_color = match resource_type {
                    ResourceNodeType::Tree => Color::rgb(0.2, 0.8, 0.2),        // Green for trees
                    ResourceNodeType::Rock => Color::rgb(0.7, 0.7, 0.7),        // Gray for rocks
                    ResourceNodeType::OreDeposit => Color::rgb(0.8, 0.6, 0.2),  // Gold for ore
                    ResourceNodeType::FishingSpot => Color::rgb(0.2, 0.6, 0.9), // Blue for fishing
                };

                // Create a ring mesh
                let ring_mesh = meshes.add(shape::Torus {
                    radius: 0.6,
                    ring_radius: 0.05,
                    subdivisions_segments: 12,
                    subdivisions_sides: 6,
                }.into());

                // Create a glowing material
                let mut material = StandardMaterial {
                    base_color: indicator_color,
                    emissive: indicator_color * 2.0,
                    ..default()
                };
                material.alpha_mode = AlphaMode::Blend;
                let material_handle = materials.add(material);

                // Spawn the indicator
                commands.spawn((
                    PbrBundle {
                        mesh: ring_mesh,
                        material: material_handle,
                        transform: Transform::from_translation(
                            resource_transform.translation + Vec3::new(0.0, 0.1, 0.0)
                        ),
                        ..default()
                    },
                    ResourceIndicator {
                        target: resource_entity,
                    },
                ));

                // Also spawn a text prompt
                commands.spawn((
                    InteractionPrompt {
                        target: resource_entity,
                        offset: Vec3::new(0.0, 1.5, 0.0),
                        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    },
                    // Use a text bundle for the prompt
                    Text2dBundle {
                        text: Text::from_section(
                            "Press F to gather",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ).with_alignment(TextAlignment::Center),
                        transform: Transform::from_translation(
                            resource_transform.translation + Vec3::new(0.0, 1.5, 0.0)
                        ),
                        ..default()
                    },
                ));
            }
            // If out of range but has indicator, remove it
            else if distance >= interaction_range && has_indicator {
                for (indicator_entity, indicator) in indicator_query.iter() {
                    if indicator.target == resource_entity {
                        commands.entity(indicator_entity).despawn();
                    }
                }
            }
        }
    }
}

// System to update interaction prompts
fn update_interaction_prompts(
    mut commands: Commands,
    mut prompt_query: Query<(Entity, &mut InteractionPrompt, &mut Transform, &mut Text)>,
    resource_query: Query<&Transform, Without<InteractionPrompt>>,
    time: Res<Time>,
) {
    for (entity, mut prompt, mut transform, mut text) in prompt_query.iter_mut() {
        // Update timer
        prompt.timer.tick(time.delta());

        // Make the text pulse
        if prompt.timer.just_finished() {
            // Toggle visibility for pulsing effect
            text.sections[0].style.color = if text.sections[0].style.color.a() > 0.5 {
                Color::rgba(1.0, 1.0, 1.0, 0.5)
            } else {
                Color::rgba(1.0, 1.0, 1.0, 1.0)
            };
        }

        // Update position to follow resource
        if let Ok(resource_transform) = resource_query.get(prompt.target) {
            transform.translation = resource_transform.translation + prompt.offset;

            // Make text face camera by setting rotation to identity
            transform.rotation = Quat::IDENTITY;
        } else {
            // If resource no longer exists, remove the prompt
            commands.entity(entity).despawn();
        }
    }
}
