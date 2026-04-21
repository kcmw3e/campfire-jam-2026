use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

use crate::GameState;
use crate::background::{BACKGROUND_SIZE, Background};
use crate::campsite::Campsite;
use crate::forest::Forest;
use crate::input_bindings::KeyBinds;
use crate::y_sort::{DEFAULT_POS, DEFAULT_Z, YSort, z_indices};

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_SIZE: Vec2 = Vec2::new(30., 50.);
const PLAYER_STARTING_POS: Vec2 = Vec2::new(-PLAYER_SIZE.x, DEFAULT_POS.y);

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
        ))
        .with_child(Camera2d::default());
}

/// Returns a direction vector based on the currently pressed movement keys (WASD or arrow keys)
fn get_direction(
    keyboard: Res<ButtonInput<KeyCode>>,
    key_binds: Res<KeyBinds>,
) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if key_binds.left(&keyboard) {
        direction.x -= 1.;
    }
    if key_binds.right(&keyboard) {
        direction.x += 1.;
    }
    if key_binds.up(&keyboard) {
        direction.y += 1.;
    }
    if key_binds.down(&keyboard) {
        direction.y -= 1.;
    }

    direction
}

pub fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    key_binds: Res<KeyBinds>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let direction = get_direction(keyboard, key_binds);
    if direction == Vec3::ZERO {
        return;
    }

    let movement =
        direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();

    let Ok(mut player) = player_query.single_mut() else {
        return;
    };
    player.translation.x += movement.x;
    player.translation.y += movement.y;
}
