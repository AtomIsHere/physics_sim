use bevy::prelude::{Component, Query, Res, Transform, Windows};

pub const GRAVITY: f32 = 0.1;

#[derive(Component)]
pub struct Ball {
    pub mass: f32,
    pub elasticity: f32,
}

#[derive(Component)]
pub struct BallMovement {
    pub direction: f32,
    pub velocity: f32
}

pub fn ball_movement_system(mut query: Query<(&Ball, &mut BallMovement, &mut Transform)>) {
    // Acquire data from query
    let (_, mut ball_movement, mut transform) = query.single_mut();

    // Calculate velocity
    let dx = ball_movement.velocity * ball_movement.direction.cos();
    let dy = ball_movement.velocity * ball_movement.direction.sin();

    // Apply velocity
    transform.translation.x += dx;
    transform.translation.y += dy;

    ball_movement.velocity = (dx.powi(2) + (dy - GRAVITY).powi(2)).sqrt();
    ball_movement.direction = ((dy - GRAVITY)/dx).atan();
}
