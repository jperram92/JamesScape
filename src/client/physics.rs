use bevy::prelude::*;

// Physics components
#[derive(Component, Debug)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: f32,
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub linear: Vec3,
}

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub height: f32,
    #[allow(dead_code)]
    pub shape: ColliderShape,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ColliderShape {
    Sphere,
    Capsule,
    Box,
    Cylinder,
}

#[derive(Component, Debug)]
pub struct Gravity(pub f32);

#[derive(Component, Debug)]
pub struct OnGround(pub bool);

#[derive(Component, Debug)]
pub struct JumpStrength(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_gravity,
            apply_acceleration,
            apply_velocity,
            check_ground_collision,
            check_object_collisions,
        ).chain());
    }
}

// Apply gravity to entities with Velocity and Gravity components
fn apply_gravity(
    mut query: Query<(&mut Velocity, &Gravity, &OnGround)>,
    time: Res<Time>,
) {
    for (mut velocity, gravity, on_ground) in query.iter_mut() {
        if !on_ground.0 {
            velocity.linear.y -= gravity.0 * time.delta_seconds();
        }
    }
}

// Apply acceleration to velocity
fn apply_acceleration(
    mut query: Query<(&mut Velocity, &Acceleration)>,
    time: Res<Time>,
) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.linear += acceleration.linear * time.delta_seconds();

        // Apply friction to horizontal movement when on ground
        let friction = 5.0;
        let horizontal_velocity = Vec3::new(velocity.linear.x, 0.0, velocity.linear.z);
        if horizontal_velocity.length() > 0.0 {
            let friction_force = horizontal_velocity.normalize() * friction * time.delta_seconds();
            let new_horizontal_velocity = horizontal_velocity - friction_force;

            // Only apply friction if it doesn't change direction
            if new_horizontal_velocity.dot(horizontal_velocity) > 0.0 {
                velocity.linear.x = new_horizontal_velocity.x;
                velocity.linear.z = new_horizontal_velocity.z;
            } else {
                velocity.linear.x = 0.0;
                velocity.linear.z = 0.0;
            }
        }

        // Clamp maximum velocity
        let max_horizontal_speed = 5.0;
        let horizontal_speed = Vec2::new(velocity.linear.x, velocity.linear.z).length();
        if horizontal_speed > max_horizontal_speed {
            let scale = max_horizontal_speed / horizontal_speed;
            velocity.linear.x *= scale;
            velocity.linear.z *= scale;
        }

        // Clamp maximum vertical velocity
        let max_vertical_speed = 10.0;
        velocity.linear.y = velocity.linear.y.clamp(-max_vertical_speed, max_vertical_speed);
    }
}

// Apply velocity to position
fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.linear * time.delta_seconds();
        transform.rotate_y(velocity.angular * time.delta_seconds());
    }
}

// Check for ground collisions
fn check_ground_collision(
    mut query: Query<(&Transform, &Collider, &mut Velocity, &mut OnGround)>,
) {
    let ground_height = 0.0;

    for (transform, collider, mut velocity, mut on_ground) in query.iter_mut() {
        let entity_bottom = transform.translation.y - collider.height / 2.0;

        if entity_bottom <= ground_height {
            // Entity is on or below ground
            on_ground.0 = true;

            // Adjust position to be exactly on ground
            // (This would be handled by a proper physics system)

            // Stop downward velocity
            if velocity.linear.y < 0.0 {
                velocity.linear.y = 0.0;
            }
        } else {
            // Entity is above ground
            on_ground.0 = false;
        }
    }
}

// Check for collisions between objects
fn check_object_collisions(
    mut query: Query<(Entity, &Transform, &Collider, &mut Velocity)>,
) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([(_, transform_a, collider_a, mut velocity_a), (_, transform_b, collider_b, mut velocity_b)]) = combinations.fetch_next() {
        // Simple sphere-sphere collision detection
        let distance = transform_a.translation.distance(transform_b.translation);
        let min_distance = collider_a.radius + collider_b.radius;

        if distance < min_distance {
            // Calculate collision response
            let direction = (transform_a.translation - transform_b.translation).normalize();

            // Move entities apart (simplified collision resolution)
            let overlap = min_distance - distance;
            let response_magnitude = overlap * 0.5;

            // Apply collision response to velocities
            velocity_a.linear += direction * response_magnitude;
            velocity_b.linear -= direction * response_magnitude;
        }
    }
}
