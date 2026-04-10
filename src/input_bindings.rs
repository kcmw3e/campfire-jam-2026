use bevy::prelude::*;

#[derive(Resource)]
pub struct KeyBinds {
    pub left: Vec<KeyCode>,
    pub right: Vec<KeyCode>,
    pub up: Vec<KeyCode>,
    pub down: Vec<KeyCode>,
    pub quit: Vec<KeyCode>,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            left: vec![KeyCode::KeyA, KeyCode::ArrowLeft],
            right: vec![KeyCode::KeyD, KeyCode::ArrowRight],
            up: vec![KeyCode::KeyW, KeyCode::ArrowUp],
            down: vec![KeyCode::KeyS, KeyCode::ArrowDown],
            quit: vec![KeyCode::KeyQ, KeyCode::Escape],
        }
    }
}

impl KeyBinds {
    fn any_pressed(
        &self,
        keys: &[KeyCode],
        input: &ButtonInput<KeyCode>,
    ) -> bool {
        keys.iter().any(|k| input.pressed(*k))
    }

    pub fn left(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.any_pressed(&self.left, input)
    }
    pub fn right(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.any_pressed(&self.right, input)
    }
    pub fn up(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.any_pressed(&self.up, input)
    }
    pub fn down(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.any_pressed(&self.down, input)
    }
    pub fn quit(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.any_pressed(&self.quit, input)
    }
}
