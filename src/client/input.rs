use bevy::prelude::*;
use crate::client::physics::{Acceleration, Velocity, OnGround, JumpStrength};

pub struct InputPlugin;

// Tag component for the player entity
#[derive(Component)]
pub struct Player;

// Player control settings
#[derive(Resource)]
pub struct PlayerSettings {
    // These fields are for future use
    #[allow(dead_code)]
    pub movement_speed: f32,
    #[allow(dead_code)]
    pub jump_strength: f32,
    pub acceleration: f32,
    pub air_control: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            movement_speed: 5.0,
            jump_strength: 8.0,
            acceleration: 30.0,
            air_control: 0.3,
        }
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSettings>()
           .add_systems(Update, (
               handle_keyboard_input,
               handle_jump_input,
               handle_camera_input,
           ));
    }
}

// Handle keyboard input for player movement
fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    settings: Res<PlayerSettings>,
    mut query: Query<(&Transform, &mut Acceleration, &OnGround), With<Player>>,
    _time: Res<Time>,
) {
    if let Ok((transform, mut acceleration, on_ground)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Get movement direction from keyboard input
        if keyboard_input.pressed(KeyCode::W) {
            direction.z += 1.0;  // Forward is positive Z
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.z -= 1.0;  // Backward is negative Z
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        // Normalize direction if needed
        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        // Adjust direction based on player's facing direction
        let forward = transform.forward();
        let right = transform.right();
        let movement_direction = right * direction.x + forward * direction.z;

        // Apply acceleration based on whether player is on ground
        let accel_strength = if on_ground.0 {
            settings.acceleration
        } else {
            settings.acceleration * settings.air_control
        };

        // Set acceleration
        acceleration.linear.x = movement_direction.x * accel_strength;
        acceleration.linear.z = movement_direction.z * accel_strength;
    }
}

// Handle jump input
fn handle_jump_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &OnGround, &JumpStrength), With<Player>>,
) {
    if let Ok((mut velocity, on_ground, jump_strength)) = query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && on_ground.0 {
            velocity.linear.y = jump_strength.0;
        }
    }
}

// Handle camera input
fn handle_camera_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        let mut rotation = 0.0;
        let rotation_speed = 2.0;

        if keyboard_input.pressed(KeyCode::Q) {
            rotation += rotation_speed;
        }
        if keyboard_input.pressed(KeyCode::E) {
            rotation -= rotation_speed;
        }

        velocity.angular = rotation;
    }
}
