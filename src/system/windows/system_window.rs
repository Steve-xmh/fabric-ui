use std::mem;
use std::ptr::null_mut;

use std::{ffi::c_void};

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

use crate::system::traits::{Fabric, SystemDrawableWindow};
use crate::traits::{w_str};
use crate::{system::enums::HitResult};

use super::fabric::WindowsFabric;
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
        PostMessageW(handle as _, WM_DESTROY, 0, 0);
    }
}

pub struct SystemWindow {
    hwnd: HWND,
    wc: WNDCLASSW,
    size: SIZE,
    pos_rect: RECT,
    ppt_src: POINT,
    fabric: WindowsFabric,
    blend_func: BLENDFUNCTION,
}

impl SystemWindow {
    /// 创建自带 Fabric 的窗口
    /// 渲染时需要注意：在 Windows 中位图的字节顺序是 BGRA，而 Tiny Skia 使用的是 RGBA
    pub fn new() -> Self {
        let real_width = 800;
        let real_height = 600;
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
        let fabric = WindowsFabric::new(hwnd_win, real_width as u32, real_height as u32);
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
            fabric,
            hwnd: hwnd_win,
            pos_rect,
        };
        unsafe {
            set_window_long(hwnd_win, &mut r as *mut Self as usize);
        }
        r
    }
}

impl SystemDrawableWindow for SystemWindow {
    fn hit_test(&mut self, _x: i32, _y: i32) -> HitResult {
        HitResult::Client
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.fabric.resize(width, height);
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

    fn query_event(&mut self, peek: bool) {
        let _evt = self.query_system_event(peek);
    }

    fn query_system_event(&mut self, peek: bool) {
        unsafe {
            set_window_long(self.hwnd, self as *mut Self as usize);
            let mut msg = mem::zeroed();
            let msgr = if peek {
                PeekMessageW(&mut msg, 0 as HWND, 0, 0, PM_REMOVE)
            } else {
                GetMessageW(&mut msg, 0 as HWND, 0, 0)
            };
            if msgr != 0 && msg.message != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    fn sync(&mut self) {
        unsafe {
            UpdateLayeredWindow(
                self.hwnd,
                null_mut(),
                null_mut(),
                &mut self.size,
                self.fabric.m_hdc,
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

    fn fabric(&mut self) -> &mut dyn super::super::traits::Fabric {
        &mut self.fabric
    }
}

impl Drop for SystemWindow {
    fn drop(&mut self) {
        unsafe {
            UnregisterClassW(self.wc.lpszClassName, self.wc.hInstance);
        }
    }
}
