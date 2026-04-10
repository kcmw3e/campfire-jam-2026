use bevy::prelude::*;

use crate::GameState;
use crate::background::*;
use crate::campsite::*;
use crate::player::*;
use crate::y_sort::*;

#[derive(Component)]
#[component(on_add = on_add_background)]
pub struct CFEntrance;

pub const ENTRANCE_SIZE: Vec2 = Vec2::new(75., 125.);
pub const C_F_ENTRANCE_POS: Vec2 =
    Vec2::new(CAMPSITE_SIZE.x / 2. - ENTRANCE_SIZE.x / 2., 0.);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Sprite {
                custom_size: Some(ENTRANCE_SIZE),
                image: asset_server.load("campsite_forest_entrance.png"),
                ..default()
            },
            Transform::from_xyz(C_F_ENTRANCE_POS.x, C_F_ENTRANCE_POS.y, 0.),
        ),
        CFEntrance,
        YSort { z: 1. },
    ));
}

pub fn check_entrance(
    player_query: Query<&Transform, With<Player>>,
    entrance_query: Query<&Transform, With<CFEntrance>>,
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

    let half_entrance = ENTRANCE_SIZE / 2.;
    let in_x = (player_pos.x - entrance_pos.x).abs() < half_entrance.x;
    let in_y = (player_pos.y + PLAYER_SIZE.y / 4. - entrance_pos.y).abs()
        < half_entrance.y;

    if in_x && in_y {
        next_state.set(GameState::Forest);
    }
}

pub fn teardown(
    mut commands: Commands,
    query: Query<Entity, With<CFEntrance>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
