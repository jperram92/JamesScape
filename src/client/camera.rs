use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::client::input::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            camera_follow_player,
            camera_zoom,
        ));
    }
}

// Component to mark the main camera
#[derive(Component)]
pub struct MainCamera {
    pub zoom_level: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
    pub offset: Vec3,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            zoom_level: 10.0,
            min_zoom: 5.0,
            max_zoom: 20.0,
            zoom_speed: 1.0,
            offset: Vec3::new(0.0, 3.0, 10.0),
        }
    }
}

// System to make camera follow the player
fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((mut camera_transform, camera)) = camera_query.get_single_mut() {
            // Calculate target position based on player position and camera offset
            let target_position = player_transform.translation + camera.offset;

            // Smoothly interpolate camera position
            camera_transform.translation = camera_transform.translation.lerp(
                target_position,
                5.0 * time.delta_seconds()
            );

            // Make camera look at player
            let look_target = player_transform.translation + Vec3::new(0.0, 1.0, 0.0);
            let look_direction = (look_target - camera_transform.translation).normalize();

            if look_direction.length_squared() > 0.0 {
                camera_transform.look_to(look_direction, Vec3::Y);
            }
        }
    }
}

// System to handle camera zoom with mouse wheel
fn camera_zoom(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // Calculate zoom amount from mouse wheel
    let mut zoom_amount = 0.0;
    for event in mouse_wheel_events.read() {
        zoom_amount += event.y;
    }

    // Also allow zoom with keyboard keys
    if keyboard_input.pressed(KeyCode::Equals) || keyboard_input.pressed(KeyCode::Plus) {
        zoom_amount += 0.1;
    }
    if keyboard_input.pressed(KeyCode::Minus) {
        zoom_amount -= 0.1;
    }

    // Apply zoom if needed
    if zoom_amount != 0.0 {
        if let Ok((_, mut camera)) = camera_query.get_single_mut() {
            // Update zoom level
            camera.zoom_level = (camera.zoom_level - zoom_amount * camera.zoom_speed)
                .clamp(camera.min_zoom, camera.max_zoom);

            // Update camera position based on zoom level
            let zoom_ratio = camera.zoom_level / 10.0;
            camera.offset = Vec3::new(0.0, 3.0 * zoom_ratio, 10.0 * zoom_ratio);
        }
    }
}
