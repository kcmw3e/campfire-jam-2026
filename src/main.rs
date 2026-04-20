use bevy::prelude::*;
use bevy_easy_gif::*;

use crate::campfire::CampfirePlugin;

pub mod background;
pub mod campfire;
pub mod campsite;
mod campsite_forest_entrance;
pub mod forest;
pub mod input_bindings;
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
        .init_resource::<input_bindings::KeyBinds>()
        .add_systems(Startup, setup)
        .add_systems(Startup, player::setup.after(setup))
        .add_systems(OnEnter(GameState::Forest), forest::setup)
        .add_systems(
            OnEnter(GameState::Campsite),
            (campsite::setup, campsite_forest_entrance::setup),
        )
        .add_systems(OnExit(GameState::Forest), forest::teardown)
        .add_systems(
            OnExit(GameState::Campsite),
            (campsite::teardown, campsite_forest_entrance::teardown),
        )
        .add_systems(
            FixedUpdate,
            (player::movement, campsite_forest_entrance::check_entrance)
                .after(player::setup),
        )
        .add_systems(Update, (y_sort::y_sort, handle_quit))
        .add_plugins(CampfirePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn handle_quit(
    keyboard: Res<ButtonInput<KeyCode>>,
    binds: Res<input_bindings::KeyBinds>,
) {
    if binds.quit.iter().any(|k| keyboard.just_pressed(*k)) {
        std::process::exit(0);
    }
}
