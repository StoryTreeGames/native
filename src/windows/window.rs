use std::ffi::c_void;
use std::mem::size_of;
use std::ops::DerefMut;
use std::sync::Mutex;

use windows::core::HSTRING;
use windows::Foundation::{EventRegistrationToken, TypedEventHandler};
use windows::Win32::Foundation::{BOOL, HANDLE, HMODULE, HWND, LPARAM, WPARAM};
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};
use windows::Win32::Graphics::Gdi::{
    GetDC, GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTOPRIMARY,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, CloseWindow, CreateWindowExW, GetWindowLongW, GetWindowPlacement, LoadCursorW,
    LoadImageW, RegisterClassW, SetWindowLongW, SetWindowPlacement, SetWindowPos, ShowWindow,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, GWL_STYLE, HICON, HWND_TOP, IDC_ARROW, IMAGE_ICON,
    LR_DEFAULTSIZE, LR_LOADFROMFILE, LR_LOADTRANSPARENT, LR_SHARED, SWP_FRAMECHANGED, SWP_NOMOVE,
    SWP_NOOWNERZORDER, SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE,
    SW_SHOWNORMAL, WINDOWPLACEMENT, WINDOW_EX_STYLE, WM_ERASEBKGND, WM_PAINT, WNDCLASSW,
    WS_OVERLAPPEDWINDOW,
};
use windows::UI::ViewManagement::UISettings;

use crate::e;
use crate::error::Error;
use crate::style::{Background, Theme};
use crate::window::{WindowBuilder, WindowContext, WindowOptions};

use super::{event::wnd_proc, is_dark_mode, IntoPCWSTR, UI_SETTINGS};

thread_local! {
    static WINDOWS: Mutex<Vec<Window>> = Mutex::new(Vec::new())
}

/// Set the fullscreen state of a window
pub fn toggle_fullscreen(id: isize) {
    WINDOWS.with(|windows| {
        let mut windows = windows.lock().unwrap();
        let windows = windows.deref_mut();
        if let Some(window) = windows.iter_mut().find(|w| w.handle.0 == id) {
            window.fullscreen();
        }
    })
}

macro_rules! boxed_unwrap {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(Error::from(e)),
        }
    };
}

#[derive(Default)]
pub struct Builder {
    options: WindowOptions,
}

impl WindowBuilder for Builder {
    fn new() -> Self {
        Builder {
            options: WindowOptions::default(),
        }
    }

    fn title(mut self, title: &'static str) -> Self {
        self.options.title = title;
        self
    }

    fn theme(mut self, theme: Theme) -> Self {
        self.options.theme = theme;
        self
    }

    fn background(mut self, background: Background) -> Self {
        self.options.background = background;
        self
    }

    fn icon(mut self, icon: &'static str) -> Self {
        if !icon.ends_with(".ico") {
            panic!("Icon must be an ico file");
        }

        self.options.icon = Some(icon);
        self
    }

    fn create(self) -> Result<isize, Error> {
        Window::create(self.options)
    }

    fn show(mut self) -> Result<isize, Error> {
        self.options.show = true;
        self.create()
    }
}

pub struct Window {
    handle: HWND,
    class: HSTRING,
    instance: HMODULE,
    options: WindowOptions,

    prev_style: Option<WINDOWPLACEMENT>,

    theme_cookie: Option<EventRegistrationToken>,
}

impl Window {
    pub fn options(&self) -> &WindowOptions {
        &self.options
    }

    pub fn set_handle(&mut self, handle: HWND) {
        self.handle = handle;
    }

    pub fn fullscreen(&mut self) {
        let style = unsafe { GetWindowLongW(self.handle, GWL_STYLE) };
        if self.prev_style.is_none() {
            self.prev_style = Some(WINDOWPLACEMENT::default());
            let mut mi = MONITORINFO {
                cbSize: size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            if unsafe { GetWindowPlacement(self.handle, self.prev_style.as_mut().unwrap()) }.is_ok()
                && bool::from(unsafe {
                GetMonitorInfoW(
                    MonitorFromWindow(self.handle, MONITOR_DEFAULTTOPRIMARY),
                    &mut mi,
                )
            })
            {
                unsafe {
                    SetWindowLongW(
                        self.handle,
                        GWL_STYLE,
                        style & !(WS_OVERLAPPEDWINDOW.0 as i32),
                    );
                    if let Err(err) = SetWindowPos(
                        self.handle,
                        HWND_TOP,
                        mi.rcMonitor.left,
                        mi.rcMonitor.top,
                        mi.rcMonitor.right - mi.rcMonitor.left,
                        mi.rcMonitor.bottom - mi.rcMonitor.top,
                        SWP_NOOWNERZORDER | SWP_FRAMECHANGED,
                    ) {
                        eprintln!("{:?}", err);
                    }
                }
            }
        } else {
            unsafe {
                SetWindowLongW(self.handle, GWL_STYLE, style | WS_OVERLAPPEDWINDOW.0 as i32);
                let _ = SetWindowPlacement(self.handle, self.prev_style.as_ref().unwrap());
                let _ = SetWindowPos(
                    self.handle,
                    None,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOOWNERZORDER | SWP_FRAMECHANGED,
                );
            }
            self.prev_style = None;
        }
    }
}

impl WindowContext for Window {
    type Builder = Builder;

    fn create(options: WindowOptions) -> Result<isize, Error> {
        // Either this or thread local. Doing it this way gives the user the power to decide how to
        // handle multi-threading.
        WINDOWS.with(move |windows| {
            let mut windows = windows.lock().unwrap();
            windows.push(Window {
                handle: HWND(0),
                instance: HMODULE(0),
                class: HSTRING::from(format!("Window-StoryTree-{}", uuid::Uuid::new_v4())),
                prev_style: None,
                options,
                theme_cookie: None,
            });
            let window = windows.last_mut().unwrap();

            window.instance = e!(unsafe { GetModuleHandleW(None) })?;
            debug_assert!(window.instance.0 != 0);

            let wc = WNDCLASSW {
                hCursor: e!(unsafe { LoadCursorW(None, IDC_ARROW) })?,
                hInstance: window.instance.into(),
                lpszClassName: window.class.as_pcwstr(),
                hIcon: icon(window.options.icon.map(|i| HSTRING::from(i))),
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wnd_proc),
                ..Default::default()
            };

            let atom = unsafe { RegisterClassW(&wc) };
            debug_assert!(atom != 0);

            unsafe {
                window.set_handle(CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    window.class.as_pcwstr(),
                    HSTRING::from(window.options.title).as_pcwstr(),
                    WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    None,
                    None,
                    window.instance.clone(),
                    Some(&window.options as *const _ as *const _),
                ));
            }

            if window.handle.0 == 0 {
                return Err(Error::from(windows::core::Error::from_win32()));
            }
            window.set_theme(window.options.theme)?;
            if window.options.show {
                Window::show(window.handle.0);
            }
            Ok(window.handle.0)
        })
    }

    fn set_theme(&mut self, theme: Theme) -> Result<(), Error> {
        let state = match theme {
            Theme::Light => {
                if let Some(cookie) = self.theme_cookie {
                    boxed_unwrap!(UI_SETTINGS
                        .with(|ui_settings| { ui_settings.RemoveColorValuesChanged(cookie) }));
                }
                BOOL(0)
            }
            Theme::Dark => {
                if let Some(cookie) = self.theme_cookie {
                    boxed_unwrap!(UI_SETTINGS
                        .with(|ui_settings| { ui_settings.RemoveColorValuesChanged(cookie) }));
                }
                BOOL(1)
            }
            Theme::Auto => {
                let handle = self.handle;
                self.theme_cookie = Some(UI_SETTINGS.with(|ui_settings| {
                    ui_settings.ColorValuesChanged(&TypedEventHandler::new(
                        move |settings: &Option<UISettings>, _| {
                            if settings.is_some() {
                                unsafe {
                                    DwmSetWindowAttribute(
                                        handle,
                                        DWMWINDOWATTRIBUTE(20),
                                        &is_dark_mode() as *const _ as *const _,
                                        4,
                                    )
                                        .unwrap();
                                    CallWindowProcW(
                                        Some(wnd_proc),
                                        handle,
                                        WM_ERASEBKGND,
                                        WPARAM(GetDC(handle).0 as usize),
                                        LPARAM(0),
                                    );
                                    CallWindowProcW(
                                        Some(wnd_proc),
                                        handle,
                                        WM_PAINT,
                                        WPARAM(0),
                                        LPARAM(0),
                                    );
                                }
                            }
                            Ok(())
                        },
                    ))
                })?);

                is_dark_mode()
            }
        };

        unsafe {
            DwmSetWindowAttribute(
                self.handle,
                DWMWINDOWATTRIBUTE(20),
                &state as *const _ as *const c_void,
                4,
            )?;
        }

        Ok(())
    }

    fn builder() -> Box<Self::Builder> {
        Box::new(Builder::new())
    }

    /// Show the window
    fn show(id: isize) {
        unsafe {
            ShowWindow(HWND(id), SW_SHOWNORMAL);
        }
    }

    /// Hide the window
    fn hide(id: isize) {
        unsafe {
            ShowWindow(HWND(id), SW_HIDE);
        }
    }

    /// Minimize the window
    fn minimize(id: isize) {
        unsafe {
            ShowWindow(HWND(id), SW_MINIMIZE);
        }
    }

    /// Restore the window
    fn restore(id: isize) {
        unsafe {
            ShowWindow(HWND(id), SW_RESTORE);
        }
    }

    /// Maximize the window
    fn maximize(id: isize) {
        unsafe {
            ShowWindow(HWND(id), SW_MAXIMIZE);
        }
    }

    fn close(id: isize) -> Result<(), Error> {
        WINDOWS.with(|windows| {
            let mut windows = windows.lock().unwrap();
            if let Some(index) = windows.iter().position(|window| window.handle.0 == id) {
                windows.remove(index);
            }
        });
        Ok(e!(unsafe { CloseWindow(HWND(id)) })?)
    }
}

/// TODO: Automatic loading of other file formats?
pub fn icon(path: Option<HSTRING>) -> HICON {
    let result = HICON(path.map_or(0, |icon| {
        match unsafe {
            LoadImageW(
                None,
                icon.as_pcwstr(),
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTSIZE | LR_LOADFROMFILE | LR_SHARED | LR_LOADTRANSPARENT,
            )
        } {
            Ok(hicon) => hicon,
            Err(err) => {
                eprintln!("{}", Error::from(err));
                HANDLE(0)
            }
        }
            .0
    }));
    result
}
