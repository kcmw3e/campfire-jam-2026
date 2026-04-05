use bevy::prelude::*;

use crate::y_sort::*;

#[derive(Component)]
pub struct Campsite;

pub const CAMPSITE_SIZE: Vec2 = Vec2::new(3000., 1650.);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Sprite {
                custom_size: Some(CAMPSITE_SIZE),
                image: asset_server.load("campsite_bg.png"),
                ..default()
            },
            Transform::from_xyz(0., 0., 0.),
        ),
        Campsite,
        YSort { z: 0. },
    ));
}
