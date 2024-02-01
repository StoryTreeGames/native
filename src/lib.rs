#[cfg(target_os = "unix")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub mod event;
pub mod style;
mod window;
pub use window::Window;
pub mod error;
pub mod modal;
pub mod prelude;

/// Check if the window is in fullscreen mode
pub fn is_maxamized(id: isize) -> bool {
    #[cfg(target_os = "windows")]
    windows::is_maxamized(id)
}

pub fn toggle_fullscreen(id: isize) {
    #[cfg(target_os = "windows")]
    windows::window::toggle_fullscreen(id)
}
