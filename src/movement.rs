use bevy::input::keyboard::KeyboardInput;
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

pub fn keyboard_events(mut events: MessageReader<KeyboardInput>) {
    for event in events.read() {
        for quit_key in KeyBinds::default().quit {
            if event.key_code == quit_key && event.state.is_pressed() {
                std::process::exit(0);
            }
        }
    }
}
