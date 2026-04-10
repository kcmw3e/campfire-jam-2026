use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_easy_gif::*;

use crate::background::on_add_background;
use crate::y_sort::{DEFAULT_POS, DEFAULT_Z, YSort, z_indices};

#[derive(Component)]
#[component(on_add = on_add_background)]
pub struct Campfire;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Gif { handle: asset_server.load("campfire/campfire_preview.gif") },
            Anchor::BOTTOM_CENTER,
            Transform::from_translation(DEFAULT_POS.extend(DEFAULT_Z)),
        ),
        Campfire,
        YSort { z: z_indices::MIDGROUND },
    ));
}

pub fn teardown(mut commands: Commands, query: Query<Entity, With<Campfire>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
