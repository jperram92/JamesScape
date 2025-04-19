use bevy::prelude::*;
use crate::client::input::Player;
use crate::client::physics::{Velocity, OnGround};
use crate::systems::skills_system::GatheringInProgress;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_character_assets)
           .add_systems(Update, (
               update_character_animations,
               update_character_direction,
           ));
    }
}

// Character components
#[derive(Component)]
pub struct CharacterModel {
    pub head: Entity,
    pub body: Entity,
    pub left_arm: Entity,
    pub right_arm: Entity,
    pub left_leg: Entity,
    pub right_leg: Entity,
}

#[derive(Component)]
pub struct AnimationState {
    pub current: AnimationType,
    pub timer: Timer,
    pub direction: Vec3,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AnimationType {
    Idle,
    Walking,
    Running,
    Jumping,
    Gathering,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current: AnimationType::Idle,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            direction: Vec3::Z,
        }
    }
}

// System to set up character assets
fn setup_character_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<Entity, With<Player>>,
) {
    // Create character materials with bright, cartoon-like colors
    let skin_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.8, 0.6),
        perceptual_roughness: 0.9,
        emissive: Color::rgb(0.1, 0.05, 0.0), // Slight glow to make it more visible
        ..default()
    });

    let hair_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.2, 0.1),
        perceptual_roughness: 0.8,
        emissive: Color::rgb(0.05, 0.02, 0.0),
        ..default()
    });

    let shirt_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.5, 1.0), // Brighter blue
        perceptual_roughness: 0.7,
        emissive: Color::rgb(0.0, 0.1, 0.2),
        ..default()
    });

    let pants_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.4, 0.7), // Brighter pants
        perceptual_roughness: 0.7,
        emissive: Color::rgb(0.0, 0.0, 0.1),
        ..default()
    });

    // For each player entity, create a character model
    for player_entity in player_query.iter() {
        // Create head (larger sphere)
        let head = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.35, // Larger head
                sectors: 16,  // More detail
                stacks: 16,
            })),
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0), // Higher position
            ..default()
        }).id();

        // Create hair (flattened cube on top of head)
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.4, 0.15, 0.4))), // Larger hair
            material: hair_material.clone(),
            transform: Transform::from_xyz(0.0, 1.2, 0.0), // Higher position
            ..default()
        }).set_parent(head);

        // Create body (larger rectangular prism)
        let body = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.6, 0.7, 0.3))), // Larger body
            material: shirt_material.clone(),
            transform: Transform::from_xyz(0.0, 0.45, 0.0),
            ..default()
        }).id();

        // Create arms (thicker rectangular prisms)
        let left_arm = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.15, 0.5, 0.15))), // Thicker arms
            material: shirt_material.clone(),
            transform: Transform::from_xyz(-0.38, 0.45, 0.0), // Further out
            ..default()
        }).id();

        let right_arm = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.15, 0.5, 0.15))),
            material: shirt_material.clone(),
            transform: Transform::from_xyz(0.38, 0.45, 0.0), // Further out
            ..default()
        }).id();

        // Create legs (thicker rectangular prisms)
        let left_leg = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 0.5, 0.2))), // Thicker legs
            material: pants_material.clone(),
            transform: Transform::from_xyz(-0.15, -0.25, 0.0),
            ..default()
        }).id();

        let right_leg = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 0.5, 0.2))),
            material: pants_material.clone(),
            transform: Transform::from_xyz(0.15, -0.25, 0.0),
            ..default()
        }).id();

        // Create character model component and attach to player
        let character_model = CharacterModel {
            head,
            body,
            left_arm,
            right_arm,
            left_leg,
            right_leg,
        };

        // Add animation state
        let animation_state = AnimationState::default();

        // Add components to player entity
        commands.entity(player_entity)
            .insert(character_model)
            .insert(animation_state);

        // Make body parts children of the player entity
        commands.entity(player_entity).push_children(&[head, body, left_arm, right_arm, left_leg, right_leg]);
    }
}

// System to update character animations based on movement and actions
fn update_character_animations(
    time: Res<Time>,
    mut query: Query<(
        &Velocity,
        &OnGround,
        Option<&GatheringInProgress>,
        &mut AnimationState,
        &mut CharacterModel
    ), With<Player>>,
    mut transform_query: Query<&mut Transform>,
) {
    for (velocity, on_ground, gathering, mut anim_state, model) in query.iter_mut() {
        // Update animation timer
        anim_state.timer.tick(time.delta());

        // Determine animation type based on player state
        let new_animation = if gathering.is_some() {
            AnimationType::Gathering
        } else if !on_ground.0 {
            AnimationType::Jumping
        } else if velocity.linear.length() > 5.0 {
            AnimationType::Running
        } else if velocity.linear.length() > 0.5 {
            AnimationType::Walking
        } else {
            AnimationType::Idle
        };

        // If animation changed, reset timer
        if new_animation != anim_state.current {
            anim_state.current = new_animation;
            anim_state.timer.reset();
        }

        // Apply animations continuously
        {
            match anim_state.current {
                AnimationType::Idle => {
                    // Subtle idle animation - slight up/down movement of the body
                    if let Ok(mut body_transform) = transform_query.get_mut(model.body) {
                        let y_offset = (time.elapsed_seconds() * 2.0).sin() * 0.02;
                        body_transform.translation.y = 0.35 + y_offset;
                    }

                    // Reset arm positions
                    if let Ok(mut left_arm_transform) = transform_query.get_mut(model.left_arm) {
                        left_arm_transform.translation = Vec3::new(-0.25, 0.35, 0.0);
                        left_arm_transform.rotation = Quat::IDENTITY;
                    }

                    if let Ok(mut right_arm_transform) = transform_query.get_mut(model.right_arm) {
                        right_arm_transform.translation = Vec3::new(0.25, 0.35, 0.0);
                        right_arm_transform.rotation = Quat::IDENTITY;
                    }

                    // Reset leg positions
                    if let Ok(mut left_leg_transform) = transform_query.get_mut(model.left_leg) {
                        left_leg_transform.translation = Vec3::new(-0.1, -0.15, 0.0);
                        left_leg_transform.rotation = Quat::IDENTITY;
                    }

                    if let Ok(mut right_leg_transform) = transform_query.get_mut(model.right_leg) {
                        right_leg_transform.translation = Vec3::new(0.1, -0.15, 0.0);
                        right_leg_transform.rotation = Quat::IDENTITY;
                    }
                },
                AnimationType::Walking | AnimationType::Running => {
                    // Walking/running animation - swing arms and legs
                    let speed_multiplier = if anim_state.current == AnimationType::Running { 2.0 } else { 1.0 };
                    let swing_amount = (time.elapsed_seconds() * 5.0 * speed_multiplier).sin();

                    // Animate arms
                    if let Ok(mut left_arm_transform) = transform_query.get_mut(model.left_arm) {
                        left_arm_transform.rotation = Quat::from_rotation_x(swing_amount * 0.5);
                    }

                    if let Ok(mut right_arm_transform) = transform_query.get_mut(model.right_arm) {
                        right_arm_transform.rotation = Quat::from_rotation_x(-swing_amount * 0.5);
                    }

                    // Animate legs
                    if let Ok(mut left_leg_transform) = transform_query.get_mut(model.left_leg) {
                        left_leg_transform.rotation = Quat::from_rotation_x(-swing_amount * 0.5);
                    }

                    if let Ok(mut right_leg_transform) = transform_query.get_mut(model.right_leg) {
                        right_leg_transform.rotation = Quat::from_rotation_x(swing_amount * 0.5);
                    }
                },
                AnimationType::Jumping => {
                    // Jumping animation - arms up, legs bent
                    if let Ok(mut left_arm_transform) = transform_query.get_mut(model.left_arm) {
                        left_arm_transform.rotation = Quat::from_rotation_x(-0.5);
                    }

                    if let Ok(mut right_arm_transform) = transform_query.get_mut(model.right_arm) {
                        right_arm_transform.rotation = Quat::from_rotation_x(-0.5);
                    }

                    if let Ok(mut left_leg_transform) = transform_query.get_mut(model.left_leg) {
                        left_leg_transform.rotation = Quat::from_rotation_x(0.3);
                    }

                    if let Ok(mut right_leg_transform) = transform_query.get_mut(model.right_leg) {
                        right_leg_transform.rotation = Quat::from_rotation_x(0.3);
                    }
                },
                AnimationType::Gathering => {
                    // Gathering animation - bend forward, swing arms
                    let swing_amount = (time.elapsed_seconds() * 3.0).sin();

                    // Determine animation based on resource type
                    if let Some(gathering) = gathering {
                        match gathering.resource_type {
                            crate::client::terrain::ResourceNodeType::Tree => {
                                // Chopping animation - swing arms side to side
                                if let Ok(mut body_transform) = transform_query.get_mut(model.body) {
                                    body_transform.rotation = Quat::from_rotation_x(0.2);
                                }

                                if let Ok(mut head_transform) = transform_query.get_mut(model.head) {
                                    head_transform.rotation = Quat::from_rotation_x(0.2);
                                }

                                // Animate arms for chopping motion - more exaggerated
                                if let Ok(mut left_arm_transform) = transform_query.get_mut(model.left_arm) {
                                    // Raise arm up and to the side
                                    left_arm_transform.rotation =
                                        Quat::from_rotation_z(0.8) * // More rotation
                                        Quat::from_rotation_x(swing_amount * 1.0 - 0.7);

                                    // Add a slight translation to make the movement more visible
                                    left_arm_transform.translation.y = 0.45 + swing_amount * 0.05;
                                }

                                if let Ok(mut right_arm_transform) = transform_query.get_mut(model.right_arm) {
                                    // Swing arm down in chopping motion - more exaggerated
                                    right_arm_transform.rotation =
                                        Quat::from_rotation_z(-0.5) * // More rotation
                                        Quat::from_rotation_x(swing_amount * 1.5 - 1.0); // Larger swing

                                    // Add a slight translation to make the movement more visible
                                    right_arm_transform.translation.y = 0.45 + swing_amount * -0.1;
                                    right_arm_transform.translation.z = swing_amount * 0.1; // Forward/backward motion
                                }
                            },
                            crate::client::terrain::ResourceNodeType::Rock |
                            crate::client::terrain::ResourceNodeType::OreDeposit => {
                                // Mining animation - both arms swinging together
                                if let Ok(mut body_transform) = transform_query.get_mut(model.body) {
                                    body_transform.rotation = Quat::from_rotation_x(0.3);
                                }

                                if let Ok(mut head_transform) = transform_query.get_mut(model.head) {
                                    head_transform.rotation = Quat::from_rotation_x(0.3);
                                }

                                // Animate both arms for mining motion - more exaggerated
                                let mining_swing = (time.elapsed_seconds() * 7.0).sin() * 0.8; // Faster, larger swing

                                if let Ok(mut left_arm_transform) = transform_query.get_mut(model.left_arm) {
                                    left_arm_transform.rotation =
                                        Quat::from_rotation_z(0.3) *
                                        Quat::from_rotation_x(mining_swing - 0.9);

                                    // Add movement to make it more visible
                                    left_arm_transform.translation.y = 0.45 + mining_swing * 0.1;
                                    left_arm_transform.translation.z = mining_swing * 0.15; // Forward motion
                                }

                                if let Ok(mut right_arm_transform) = transform_query.get_mut(model.right_arm) {
                                    right_arm_transform.rotation =
                                        Quat::from_rotation_z(-0.3) *
                                        Quat::from_rotation_x(mining_swing - 0.9);

                                    // Add movement to make it more visible
                                    right_arm_transform.translation.y = 0.45 + mining_swing * 0.1;
                                    right_arm_transform.translation.z = mining_swing * 0.15; // Forward motion
                                }
                            },
                            crate::client::terrain::ResourceNodeType::FishingSpot => {
                                // Fishing animation - arm extended forward with more visible motion
                                if let Ok(mut body_transform) = transform_query.get_mut(model.body) {
                                    body_transform.rotation = Quat::from_rotation_x(0.2);
                                    // Add a slight swaying motion
                                    let sway = (time.elapsed_seconds() * 0.5).sin() * 0.05;
                                    body_transform.rotation *= Quat::from_rotation_z(sway);
                                }

                                // More visible rod movement
                                let fishing_motion = (time.elapsed_seconds() * 1.5).sin() * 0.15; // Larger motion
                                let quick_jerk = if (time.elapsed_seconds() * 0.3).sin() > 0.9 { 0.2 } else { 0.0 }; // Occasional jerking motion

                                if let Ok(mut right_arm_transform) = transform_query.get_mut(model.right_arm) {
                                    right_arm_transform.rotation =
                                        Quat::from_rotation_z(-0.4) *
                                        Quat::from_rotation_x(-0.9 + fishing_motion + quick_jerk);

                                    // Add some vertical movement
                                    right_arm_transform.translation.y = 0.45 + fishing_motion * 0.1;
                                }

                                if let Ok(mut left_arm_transform) = transform_query.get_mut(model.left_arm) {
                                    left_arm_transform.rotation =
                                        Quat::from_rotation_z(0.2) *
                                        Quat::from_rotation_x(-0.5 + fishing_motion * 0.3); // Some movement in supporting arm

                                    // Add some vertical movement
                                    left_arm_transform.translation.y = 0.45 + fishing_motion * 0.05;
                                }
                            },
                        }
                    } else {
                        // Default gathering animation if no specific type
                        if let Ok(mut body_transform) = transform_query.get_mut(model.body) {
                            body_transform.rotation = Quat::from_rotation_x(0.2);
                        }

                        if let Ok(mut head_transform) = transform_query.get_mut(model.head) {
                            head_transform.rotation = Quat::from_rotation_x(0.2);
                        }

                        // Animate arms for generic gathering motion
                        if let Ok(mut left_arm_transform) = transform_query.get_mut(model.left_arm) {
                            left_arm_transform.rotation = Quat::from_rotation_x(swing_amount * 0.7 - 0.5);
                        }

                        if let Ok(mut right_arm_transform) = transform_query.get_mut(model.right_arm) {
                            right_arm_transform.rotation = Quat::from_rotation_x(swing_amount * 0.7 - 0.5);
                        }
                    }
                }
            }
        }
    }
}

// System to update character direction based on movement or gathering target
fn update_character_direction(
    mut query: Query<(&Velocity, &mut AnimationState, &Children, Option<&GatheringInProgress>, &Transform), With<Player>>,
    mut transform_query: Query<&mut Transform, Without<Player>>,
) {
    for (velocity, mut anim_state, children, gathering, player_transform) in query.iter_mut() {
        let mut target_rotation = None;

        // If gathering, face the resource
        if let Some(gathering) = gathering {
            if let Some(target_pos) = gathering.target_position {
                // Calculate direction to resource
                let to_resource = target_pos - player_transform.translation;
                let horizontal_to_resource = Vec3::new(to_resource.x, 0.0, to_resource.z).normalize();

                println!("Facing resource at position: {:?}, direction: {:?}", target_pos, horizontal_to_resource);

                if horizontal_to_resource.length() > 0.01 {
                    anim_state.direction = horizontal_to_resource;
                    target_rotation = Some(Quat::from_rotation_y(
                        -f32::atan2(horizontal_to_resource.x, horizontal_to_resource.z)
                    ));
                }
            } else {
                println!("Gathering but no target position set!");
            }
        }
        // Otherwise, face movement direction
        else if velocity.linear.length() > 0.1 {
            let movement_direction = velocity.linear.normalize();

            // Only consider horizontal movement for direction
            let horizontal_direction = Vec3::new(movement_direction.x, 0.0, movement_direction.z).normalize();

            if horizontal_direction.length() > 0.1 {
                anim_state.direction = horizontal_direction;
                target_rotation = Some(Quat::from_rotation_y(
                    -f32::atan2(horizontal_direction.x, horizontal_direction.z)
                ));
            }
        }

        // Apply rotation if we have one
        if let Some(rotation) = target_rotation {
            // Apply rotation to all child entities
            for &child in children.iter() {
                if let Ok(mut transform) = transform_query.get_mut(child) {
                    // Keep the local translation but update the rotation
                    let local_translation = transform.translation;
                    transform.rotation = rotation;
                    transform.translation = local_translation;
                }
            }
        }
    }
}
