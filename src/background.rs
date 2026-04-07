use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

pub const BACKGROUND_SIZE: Vec2 = Vec2::new(3000., 1650.);

pub fn on_add_background(mut world: DeferredWorld, hook: HookContext) {
    world.commands().entity(hook.entity).insert(Background);
}
