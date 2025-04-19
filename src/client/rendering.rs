use bevy::prelude::*;
use bevy::render::mesh::shape::Cube;
use crate::client::input::Player;
use crate::client::physics::{Velocity, Acceleration, Collider, ColliderShape, Gravity, OnGround, JumpStrength};
use crate::shared::components::{Skills, Health};
use crate::systems::combat_system::CombatState;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
           .add_systems(Update, (update_rendering, follow_camera));
    }
}

fn update_rendering() {
    // Rendering logic will go here
}

// Camera follows the player
fn follow_camera(player_query: Query<&Transform, With<Player>>, mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Calculate camera position based on player position
            let offset = Vec3::new(-2.0, 2.5, 5.0); // Camera offset from player
            let target_position = player_transform.translation + offset;

            // Smoothly interpolate camera position
            camera_transform.translation = camera_transform.translation.lerp(target_position, 0.1);

            // Make camera look at player
            let look_target = player_transform.translation + Vec3::new(0.0, 1.0, 0.0); // Look at player's head
            camera_transform.look_at(look_target, Vec3::Y);
        }
    }
}

/// Set up a simple 3D scene with a camera and some objects
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Add a cube representing a player with physics components
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cube { size: 1.0 }.into()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0), // Start a bit above ground to avoid collision issues
            ..default()
        },
        Player,  // Add the Player component to enable movement
        Velocity { linear: Vec3::ZERO, angular: 0.0 },
        Acceleration { linear: Vec3::ZERO },
        Collider {
            radius: 0.5,
            height: 1.0,
            shape: ColliderShape::Capsule
        },
        Gravity(9.8),
        OnGround(false),
        JumpStrength(8.0),
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
        },
        Health {
            current: 100,
            maximum: 100,
        },
        CombatState::default(),
    ));
}
