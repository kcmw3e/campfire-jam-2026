use bevy::prelude::*;

pub const DEFAULT_POS: Vec2 = Vec2::new(0., 0.);
pub const DEFAULT_Z: f32 = 0.;

pub struct ZIndex {
    pub bg: f32,
    pub mg: f32,
    pub fg: f32,
}
pub const Z_INDEX: ZIndex = ZIndex { bg: 0., mg: 1., fg: 2. };
