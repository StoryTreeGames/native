extern crate storytree_native;

use storytree_native::event::{
    close,
    keyboard::{Key, KeyboardEvent},
    quit, run, Event,
};
use storytree_native::modal::{Button, Buttons, Dialog};
use storytree_native::style::{Background, Theme};
use storytree_native::{prelude::*, Window};

fn main() {
    let _ = Window::builder()
        .title("Rust Window")
        .theme(Theme::Auto)
        .background(Background::new(0xA35FC1, 0x0B0B0B))
        .icon("../../assets/images/NativeUI.ico")
        .show()
        .unwrap();

    run(|id, event| match event {
        Event::Keyboard(KeyboardEvent::KeyDown(key)) => match key {
            Key::Escape => {
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
