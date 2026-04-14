use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::campsite::CAMPSITE_SIZE;
use crate::entrance::ENTRANCE_SIZE;
use crate::forest::FOREST_SIZE;
use crate::movement::{MoveState, OldMoveState};
use crate::y_sort::{DEFAULT_POS, DEFAULT_Z, YSort, z_indices};
#[derive(Component)]
pub struct Player;

pub const PLAYER_SIZE: Vec2 = Vec2::new(30., 50.);
pub const PLAYER_STARTING_POS: Vec2 = Vec2::new(-PLAYER_SIZE.x, DEFAULT_POS.y);
const FOREST_PLAYER_POS: Vec2 =
    Vec2::new(-FOREST_SIZE.x / 2. + ENTRANCE_SIZE.x, DEFAULT_POS.y);
const CAMPSITE_PLAYER_POS: Vec2 =
    Vec2::new(CAMPSITE_SIZE.x / 2. - ENTRANCE_SIZE.x, DEFAULT_POS.y);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            (
                Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    image: asset_server.load("player.png"),
                    ..default()
                },
                Anchor::BOTTOM_CENTER,
                Transform::from_translation(
                    PLAYER_STARTING_POS.extend(DEFAULT_Z),
                ),
            ),
            Player,
            YSort { z: z_indices::MIDGROUND },
            MoveState {
                position: PLAYER_STARTING_POS.extend(DEFAULT_Z),
                ..default()
            },
            OldMoveState { position: PLAYER_STARTING_POS.extend(DEFAULT_Z) },
        ))
        .with_child(Camera2d::default());
}

pub fn enter_forest(
    mut player_query: Query<(&mut MoveState, &mut OldMoveState), With<Player>>,
) {
    let Ok((mut state, mut old_state)) = player_query.single_mut() else {
        return;
    };
    old_state.position = FOREST_PLAYER_POS.extend(DEFAULT_Z);
    state.position = FOREST_PLAYER_POS.extend(DEFAULT_Z);
}

pub fn enter_campsite(
    mut player_query: Query<(&mut MoveState, &mut OldMoveState), With<Player>>,
) {
    let Ok((mut state, mut old_state)) = player_query.single_mut() else {
        return;
    };
    old_state.position = CAMPSITE_PLAYER_POS.extend(DEFAULT_Z);
    state.position = CAMPSITE_PLAYER_POS.extend(DEFAULT_Z);
}
