use bevy::prelude::*;
use bevy_easy_gif::*;

pub mod background;
pub mod campfire;
pub mod campsite;
mod campsite_forest_entrance;
pub mod forest;
pub mod player;
pub mod y_sort;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Campsite,
    Forest,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GifPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, player::setup)
        .add_systems(Update, player::movement)
        .add_systems(Update, y_sort::y_sort)
        .add_systems(Update, campsite_forest_entrance::check_entrance)
        .add_systems(OnEnter(GameState::Forest), forest::setup)
        .add_systems(OnEnter(GameState::Campsite), campsite::setup)
        .add_systems(OnEnter(GameState::Campsite), campfire::setup)
        .add_systems(
            OnEnter(GameState::Campsite),
            campsite_forest_entrance::setup,
        )
        .add_systems(OnExit(GameState::Forest), forest::teardown)
        .add_systems(OnExit(GameState::Campsite), campsite::teardown)
        .add_systems(OnExit(GameState::Campsite), campfire::teardown)
        .add_systems(
            OnExit(GameState::Campsite),
            campsite_forest_entrance::teardown,
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
