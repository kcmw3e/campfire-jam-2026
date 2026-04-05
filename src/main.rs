use bevy::prelude::*;
use bevy_easy_gif::*;

mod campfire;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GifPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, campfire::setup)
        .add_systems(Startup, player::setup)
        .add_systems(Update, player::movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
