extern crate storytree_native;

use storytree_native::{
    modal::Dialog,
    prelude::WindowBuilder
    ,
};

fn main() {
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
        .save_file()
    {
        Ok(result) => println!("{:?}", result),
        Err(error) => eprintln!("{}", error),
    }
}
