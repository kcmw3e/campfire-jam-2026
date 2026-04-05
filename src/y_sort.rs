use bevy::prelude::*;

#[derive(Component)]
pub struct YSort {
    pub z: f32,
}

pub fn y_sort(mut q: Query<(&mut Transform, &YSort)>) {
    for (mut tf, y_sort) in q.iter_mut() {
        tf.translation.z = y_sort.z - (tf.translation.y * 0.001);
    }
}
