use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

use crate::GameState;
use crate::background::*;
use crate::campsite::*;
use crate::forest::*;
use crate::keyboard::*;
use crate::y_sort::*;

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_SIZE: Vec2 = Vec2::new(30., 50.);
const EDGE_BUFFER: f32 = 200.;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Sprite {
                custom_size: Some(PLAYER_SIZE),
                image: asset_server.load("player.png"),
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
fn get_direction(
    keyboard: Res<ButtonInput<KeyCode>>,
    key_binds: Res<KeyBinds>,
) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if key_binds.left.iter().any(|k| keyboard.pressed(*k)) {
        direction.x -= 1.;
    }
    if key_binds.right.iter().any(|k| keyboard.pressed(*k)) {
        direction.x += 1.;
    }
    if key_binds.up.iter().any(|k| keyboard.pressed(*k)) {
        direction.y += 1.;
    }
    if key_binds.down.iter().any(|k| keyboard.pressed(*k)) {
        direction.y -= 1.;
    }

    direction
}

/// Returns (player_delta, world_delta) for one axis.
fn resolve_axis(
    player_pos: f32,
    player_extent: f32, // half-height or half-width for screen edge check
    site_pos: f32,
    half_bg: f32,
    half_screen: f32,
    limit: f32,
    movement: f32,
) -> (f32, f32) {
    let moving_outwards = (player_pos + movement).abs() > player_pos.abs();

    if player_pos.abs() < limit || !moving_outwards {
        let new_pos = player_pos + movement;
        if (new_pos + player_extent).abs() < half_screen {
            (movement, 0.)
        } else {
            (0., 0.)
        }
    } else {
        // Buffer zone, moving deeper — try scrolling the world
        let new_site_pos = site_pos - movement;
        let site_near_edge = new_site_pos - half_bg; // left or bottom
        let site_far_edge = new_site_pos + half_bg; // right or top

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
    key_binds: Res<KeyBinds>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_state: Res<State<GameState>>,
    mut queries: ParamSet<(
        Query<&Transform, With<Campsite>>, // p0: read campsite pos
        Query<&Transform, With<Forest>>,   // p1: read forest pos
        Query<&mut Transform, With<Background>>, // p2: move all bgs
        Query<&mut Transform, With<Player>>, // p3: move player
    )>,
) {
    let Ok(window) = window_query.single() else {
        println!("early return: no window");
        return;
    };
    let background = match game_state.get() {
        GameState::Campsite => {
            let campsite_query = queries.p0();
            let Ok(t) = campsite_query.single() else {
                println!("early return: no campsite");
                return;
            };
            t.translation
        },
        GameState::Forest => {
            let forest_query = queries.p1();
            let Ok(t) = forest_query.single() else {
                println!("early return: no forest");
                return;
            };
            t.translation
        },
    };
    let player = {
        let player_query = queries.p3();
        let Ok(t) = player_query.single() else {
            println!("early return: no player");
            return;
        };
        t.translation
    };

    let half_screen = Vec2::new(window.width(), window.height()) / 2.;
    let half_bg = BACKGROUND_SIZE / 2.;
    let limit = half_screen - Vec2::splat(EDGE_BUFFER);

    let direction = get_direction(keyboard, key_binds);
    if direction == Vec3::ZERO {
        return;
    }

    let movement =
        direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();

    let (pdx, wdx) = resolve_axis(
        player.x,
        0., // x: anchor is center, no extent needed
        background.x,
        half_bg.x,
        half_screen.x,
        limit.x,
        movement.x,
    );
    let (pdy, wdy) = resolve_axis(
        player.y,
        PLAYER_SIZE.y / 2., // y: anchor is bottom, offset by half height
        background.y,
        half_bg.y,
        half_screen.y,
        limit.y,
        movement.y,
    );

    let mut player_query = queries.p3();
    let Ok(mut pt) = player_query.single_mut() else {
        return;
    };
    pt.translation.x += pdx;
    pt.translation.y += pdy;
    drop(player_query);

    let mut bg_query = queries.p2();
    for mut bg in bg_query.iter_mut() {
        bg.translation.x += wdx;
        bg.translation.y += wdy;
    }
}
