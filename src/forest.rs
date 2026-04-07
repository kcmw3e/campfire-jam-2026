use bevy::prelude::*;

use crate::background::*;
use crate::y_sort::*;

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
            Transform::from_xyz(0., 0., 0.),
        ),
        Forest,
        YSort { z: 0. },
    ));
}

pub fn teardown(mut commands: Commands, query: Query<Entity, With<Forest>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
