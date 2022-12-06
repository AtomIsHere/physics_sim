use bevy::prelude::{Component, Query, Res, Transform, Windows};

#[derive(Component)]
pub struct Ball {
    pub mass: f32,
    pub elasticity: f32,
}

#[derive(Component)]
pub struct BallMovement {
    pub direction: f32,
    pub speed: f32
}

pub fn ball_movement_system(windows: Res<Windows>, mut query: Query<(&Ball, &BallMovement, &mut Transform)>) {
    let (ball, ball_movement, mut transform) = query.single_mut();
    let window = windows.get_primary().unwrap();
}
