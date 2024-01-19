extern crate storytree_native;

use storytree_native::{
    event::{
        Event
        , keyboard::{KeyCode, KeyEvent},
    },
    modal::{Dialog, FontWeight},
    prelude::{WindowBuilder, WindowContext},
    Window,
};
use storytree_native::event::App;

fn main() {
    match Dialog::font()
        .weight(FontWeight::Bold)
        .underline()
        .strikethrough()
        .italic()
        .size(12)
        // This will make the specified window the parent of this dialog
        .show()
    {
        Ok(result) => println!("{:?}", result),
        Err(error) => eprintln!("{}", error),
    }
}
