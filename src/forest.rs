use bevy::prelude::*;

use crate::background::{BACKGROUND_SIZE, on_add_background};
use crate::y_sort::{DEFAULT_POS, DEFAULT_Z, YSort, z_indices};

#[derive(Component)]
#[component(on_add = on_add_background)]
pub struct Forest;

pub const FOREST_SIZE: Vec2 = BACKGROUND_SIZE;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Sprite {
                custom_size: Some(FOREST_SIZE),
                image: asset_server.load("forest_bg.png"),
                ..default()
            },
            Transform::from_translation(DEFAULT_POS.extend(DEFAULT_Z)),
        ),
        Forest,
        YSort { z: z_indices::BACKGROUND },
    ));
}

pub fn teardown(mut commands: Commands, query: Query<Entity, With<Forest>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
