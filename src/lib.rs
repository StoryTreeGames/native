#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "unix")]
mod linux;

pub mod event;
pub mod style;
mod window;
pub use window::Window;
pub mod prelude;
pub mod modal;
pub mod error;

/// Check if the window is in fullscreen mode
pub fn is_maxamized(id: isize) -> bool {
    #[cfg(target_os = "windows")]
    windows::is_maxamized(id)
}

pub fn toggle_fullscreen(id: isize) {
    #[cfg(target_os = "windows")]
    windows::window::toggle_fullscreen(id)
}
