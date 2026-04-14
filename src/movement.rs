use bevy::prelude::*;

use crate::input_bindings::KeyBinds;
use crate::player::Player;

const MOVE_SPEED: f32 = 500.;

#[derive(Component)]
pub struct MoveState {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct OldMoveState {
    pub position: Vec3,
}

impl Default for MoveState {
    fn default() -> Self {
        Self { position: Vec3::ZERO, velocity: Vec3::ZERO }
    }
}

impl Default for OldMoveState {
    fn default() -> Self {
        Self { position: Vec3::ZERO }
    }
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

pub fn set_velocity(
    keyboard: Res<ButtonInput<KeyCode>>,
    key_binds: Res<KeyBinds>,
    mut player_query: Query<(&mut MoveState, &mut OldMoveState), With<Player>>,
) {
    let direction = get_direction(keyboard, key_binds);

    let Ok((mut state, _)) = player_query.single_mut() else {
        return;
    };

    state.velocity = direction.normalize_or_zero() * MOVE_SPEED;
}

pub fn set_position(
    time: Res<Time>,
    mut move_query: Query<(&mut MoveState, &mut OldMoveState)>,
) {
    for (mut state, mut old_state) in &mut move_query {
        // Reborrow `state` to mutably access both of its fields
        // See Cheatbook page on "Split Borrows"
        let state = &mut *state;

        // Store the old position.
        old_state.position = state.position;

        // Compute the new position.
        // (`delta_seconds` always returns the fixed timestep
        // duration, if this system is added to `FixedUpdate`)
        state.position += state.velocity * time.delta_secs();
    }
}

pub fn interpolate_transform(
    fixed_time: Res<Time<Fixed>>,
    mut player_query: Query<
        (&MoveState, &OldMoveState, &mut Transform),
        With<Player>,
    >,
) {
    let alpha = fixed_time.overstep_fraction();

    let Ok((state, old_state, mut transform)) = player_query.single_mut()
    else {
        return;
    };

    transform.translation = old_state.position.lerp(state.position, alpha);
}
