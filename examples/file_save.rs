extern crate storytree_native;

use storytree_native::{
    event::{
        Event
        , keyboard::{Key, KeyEvent},
    },
    modal::Dialog,
    prelude::{WindowBuilder, WindowContext},
    Window,
};
use storytree_native::event::App;

fn main() {
    let _ = Window::builder().title("Rust Window").show().unwrap();

    // Run the Program, when the window opens `Escape` can be pressed to open a dialog
    App::run(|id, event| match event {
        Event::Keyboard(KeyEvent::KeyDown(key)) => match key {
            Key::Escape => {
                match Dialog::file()
                    .title("Save a File")
                    .filters(
                        1,
                        &[
                            ("All Files", &["*"]),
                            ("Text Files", &["txt"]),
                            ("Image Files", &["png", "jpg", "avif"]),
                        ],
                    )
                    .filename("text.txt")
                    // This will make the specified window the parent of this dialog
                    .save_file_with(id)
                {
                    Ok(result) => println!("{:?}", result),
                    Err(error) => eprintln!("{}", error),
                }
            }
            _ => {}
        },
        _ => {}
    })
}
