mod ball;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();


}

#[derive(Component)]
pub struct Constants {
    pub gravity: f32
}