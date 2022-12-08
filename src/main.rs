mod ball;
mod arrow;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::FixedTimestep;
use bevy_prototype_lyon::draw::{DrawMode, FillMode};
use bevy_prototype_lyon::prelude::{GeometryBuilder, ShapePlugin};
use crate::arrow::Arrow;
use crate::ball::{Ball, ball_bounce_system, ball_movement_system, BallMovement, velocity_arrow_system, VelocityVectorArrow};

pub const BALL_RADIUS: f32 = 10.;

pub const FPS: i64 = 60;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / (FPS as f64)))
                .with_system(ball_movement_system)
                .with_system(ball_bounce_system)
                .with_system(velocity_arrow_system)
        )
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let arrow = Arrow {
        rectangle_length: BALL_RADIUS + 10.,
        rectangle_width: 5.,
        triangle_offset: 3.,
        triangle_height: 10.,
    };
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
        dx: -2.,
        dy: 1.,
    });

    commands.spawn(GeometryBuilder::build_as(
        &arrow,
        DrawMode::Fill(FillMode::color(Color::WHITE)),
        Transform::default()
    )).insert(VelocityVectorArrow);
}