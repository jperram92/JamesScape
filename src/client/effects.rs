use bevy::prelude::*;
use crate::client::input::Player;
use crate::systems::skills_system::GatheringInProgress;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_gathering_effects,
            update_effect_particles,
        ));
    }
}

// Component for particle effects
#[derive(Component)]
pub struct ParticleEffect {
    pub lifetime: Timer,
    pub velocity: Vec3,
    pub fade_start: f32,
    pub size_change: f32,
}

// System to create and update gathering effects
fn update_gathering_effects(
    mut commands: Commands,
    player_query: Query<(&Transform, Option<&GatheringInProgress>), With<Player>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((player_transform, gathering)) = player_query.get_single() {
        if let Some(gathering) = gathering {
            // Only spawn particles occasionally
            if (time.elapsed_seconds() * 5.0).sin() > 0.9 {
                // Determine effect type based on resource
                match gathering.resource_type {
                    crate::client::terrain::ResourceNodeType::Tree => {
                        // Wood chips effect
                        spawn_particles(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            player_transform.translation + Vec3::new(0.0, 0.5, 0.0),
                            5, // Number of particles
                            Color::rgb(0.6, 0.4, 0.2), // Brown color
                            0.05, // Small particles
                            1.0, // Medium lifetime
                            1.5, // Medium speed
                        );
                    },
                    crate::client::terrain::ResourceNodeType::Rock |
                    crate::client::terrain::ResourceNodeType::OreDeposit => {
                        // Rock dust effect
                        spawn_particles(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            player_transform.translation + Vec3::new(0.0, 0.5, 0.0),
                            8, // More particles
                            Color::rgb(0.7, 0.7, 0.7), // Gray color
                            0.03, // Smaller particles
                            0.8, // Shorter lifetime
                            2.0, // Faster speed
                        );
                    },
                    crate::client::terrain::ResourceNodeType::FishingSpot => {
                        // Water splash effect
                        spawn_particles(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            player_transform.translation + Vec3::new(0.0, 0.2, 1.0), // In front of player
                            10, // More particles
                            Color::rgb(0.3, 0.5, 1.0), // Blue color
                            0.04, // Medium particles
                            0.6, // Short lifetime
                            1.0, // Medium speed
                        );
                    },
                }
            }
        }
    }
}

// Helper function to spawn particles
fn spawn_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    count: usize,
    color: Color,
    size: f32,
    lifetime: f32,
    speed: f32,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: size }));

    let material = materials.add(StandardMaterial {
        base_color: color,
        emissive: color * 0.5, // Make particles glow
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    for _ in 0..count {
        // Random direction
        let direction = Vec3::new(
            rand::random::<f32>() * 2.0 - 1.0,
            rand::random::<f32>() * 1.0, // Mostly upward
            rand::random::<f32>() * 2.0 - 1.0,
        ).normalize();

        // Random speed variation
        let particle_speed = speed * (0.8 + rand::random::<f32>() * 0.4);

        // Random lifetime variation
        let particle_lifetime = lifetime * (0.8 + rand::random::<f32>() * 0.4);

        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            ParticleEffect {
                lifetime: Timer::from_seconds(particle_lifetime, TimerMode::Once),
                velocity: direction * particle_speed,
                fade_start: 0.7,
                size_change: -0.5, // Shrink over time
            },
        ));
    }
}

// System to update particle effects
fn update_effect_particles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ParticleEffect, &mut Transform, &mut Handle<StandardMaterial>)>,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut effect, mut transform, material_handle) in query.iter_mut() {
        // Update timer
        effect.lifetime.tick(time.delta());

        // Move particle
        transform.translation += effect.velocity * time.delta_seconds();

        // Apply gravity
        effect.velocity.y -= 2.0 * time.delta_seconds();

        // Calculate remaining lifetime ratio
        let remaining = effect.lifetime.remaining_secs() / effect.lifetime.duration().as_secs_f32();

        // Fade out
        if remaining < effect.fade_start {
            let alpha = remaining / effect.fade_start;
            if let Some(material) = materials.get_mut(material_handle.id()) {
                let mut color = material.base_color;
                color.set_a(alpha);
                material.base_color = color;
            }
        }

        // Change size
        let scale_factor = 1.0 + effect.size_change * (1.0 - remaining);
        transform.scale = Vec3::splat(scale_factor.max(0.1)); // Don't let it get too small

        // Remove when timer is finished
        if effect.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
