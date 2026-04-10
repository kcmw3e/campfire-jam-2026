use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_easy_gif::*;

use crate::background::on_add_background;
use crate::y_sort::{DEFAULT_POS, DEFAULT_Z, YSort, z_indices};

/// The campfire is the central part of the game that the player must interact
/// with. It will burn through fuel over time.
///
/// Campfire fuel represents the number of seconds (in-game time) that the fire
/// has left before it dies and the game is over. Different fuels will have
/// different effects on the campfire. Some will simply prolong the campfire's
/// life by increasing its fuel (up to the capacity), others may increase the
/// capacity itself.
#[derive(Component)]
#[component(on_add = on_add_background)]
pub struct Campfire {
    /// The amount of fuel the campfire has left to burn.
    fuel: f32,

    /// The maximum amount of fuel the campfire can hold.
    capacity: f32,
}

impl Default for Campfire {
    fn default() -> Self {
        Self { fuel: Self::STARTING_FUEL, capacity: Self::STARTING_FUEL }
    }
}

impl Campfire {
    /// The default amount of starting fuel for the campfire.
    const STARTING_FUEL: f32 = 60.; // 1 minute
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (
            Gif { handle: asset_server.load("campfire/campfire_preview.gif") },
            Anchor::BOTTOM_CENTER,
            Transform::from_translation(DEFAULT_POS.extend(DEFAULT_Z)),
        ),
        Campfire::default(),
        YSort { z: z_indices::MIDGROUND },
    ));
}

pub fn teardown(mut commands: Commands, query: Query<Entity, With<Campfire>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
