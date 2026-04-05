use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

use crate::campfire::*;
use crate::campsite::*;
use crate::forest::*;
use crate::y_sort::*;

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 300.;
pub const PLAYER_SIZE: Vec2 = Vec2::new(30., 50.);
const EDGE_BUFFER: f32 = 200.;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Sprite {
                custom_size: Some(PLAYER_SIZE),
                image: asset_server.load("person.png"),
                ..default()
            },
            Anchor::BOTTOM_CENTER,
            Transform::from_xyz(-50., 0., 0.),
        ),
        Player,
        YSort { z: 10. },
    ));
}

/// Returns a direction vector based on the currently pressed movement keys (WASD or arrow keys)
fn get_direction(keyboard: Res<ButtonInput<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight)
    {
        direction.x += 1.;
    }
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.;
    }

    direction
}

/// Returns (player_delta, world_delta) for one axis.
fn resolve_axis(
    player_pos: f32,
    player_extent: f32, // half-height or half-width for screen edge check
    site_pos: f32,
    half_site: f32,
    half_screen: f32,
    limit: f32,
    movement: f32,
) -> (f32, f32) {
    let moving_deeper = movement.signum() == player_pos.signum();

    if player_pos.abs() < limit || !moving_deeper {
        // Free zone or moving back toward center — move player, clamped to screen
        let new_pos = player_pos + movement;
        if (new_pos + player_extent).abs() < half_screen {
            (movement, 0.)
        } else {
            (0., 0.)
        }
    } else {
        // Buffer zone, moving deeper — try scrolling the world
        let new_site_pos = site_pos - movement;
        let site_near_edge = new_site_pos - half_site; // left or bottom
        let site_far_edge = new_site_pos + half_site; // right or top

        if site_near_edge <= -half_screen && site_far_edge >= half_screen {
            (0., -movement)
        } else {
            // Campsite at its limit — move player if still on screen
            let new_pos = player_pos + movement;
            if (new_pos + player_extent).abs() < half_screen {
                (movement, 0.)
            } else {
                (0., 0.)
            }
        }
    }
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
    let Ok(mut player_transform) = player_query.single_mut() else {
        return;
    };
    let Ok(mut campfire_transform) = campfire_query.single_mut() else {
        return;
    };
    // TODO: Find a better way to get the campsite transform without assuming it's the first
    let Some(campsite_transform) = campsite_query.iter_mut().nth(0) else {
        return;
    };

    let half_screen = Vec2::new(window.width(), window.height()) / 2.;
    // TODO: Once forest env is added, this will need to be dynamically set based on current map
    let half_site = CAMPSITE_SIZE / 2.;
    let limit = half_screen - Vec2::splat(EDGE_BUFFER);

    let direction = get_direction(keyboard);
    if direction == Vec3::ZERO {
        return;
    }

    let movement =
        direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();

    let player = player_transform.translation;
    let site = campsite_transform.translation;

    let (pdx, wdx) = resolve_axis(
        player.x,
        0., // x: anchor is center, no extent needed
        site.x,
        half_site.x,
        half_screen.x,
        limit.x,
        movement.x,
    );
    let (pdy, wdy) = resolve_axis(
        player.y,
        PLAYER_SIZE.y / 2., // y: anchor is bottom, offset by half height
        site.y,
        half_site.y,
        half_screen.y,
        limit.y,
        movement.y,
    );

    player_transform.translation.x += pdx;
    player_transform.translation.y += pdy;
    campfire_transform.translation.x += wdx;
    campfire_transform.translation.y += wdy;
    for mut campsite_transform in campsite_query.iter_mut() {
        campsite_transform.translation.x += wdx;
        campsite_transform.translation.y += wdy;
    }
}
