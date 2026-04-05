use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 300.0;

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
    mut query: Query<&mut Transform, With<Player>>,
) {
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
    for mut transform in &mut query {
        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}
