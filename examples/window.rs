extern crate storytree_native;

use std::fmt::Debug;

use storytree_native::event::App;
use storytree_native::style::{Background, Theme};
use storytree_native::toggle_fullscreen;
use storytree_native::{
    event::{
        close,
        keyboard::{KeyCode, KeyEvent},
        Event,
    },
    prelude::*,
    Window,
};

/// Controls the state of the modifier keys
#[derive(Debug, Default, Clone, Copy)]
struct KeyState {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub caps: bool,
}

fn main() {
    let _ = Window::builder()
        .title("Rust Window")
        // Try changing this to `Light` and `Dark` and see what happens
        .theme(Theme::Auto)
        .background(Background::new(0xA35FC1, 0x0B0B0B))
        .icon("examples/images/NativeUI")
        .show()
        .unwrap();

    App::run_with(KeyState::default(), |id, event, state| match event {
        Event::Keyboard(KeyEvent::KeyDown(key)) => match key {
            KeyCode::Control => state.as_mut().ctrl = true,
            KeyCode::Alt => state.as_mut().alt = true,
            KeyCode::Shift => state.as_mut().shift = true,
            KeyCode::Capital => state.as_mut().caps = true,

            // Window controls
            KeyCode::F11 => toggle_fullscreen(id),
            KeyCode::Escape => close(id),

            key => {
                // Print key with current modifiers
                println!("{:?}: {:?}", state.as_ref(), key);
            }
        },
        Event::Keyboard(KeyEvent::KeyUp(key)) => match key {
            KeyCode::Control => state.as_mut().ctrl = false,
            KeyCode::Alt => state.as_mut().alt = false,
            KeyCode::Shift => state.as_mut().shift = false,
            KeyCode::Capital => state.as_mut().caps = false,
            _ => {}
        },
        _ => {}
    });
}
