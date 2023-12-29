use std::{cell::RefCell, sync::{Arc, RwLockWriteGuard}};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::{RwLock, RwLockReadGuard};

use keyboard::KeyEvent;
use mouse::MouseEvent;
use crate::event::mouse::MouseEventType;

pub mod keyboard;
pub mod mouse;

pub trait IntoEventResult {
    fn into_event_result(self) -> bool;
}

impl IntoEventResult for () {
    fn into_event_result(self) -> bool {
        true
    }
}

impl IntoEventResult for bool {
    fn into_event_result(self) -> bool {
        self
    }
}

#[derive(Debug, Clone)]
pub struct PaintEvent {
    pub handle: isize,
}

#[derive(Debug, Clone)]
pub enum Event {
    Close,
    Repaint,
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
}

pub trait IntoEvent {
    fn into_event(self) -> Event;
}

pub fn close(id: isize) {
    #[cfg(target_os = "windows")]
    unsafe {
        use crate::windows::event::wnd_proc;
        use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
        use windows::Win32::UI::WindowsAndMessaging::{CallWindowProcW, WM_CLOSE};

        CallWindowProcW(Some(wnd_proc), HWND(id), WM_CLOSE, WPARAM(0), LPARAM(0));
    }
}

pub fn quit(code: i32) {
    #[cfg(target_os = "windows")]
    unsafe {
        ::windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
    }
    std::process::exit(code);
}

#[derive(Clone)]
pub struct State<T: Send + Sync + Clone>(Arc<RwLock<T>>);
impl<T: Clone + Send + Sync> State<T> {
    pub fn new(state: T) -> Self {
        Self(Arc::new(RwLock::new(state)))
    }

    pub fn as_ref(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().unwrap()
    }

    pub fn as_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap()
    }
}
impl Default for State<()> {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(())))
    }
}

impl<T: Send + Sync + Clone + Debug> Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub struct App;
impl App
{
    pub fn run<F, R>(callback: F)
    where
        F: Fn(isize, Event, State<()>) -> R + 'static + Sync + Send,
        R: IntoEventResult,
    {
        #[cfg(target_os = "windows")]
        crate::windows::event::run(State::default(), callback);
    }

    pub fn run_with<S, F, R>(state: S, callback: F)
        where
            S: Clone + Send + Sync + 'static,
            F: Fn(isize, Event, State<S>) -> R + 'static + Sync + Send,
            R: IntoEventResult,
    {
        #[cfg(target_os = "windows")]
        crate::windows::event::run(State::new(state), callback);
    }
}

pub fn run<R, F, T>(state: T, callback: F)
where
    R: IntoEventResult,
    F: Fn(isize, Event, State<T>) -> R + 'static + Sync + Send,
    T: Clone + Send + Sync + 'static
{
    #[cfg(target_os = "windows")]
    crate::windows::event::run(State::new(state), callback);
}
