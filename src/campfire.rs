use bevy::prelude::*;
use bevy_easy_gif::*;

#[derive(Component)]
pub struct Campfire;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Gif { handle: asset_server.load("campfire/campfire_preview.gif") },
            Transform::from_xyz(0., 0., 1.),
        ),
        Campfire,
    ));
}
