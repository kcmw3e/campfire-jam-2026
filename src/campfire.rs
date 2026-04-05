use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_easy_gif::*;

use crate::y_sort::*;

#[derive(Component)]
pub struct Campfire;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Gif { handle: asset_server.load("campfire/campfire_preview.gif") },
            Anchor::BOTTOM_CENTER,
            Transform::from_xyz(0., 0., 0.),
        ),
        Campfire,
        YSort { z: 10. },
    ));
}

pub fn teardown(mut commands: Commands, query: Query<Entity, With<Campfire>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

