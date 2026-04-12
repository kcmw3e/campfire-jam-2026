use bevy::prelude::*;
use bevy_easy_gif::*;

pub mod background;
pub mod campfire;
pub mod campsite;
pub mod entrance;
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
        .add_systems(Startup, player::setup)
        .add_systems(
            OnEnter(GameState::Forest),
            (entrance::setup, forest::setup, player::enter_forest),
        )
        .add_systems(
            OnEnter(GameState::Campsite),
            (
                campsite::setup,
                campfire::setup,
                entrance::setup,
                player::enter_campsite,
            ),
        )
        .add_systems(
            OnExit(GameState::Forest),
            (entrance::teardown, forest::teardown),
        )
        .add_systems(
            OnExit(GameState::Campsite),
            (campsite::teardown, campfire::teardown, entrance::teardown),
        )
        .add_systems(
            FixedUpdate,
            (player::movement, entrance::check_entrance).after(player::setup),
        )
        .add_systems(Update, (y_sort::y_sort, handle_quit))
        .run();
}

fn handle_quit(
    keyboard: Res<ButtonInput<KeyCode>>,
    binds: Res<input_bindings::KeyBinds>,
) {
    if binds.quit.iter().any(|k| keyboard.just_pressed(*k)) {
        std::process::exit(0);
    }
}
