mod ball;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::FixedTimestep;
use crate::ball::{Ball, ball_movement_system, BallMovement};

pub const BALL_RADIUS: f32 = 10.;

pub const FPS: i64 = 60;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / (FPS as f64)))
                .with_system(ball_movement_system)
        )
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }).insert(Ball {
        mass: 1.,
        elasticity: 0.8,
    }).insert(BallMovement {
        direction: 1.,
        velocity: 1.,
    });
}