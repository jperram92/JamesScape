use bevy::prelude::*;
use bevy::render::mesh::shape::{Plane, Cylinder, UVSphere};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use crate::client::physics::{Collider, ColliderShape};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_terrain);
    }
}

// Terrain settings
#[derive(Resource)]
pub struct TerrainSettings {
    pub size: f32,
    #[allow(dead_code)]
    pub height_scale: f32,
    #[allow(dead_code)]
    pub noise_scale: f32,
    pub seed: u32,
}

impl Default for TerrainSettings {
    fn default() -> Self {
        Self {
            size: 100.0,
            height_scale: 5.0,
            noise_scale: 0.1,
            seed: 42,
        }
    }
}

// Terrain types
#[derive(Component)]
pub struct Terrain;

#[derive(Component)]
#[allow(dead_code)]
pub enum BiomeType {
    Forest,
    Plains,
    Mountains,
    Desert,
}

// Resource node types
#[derive(Component, Debug, Clone)]
pub enum ResourceNodeType {
    Tree,
    Rock,
    OreDeposit,
    FishingSpot,
}

// Generate terrain with different biomes
fn generate_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let settings = TerrainSettings::default();
    let perlin = Perlin::new(settings.seed);
    let mut rng = rand::thread_rng();

    // Create ground plane with perlin noise height
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane { size: settings.size, subdivisions: 100 }.into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Terrain,
    ));

    // Generate trees in forest biome
    for _ in 0..50 {
        let x = rng.gen_range(-settings.size/2.0..settings.size/2.0);
        let z = rng.gen_range(-settings.size/2.0..settings.size/2.0);

        // Use noise to determine if we should place a tree here
        let noise_value = perlin.get([x as f64 * 0.05, z as f64 * 0.05]);

        if noise_value > 0.2 && noise_value < 0.8 {
            let height = 1.5 + (noise_value as f32 * 1.5);
            let radius = 0.2 + (noise_value as f32 * 0.1);

            // Tree trunk
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cylinder {
                        radius,
                        height,
                        resolution: 8,
                        segments: 1,
                    }.into()),
                    material: materials.add(Color::rgb(0.6, 0.4, 0.2).into()),
                    transform: Transform::from_xyz(x, height/2.0, z),
                    ..default()
                },
                ResourceNodeType::Tree,
                Collider {
                    radius,
                    height,
                    shape: ColliderShape::Cylinder,
                },
            ));

            // Tree leaves
            commands.spawn(PbrBundle {
                mesh: meshes.add(UVSphere {
                    radius: radius * 3.0,
                    sectors: 8,
                    stacks: 8,
                }.into()),
                material: materials.add(Color::rgb(0.2, 0.6, 0.2).into()),
                transform: Transform::from_xyz(x, height + radius * 1.5, z),
                ..default()
            });
        }
    }

    // Generate rocks
    for _ in 0..30 {
        let x = rng.gen_range(-settings.size/2.0..settings.size/2.0);
        let z = rng.gen_range(-settings.size/2.0..settings.size/2.0);

        // Use noise to determine if we should place a rock here
        let noise_value = perlin.get([x as f64 * 0.1 + 100.0, z as f64 * 0.1 + 100.0]);

        if noise_value > 0.5 {
            let size = 0.5 + (noise_value as f32 * 0.5);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(UVSphere {
                        radius: size,
                        sectors: 8,
                        stacks: 8,
                    }.into()),
                    material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                    transform: Transform::from_xyz(x, size, z),
                    ..default()
                },
                ResourceNodeType::Rock,
                Collider {
                    radius: size,
                    height: size * 2.0,
                    shape: ColliderShape::Sphere,
                },
            ));
        }
    }

    // Generate ore deposits
    for _ in 0..15 {
        let x = rng.gen_range(-settings.size/2.0..settings.size/2.0);
        let z = rng.gen_range(-settings.size/2.0..settings.size/2.0);

        let size = rng.gen_range(0.3..0.7);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(UVSphere {
                    radius: size,
                    sectors: 8,
                    stacks: 8,
                }.into()),
                material: materials.add(Color::rgb(0.6, 0.3, 0.1).into()),
                transform: Transform::from_xyz(x, size, z),
                ..default()
            },
            ResourceNodeType::OreDeposit,
            Collider {
                radius: size,
                height: size * 2.0,
                shape: ColliderShape::Sphere,
            },
        ));
    }

    // Generate fishing spots
    for _ in 0..10 {
        let x = rng.gen_range(-settings.size/2.0..settings.size/2.0);
        let z = rng.gen_range(-settings.size/2.0..settings.size/2.0);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Plane { size: 1.0, subdivisions: 0 }.into()),
                material: materials.add(Color::rgba(0.2, 0.4, 0.8, 0.7).into()),
                transform: Transform::from_xyz(x, 0.01, z),
                ..default()
            },
            ResourceNodeType::FishingSpot,
        ));
    }

    // Add some mountains
    for _ in 0..5 {
        let x = rng.gen_range(-settings.size/2.0..settings.size/2.0);
        let z = rng.gen_range(-settings.size/2.0..settings.size/2.0);

        let height = rng.gen_range(5.0..10.0);
        let radius = rng.gen_range(3.0..8.0);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(UVSphere {
                    radius,
                    sectors: 16,
                    stacks: 16,
                }.into()),
                material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
                transform: Transform::from_xyz(x, height/2.0 - radius/2.0, z),
                ..default()
            },
            BiomeType::Mountains,
            Collider {
                radius,
                height: radius * 2.0,
                shape: ColliderShape::Sphere,
            },
        ));
    }
}
