use bevy::prelude::*;

#[derive(Component)]
pub struct Campsite;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Sprite::from_image(asset_server.load("campsite_bg.png")),
            Transform::from_xyz(0., 0., 0.),
        ),
        Campsite,
    ));
}
