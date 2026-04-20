use bevy::prelude::*;

use crate::y_sort::{DEFAULT_Z, YSort, z_indices};

#[derive(Component)]
pub struct Fuel;

pub enum FuelType {
    Stick,
    Log,
}

impl Fuel {
    const STICK_SIZE: Vec2 = Vec2::new(25., 25.);
    const LOG_SIZE: Vec2 = Vec2::new(50., 25.);

    static FUEL_COORDS: [(Vec2, FuelType); 2] = [
        (Vec2::new(-100., -50.), FuelType::Stick),
        (Vec2::new(50., -75.), FuelType::Log), 
    ];

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        for (coordinates, fuel_type) in FUEL_COORDS.iter() {
            let asset_path = match fuel_type {
                FuelType::Stick => "fuel/stick.png",
                FuelType::Log => "fuel/log.png",
            };

            let fuel_size = match fuel_type {
                FuelType::Stick => STICK_SIZE,
                FuelType::Log => LOG_SIZE,
            };

            commands.spawn((
                (
                    Sprite {
                        custom_size: Some(fuel_size),
                        image: asset_server.load(asset_path),
                        ..default()
                    },
                    Transform::from_translation(coordinates.extend(DEFAULT_Z)),
                ),
                Fuel,
                YSort { z: z_indices::MIDGROUND },
            ));
        }
    }

    pub fn teardown(mut commands: Commands, query: Query<Entity, With<Fuel>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
