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
    match Dialog::file()
        .title("Choose a Folder")
        .filters(
            1,
            &[
                ("All Files", &["*"]),
                ("Text Files", &["txt"]),
                ("Image Files", &["png", "jpg", "avif"]),
            ],
        )
        .directory("../")
        // This will make the specified window the parent of this dialog
        .open_folder()
    {
        Ok(result) => println!("{:?}", result),
        Err(error) => eprintln!("{}", error),
    }
}
