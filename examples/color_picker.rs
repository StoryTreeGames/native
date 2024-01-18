extern crate storytree_native;

use std::fs;

use storytree_native::{
    modal::{Dialog, DialogAction},
    prelude::{WindowBuilder, WindowContext}
    ,
};

fn rgb(color: u32) -> (u8, u8, u8) {
    let color: [u8; 4] = color.to_be_bytes();
    (color[1], color[2], color[3])
}

/// File that will contain custom colors between example runs.
static COLORS_CACHE: &'static str = "examples/custom_colors.txt";

/// Load custom colors from cache file
fn load_custom_colors() -> Vec<u32> {
    match fs::read_to_string(COLORS_CACHE) {
        Ok(text) => {
            text.lines().filter_map(|line| {
                if line.trim().len() > 0 {
                    return line.parse::<u32>().ok();
                }
                None
            }).collect::<Vec<u32>>()
        }
        Err(error) => {
            panic!("{}", error);
        }
    }
}

/// Save custom colors to cache file
fn save_custom_colors(colors: &Vec<u32>) {
    fs::write(COLORS_CACHE, colors.iter().map(|v| format!("{}\n", v)).collect::<String>()).expect("Could not save custom colors");
}

fn main() {
    match Dialog::color()
        // What color should be chosen first
        .initial(0xA0FF0C)
        // Sets the custom colors for the dialog. This must be set each time the dialog is shown
        .custom_colors(load_custom_colors())
        .show()
    {
        Ok(result) => {
            // Unwrap the dialog result/action and convert to rgb tuples for updated custom colors
            let (color, colors) = if let DialogAction::Color(clr, clrs) = result {
                save_custom_colors(&clrs);
                (rgb(clr), clrs.iter().map(|c| rgb(*c)).collect::<Vec<(u8, u8, u8)>>())
            } else {
                (rgb(0xFFFFFF), Vec::new())
            };

            print!("\x1b[38;2;{};{};{}m██\x1b[39m│", color.0, color.1, color.2);

            // This will print custom colors out to the terminal similar to how they are layed out in the windows
            // color dialog. This is unique because it is a vertical zigzag pattern going from left to right.
            let colors = colors.split_at(colors.len() / 2);
            for (fg, bg) in colors.0.iter().zip(colors.1.iter()) {
                print!("\x1b[38;2;{};{};{};48;2;{};{};{}m▀\x1b[39;49m", fg.0, fg.1, fg.2, bg.0, bg.1, bg.2);
            }
            print!("\n");
        }
        Err(error) => eprintln!("{}", error),
    }
}
