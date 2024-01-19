extern crate storytree_native;

use storytree_native::{modal::Dialog, prelude::*};

fn main() {
    match Dialog::file()
        .title("Select File(s)")
        .multiple()
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
        .open_file()
    {
        Ok(result) => println!("{:?}", result),
        Err(error) => eprintln!("{}", error),
    }
}
