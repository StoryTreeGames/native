use std::cell::RefCell;
use std::mem::transmute;
use std::sync::Arc;

use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, FillRect, HDC};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DestroyWindow, DispatchMessageW, GetClientRect, GetMessageW, GetWindowLongPtrW,
    PostQuitMessage, SetWindowLongPtrW, CREATESTRUCTW, GWLP_USERDATA, MSG, WM_CLOSE, WM_CREATE,
    WM_DESTROY, WM_ERASEBKGND, WM_PAINT,
};

use crate::event::keyboard::KeyboardEvent;
use crate::event::mouse::MouseEvent;
use crate::event::{keyboard, mouse, Event, IntoEventResult};
use crate::style::{Background, Theme};
use crate::window::WindowOptions;
use crate::windows::{is_dark_mode, swap_rb};

#[derive(Default)]
struct Handler {
    handler: Option<Arc<dyn Fn(HWND, u32, WPARAM, LPARAM) + Sync + Send + 'static>>,
}

impl Handler {
    pub fn set_handler<F: Fn(HWND, u32, WPARAM, LPARAM) + Sync + Send + 'static>(
        &mut self,
        handler: F,
    ) {
        self.handler = Some(Arc::new(handler));
    }

    pub fn handle(&self, hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) {
        if let Some(handler) = &self.handler {
            handler(hwnd, message, wparam, lparam);
        }
    }
}

thread_local! {
    static HANDLER: RefCell<Handler> = RefCell::new(Handler::default());
}

/// Converts (u32, usize, isize) to Event
impl From<(u32, WPARAM, LPARAM)> for Event {
    fn from(v: (u32, WPARAM, LPARAM)) -> Self {
        match v.0 {
            _ if keyboard::KeyboardEvent::message(v.0) => Event::Keyboard(KeyboardEvent::from(v)),
            _ if mouse::MouseEvent::message(v.0) => Event::Mouse(MouseEvent::from(v)),
            _ => panic!("Unknown event message: {}", v.0),
        }
    }
}

fn input_message(message: u32) -> bool {
    KeyboardEvent::message(message) || MouseEvent::message(message)
}

pub fn run<R, F, T>(state: T, callback: F)
where
    R: IntoEventResult,
    F: (Fn(isize, Event, T) -> R) + 'static + Sync + Send,
    T: Clone + Send + Sync + 'static,
{
    let mut message = MSG::default();
    let state = state;

    HANDLER.with(move |handler| {
        handler.borrow_mut().set_handler(
            move |hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM| match message {
                _ if input_message(message) => {
                    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
                    callback(
                        hwnd.0,
                        Event::from((message, wparam, lparam)),
                        state.clone(),
                    );
                }
                WM_CLOSE => {
                    let result =
                        { callback(hwnd.0, Event::Close, state.clone()).into_event_result() };
                    if result {
                        let _ = unsafe { DestroyWindow(hwnd) };
                    }
                }
                WM_PAINT => {
                    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
                    callback(hwnd.0, Event::Repaint, state.clone());
                }
                _ => {}
            },
        );
    });

    while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
        unsafe { DispatchMessageW(&message) };
    }
}

pub extern "system" fn wnd_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // TODO: Extra error handling for win32 api calls
    HANDLER.with(|handler| {
        let handler = handler.borrow();
        handler.handle(window, message, wparam, lparam);
        match message {
            WM_CREATE => {
                let create_struct: &CREATESTRUCTW = unsafe { transmute(lparam) };
                unsafe {
                    SetWindowLongPtrW(window, GWLP_USERDATA, create_struct.lpCreateParams as _)
                };
                LRESULT(0)
            }
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                LRESULT(0)
            }
            WM_ERASEBKGND => {
                // Auto fill background with window theme color
                let user_data = unsafe { GetWindowLongPtrW(window, GWLP_USERDATA) };
                let sample = std::ptr::NonNull::<WindowOptions>::new(user_data as _);
                let (theme, background) =
                    sample.map_or((Theme::default(), Background::default()), |s| {
                        (
                            unsafe { s.as_ref() }.theme.clone(),
                            unsafe { s.as_ref() }.background.clone(),
                        )
                    });

                let mut rect = RECT::default();
                unsafe { GetClientRect(window, &mut rect).unwrap() };

                let color = match theme {
                    Theme::Light => background.light(),
                    Theme::Dark => background.dark(),
                    Theme::Auto => background.color(is_dark_mode().into()),
                };

                let brush = unsafe { CreateSolidBrush(COLORREF(swap_rb(color))) };
                unsafe { FillRect(HDC(wparam.0 as isize), &rect, brush) };
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
        }
    })
}
