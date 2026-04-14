use bevy::prelude::*;

use crate::BACKGROUND_SIZE;

pub const DEFAULT_POS: Vec2 = Vec2::ZERO;
pub const DEFAULT_Z: f32 = 0.;

pub mod z_indices {
    pub const BACKDROP: f32 = 0.;
    pub const ENTRANCE: f32 = 1.;
    pub const MIDGROUND: f32 = 2.;
    pub const FOREGROUND: f32 = 3.;
}

#[derive(Component)]
pub struct YSort {
    pub z: f32,
}

pub fn y_sort(mut q: Query<(&mut Transform, &YSort)>) {
    for (mut tf, y_sort) in q.iter_mut() {
        tf.translation.z = y_sort.z - (tf.translation.y / BACKGROUND_SIZE.y);
    }
}
