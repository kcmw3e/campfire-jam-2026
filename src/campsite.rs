use bevy::prelude::*;

use crate::background::*;
use crate::constants::*;
use crate::y_sort::*;

#[derive(Component)]
#[component(on_add = on_add_background)]
pub struct Campsite;

pub const CAMPSITE_SIZE: Vec2 = BACKGROUND_SIZE;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Sprite {
                custom_size: Some(CAMPSITE_SIZE),
                image: asset_server.load("campsite_bg.png"),
                ..default()
            },
            Transform::from_translation(DEFAULT_POS.extend(DEFAULT_Z)),
        ),
        Campsite,
        YSort { z: Z_INDEX.bg },
    ));
}

pub fn teardown(mut commands: Commands, query: Query<Entity, With<Campsite>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
