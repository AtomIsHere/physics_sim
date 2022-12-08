use std::f32::consts::PI;
use bevy::prelude::{Component, Quat, Query, Res, Transform, Vec3, Windows, Without};
use crate::BALL_RADIUS;

pub const GRAVITY: f32 = 0.1;

#[derive(Component)]
pub struct Ball {
    pub mass: f32,
    pub elasticity: f32,
}

#[derive(Component)]
pub struct BallMovement {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Component)]
pub struct VelocityVectorArrow;

pub fn ball_movement_system(mut query: Query<(&Ball, &mut BallMovement, &mut Transform)>) {
    // Acquire data from query
    let (_, mut ball_movement, mut transform) = query.single_mut();

    // Apply velocity
    transform.translation.x += ball_movement.dx;
    transform.translation.y += ball_movement.dy;

    // Apply gravity
    ball_movement.dy -= GRAVITY;
}

pub fn ball_bounce_system(windows: Res<Windows>, mut query: Query<(&Ball, &mut BallMovement, &mut Transform)>) {
    // Acquire data
    let (ball, mut ball_movement, mut transform) = query.single_mut();
    let window = windows.get_primary().unwrap();

    // Calculate bounds
    let max_y: f32 = window.height()/2.;
    let min_y: f32 = -max_y;

    let max_x: f32 = window.width()/2.;
    let min_x: f32 = -max_x;

    // Check if ball hits y bounds, then bounce
    if max_y <= transform.translation.y + BALL_RADIUS {
        // Ensure the bounce always occurs on the edge. This approximation should be fine as the
        // error is small enough to ignore for most practical purposes.
        transform.translation.y = max_y - BALL_RADIUS;

        bounce_y(&mut ball_movement, ball.elasticity);
    } else if min_y >= transform.translation.y - BALL_RADIUS {
        transform.translation.y = min_y + BALL_RADIUS;

        bounce_y(&mut ball_movement, ball.elasticity)
    } else if max_x <= transform.translation.x + BALL_RADIUS {
        transform.translation.x = max_x - BALL_RADIUS;

        bounce_x(&mut ball_movement, ball.elasticity);
    } else if min_x >= transform.translation.x - BALL_RADIUS {
        transform.translation.x = min_x + BALL_RADIUS;

        bounce_x(&mut ball_movement, ball.elasticity)
    }
}

pub fn velocity_arrow_system(ball_query: Query<(&Ball, &Transform, &BallMovement), Without<VelocityVectorArrow>>, mut vector_arrow_query: Query<(&VelocityVectorArrow, &mut Transform), Without<Ball>>) {
    const PI_ON_2: f32 = PI / 2.;

    let (_, ball_transform, ball_movement) = ball_query.single();
    let (_, mut vec_transform) = vector_arrow_query.single_mut();

    vec_transform.translation.x = ball_transform.translation.x;
    vec_transform.translation.y = ball_transform.translation.y;

    let angle = (ball_movement.dy / ball_movement.dx).atan() - PI_ON_2;

    if ball_movement.dx >= 0. {
        vec_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle)
    } else {
        vec_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), PI + angle)
    }
}

fn bounce_y(movement: &mut BallMovement, elasticity: f32) {
    movement.dy = -movement.dy * elasticity;
}

fn bounce_x(movement: &mut BallMovement, elasticity: f32) {
    movement.dx = -movement.dx * elasticity;
}
