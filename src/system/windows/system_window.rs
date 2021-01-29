use std::mem;
use std::ptr::null_mut;
use std::time::Instant;
use std::{cell::RefCell, ffi::c_void};

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

use crate::system::traits::SystemDrawableWindow;
use crate::traits::{w_str, TopControl};
use crate::widgets::WindowEvent;
use crate::{system::enums::HitResult, utils::ControlUid, widgets::UserEvent};

use super::fabric::Fabric;
use super::window_proc::window_proc;

#[cfg(target_arch = "x86_64")]
unsafe fn set_window_long(window: HWND, data: usize) -> usize {
    SetWindowLongPtrW(window, GWLP_USERDATA, data as isize) as usize
}

#[cfg(target_arch = "x86")]
unsafe fn set_window_long(window: HWND, data: usize) -> usize {
    SetWindowLongW(window, GWLP_USERDATA, data as usize)
}

pub fn destroy_window(handle: usize) {
    unsafe {
        PostMessageW(handle as HWND, WM_DESTROY, 0, 0);
    }
}

pub struct SystemWindow<'a> {
    hwnd: HWND,
    wc: WNDCLASSW,
    size: SIZE,
    pos_rect: RECT,
    ppt_src: POINT,
    fabric: RefCell<Fabric>,
    render_time: Instant,
    blend_func: BLENDFUNCTION,
    focused_control: ControlUid,
    pub window_events: Vec<WindowEvent>,
    pub user_events: Vec<UserEvent>,
    top_control: &'a mut dyn TopControl,
}

impl<'a> SystemDrawableWindow<'a> for SystemWindow<'a> {
    fn new(top_control: &'a mut dyn TopControl) -> Self {
        let real_width = top_control.real_width() as i32;
        let real_height = top_control.real_height() as i32;
        let class_name = w_str("FabricWindowClass");
        let wc = unsafe {
            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_HREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: null_mut(),
                hIcon: LoadIconW(GetModuleHandleW(null_mut()), null_mut()),
                hCursor: null_mut(),
                hbrBackground: COLOR_BACKGROUND as HBRUSH,
                lpszMenuName: null_mut(),
                lpszClassName: class_name.as_ptr(),
            };
            RegisterClassW(&wc);
            wc
        };
        let hwnd_win = unsafe {
            CreateWindowExW(
                WS_EX_LAYERED,
                class_name.as_ptr(),
                class_name.as_ptr(),
                WS_POPUP | WS_VISIBLE,
                0,
                0,
                real_width,
                real_height,
                0 as HWND,
                0 as HMENU,
                0 as HINSTANCE,
                null_mut(),
            )
        };
        unsafe {
            let mut area: RECT = mem::zeroed();
            SystemParametersInfoW(
                SPI_GETWORKAREA,
                0,
                (&mut area as *mut RECT) as *mut c_void,
                0,
            );
            let pos_x = area.left + (area.right - area.left) / 2;
            let pos_y = area.top + (area.bottom - area.top) / 2;
            SetWindowPos(
                hwnd_win,
                HWND_TOP,
                pos_x - real_width / 2,
                pos_y - real_height / 2,
                real_width,
                real_height,
                0,
            );
        }
        let fabric = RefCell::new(Fabric::new(hwnd_win, real_width as u32, real_height as u32));
        let mut pos_rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        unsafe {
            GetWindowRect(hwnd_win, &mut pos_rect);
        }
        let mut r = Self {
            size: SIZE {
                cx: real_width,
                cy: real_height,
            },
            ppt_src: POINT { x: 0, y: 0 },
            blend_func: BLENDFUNCTION {
                BlendOp: AC_SRC_OVER,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA,
            },
            wc,
            render_time: Instant::now(),
            fabric,
            hwnd: hwnd_win,
            focused_control: top_control.uid(),
            top_control,
            pos_rect,
            window_events: Vec::with_capacity(16),
            user_events: Vec::with_capacity(16),
        };
        unsafe {
            set_window_long(hwnd_win, &mut r as *mut Self as usize);
        }
        r
    }

    fn hit_test(&mut self, x: i32, y: i32) -> HitResult {
        self.top_control.hit_test(x, y)
    }

    fn set_top_control(&mut self, top_control: &'a mut dyn TopControl) {
        self.top_control = top_control;
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.fabric.borrow_mut().resize(width, height);
    }

    fn pos_x(&self) -> i32 {
        self.pos_rect.left
    }

    fn set_pos_x(&mut self, x: i32) {
        self.pos_rect.left = x;
    }

    fn pos_y(&self) -> i32 {
        self.pos_rect.top
    }

    fn set_pos_y(&mut self, y: i32) {
        self.pos_rect.top = y;
    }

    fn size_x(&self) -> u32 {
        self.size.cx as u32
    }

    fn set_size_x(&mut self, w: u32) {
        self.size.cx = w as i32;
    }

    fn size_y(&self) -> u32 {
        self.size.cy as u32
    }

    fn set_size_y(&mut self, h: u32) {
        self.size.cy = h as i32;
    }

    fn query_event(&mut self, peek: bool) -> UserEvent {
        let evt = self.query_system_event(peek);
        match evt {
            WindowEvent::None => {}
            WindowEvent::CharInput(_, c) => {
                self.top_control.emit(
                    WindowEvent::CharInput(self.focused_control, c),
                    &mut self.user_events,
                );
            }
            other => {
                // println!("WE {:?}", other);
                self.top_control.emit(other, &mut self.user_events);
            }
        };
        if self.user_events.len() > 0 {
            match self.user_events.remove(0) {
                UserEvent::ControlClicked(c) => {
                    self.focused_control = c;
                    UserEvent::ControlClicked(c)
                }
                UserEvent::WindowResize(w, h) => {
                    self.resize(w, h);
                    UserEvent::None
                }
                other => other,
            }
        } else {
            UserEvent::None
        }
    }

    fn query_system_event(&mut self, peek: bool) -> WindowEvent {
        let need_update = self.top_control.update();
        if need_update && self.render_time.elapsed().as_millis() > 16 {
            self.size.cx = self.top_control.real_width() as i32;
            self.size.cy = self.top_control.real_height() as i32;
            self.top_control
                .draw(0., 0., &mut self.fabric.borrow_mut().dt);
            self.sync();
            self.render_time = Instant::now();
            self.top_control.cancel_update();
        }
        unsafe {
            set_window_long(self.hwnd, self as *mut Self as usize);
            let mut msg = mem::zeroed();
            let msgr = if peek || need_update || self.window_events.len() > 0 {
                PeekMessageW(&mut msg, 0 as HWND, 0, 0, PM_REMOVE)
            } else {
                GetMessageW(&mut msg, 0 as HWND, 0, 0)
            };
            if msgr != 0 {
                if msg.message != 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
        if self.window_events.len() > 0 {
            self.window_events.remove(0)
        } else {
            WindowEvent::None
        }
    }

    fn sync(&mut self) {
        self.fabric.borrow_mut().sync();
        unsafe {
            UpdateLayeredWindow(
                self.hwnd,
                null_mut(),
                null_mut(),
                &mut self.size,
                self.fabric.borrow().m_hdc,
                &mut self.ppt_src,
                0,
                &mut self.blend_func,
                ULW_ALPHA,
            );
        }
    }

    fn show(&mut self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
    }

    fn hide(&mut self) {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
    }

    fn raw_handle(&self) -> usize {
        self.hwnd as usize
    }
}

impl<'a> Drop for SystemWindow<'a> {
    fn drop(&mut self) {
        unsafe {
            UnregisterClassW(self.wc.lpszClassName, self.wc.hInstance);
        }
    }
}
