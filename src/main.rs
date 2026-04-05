use bevy::prelude::*;
use bevy_easy_gif::*;

pub mod campfire;
pub mod campsite;
mod campsite_forest_entrance;
pub mod player;
pub mod y_sort;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GifPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, campsite::setup)
        .add_systems(Startup, campsite_forest_entrance::setup)
        .add_systems(Startup, campfire::setup)
        .add_systems(Startup, player::setup)
        .add_systems(Update, player::movement)
        .add_systems(Update, y_sort::y_sort)
        .add_systems(Update, campsite_forest_entrance::check_entrance)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
