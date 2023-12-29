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