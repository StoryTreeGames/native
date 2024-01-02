extern crate storytree_native;

use storytree_native::{
    event::{
        Event
        , keyboard::{KeyCode, KeyEvent},
    },
    modal::Dialog,
    prelude::{WindowBuilder, WindowContext},
    Window,
};
use storytree_native::event::App;

fn main() {
    let _ = Window::builder().title("Rust Window").show().unwrap();

    // Run the Program, when the window opens `Escape` can be pressed to open a file select dialog
    App::run(|id, event, _| match event {
        Event::Keyboard(KeyEvent::KeyDown(key)) => match key {
            KeyCode::Escape => {
                match Dialog::color()
                .initial(0xA0FF0C)
                // This will make the specified window the parent of this dialog
                .show_with(id)
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
