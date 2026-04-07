use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::campfire::*;
use crate::campsite::*;

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 300.0;
const EDGE_BUFFER: f32 = 200.0;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Sprite::from_image(asset_server.load("person.png")),
            Transform::from_xyz(0., 0., 1.),
        ),
        Player,
    ));
}

pub fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut campfire_query: Query<
        &mut Transform,
        (With<Campfire>, Without<Player>),
    >,
    mut campsite_query: Query<
        &mut Transform,
        (With<Campsite>, Without<Campfire>, Without<Player>),
    >,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let window_width = window.width();
    let window_height = window.height();

    let Ok(mut player_transform) = player_query.single_mut() else {
        // println!("Player not found");
        return;
    };
    let Ok(mut campfire_transform) = campfire_query.single_mut() else {
        // println!("Campfire not found");
        return;
    };
    let Ok(mut campsite_transform) = campsite_query.single_mut() else {
        // println!("Campsite background not found");
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight)
    {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction == Vec3::ZERO {
        return;
    }

    direction = direction.normalize_or_zero();
    let movement = direction * PLAYER_SPEED * time.delta_secs();

    let x_limit = (window_width / 2.0) - EDGE_BUFFER;
    let y_limit = (window_height / 2.0) - EDGE_BUFFER;

    let new_player_pos = player_transform.translation + movement;
    if new_player_pos.x.abs() < x_limit {
        player_transform.translation.x += movement.x;
    } else {
        campfire_transform.translation.x -= movement.x;
        campsite_transform.translation.x -= movement.x;
    }

    if new_player_pos.y.abs() < y_limit {
        player_transform.translation.y += movement.y;
    } else {
        campfire_transform.translation.y -= movement.y;
        campsite_transform.translation.y -= movement.y;
    }
}
