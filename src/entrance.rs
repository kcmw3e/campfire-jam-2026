use bevy::prelude::*;

use crate::GameState;
use crate::background::on_add_background;
use crate::campsite::CAMPSITE_SIZE;
use crate::forest::FOREST_SIZE;
use crate::player::{Player, PLAYER_SIZE};
use crate::y_sort::{DEFAULT_Z, YSort, z_indices};

#[derive(Component)]
#[component(on_add = on_add_background)]
pub struct Entrance;

pub const ENTRANCE_SIZE: Vec2 = Vec2::new(75., 125.);
pub const TO_FOREST_ENTRANCE_POS: Vec2 =
    Vec2::new(CAMPSITE_SIZE.x / 2. - ENTRANCE_SIZE.x / 2., 0.);
pub const TO_CAMPSITE_ENTRANCE_POS: Vec2 =
    Vec2::new(-FOREST_SIZE.x / 2. + ENTRANCE_SIZE.x / 2., 0.);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
) {
    let (entrance_pos, asset_path) = match game_state.as_ref().get() {
        GameState::Campsite => {
            (TO_FOREST_ENTRANCE_POS, "to_forest_entrance.png")
        },
        GameState::Forest => {
            (TO_CAMPSITE_ENTRANCE_POS, "to_campsite_entrance.png")
        },
    };

    commands.spawn((
        (
            Sprite {
                custom_size: Some(ENTRANCE_SIZE),
                image: asset_server.load(asset_path),
                ..default()
            },
            Transform::from_translation(entrance_pos.extend(DEFAULT_Z)),
        ),
        Entrance,
        YSort { z: z_indices::MIDGROUND },
    ));
}

pub fn check_entrance(
    player_query: Query<&Transform, With<Player>>,
    entrance_query: Query<&Transform, With<Entrance>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(entrance_transform) = entrance_query.single() else {
        return;
    };

    let player_pos = player_transform.translation;
    let entrance_pos = entrance_transform.translation;

    let half_player = PLAYER_SIZE / 2.;
    let half_entrance = ENTRANCE_SIZE / 2.;
    let in_x = (player_pos.x - entrance_pos.x).abs() < half_player.x + half_entrance.x;
    let in_y = (player_pos.y - entrance_pos.y).abs() < half_entrance.y;

    let new_state = match game_state.as_ref().get() {
        GameState::Campsite => GameState::Forest,
        GameState::Forest => GameState::Campsite,
    };

    if in_x && in_y {
        next_state.set(new_state);
    }
}

pub fn teardown(mut commands: Commands, query: Query<Entity, With<Entrance>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
