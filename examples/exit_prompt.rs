extern crate storytree_native;

use storytree_native::{prelude::*, Window};
use storytree_native::event::{App, close, Event, keyboard::{KeyCode, KeyEvent}, quit};
use storytree_native::modal::{Button, Buttons, Dialog};
use storytree_native::style::{Background, Theme};

fn main() {
    let _ = Window::builder()
        .title("Rust Window")
        .theme(Theme::Auto)
        .background(Background::new(0xA35FC1, 0x0B0B0B))
        .icon("../../assets/images/NativeUI.ico")
        .show()
        .unwrap();

    App::run(|id, event| match event {
        Event::Keyboard(KeyEvent::KeyDown(key)) => match key {
            KeyCode::Escape => {
                close(id);
            }
            _ => {}
        },
        Event::Close => {
            if Dialog::prompt()
                .title("Exit Application")
                .message("Are you sure?")
                .buttons(Buttons::OkCancel)
                .show()
                == Button::Ok
            {
                quit(0)
            }
        }
        _ => {}
    })
}
