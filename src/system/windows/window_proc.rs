use std::mem;

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::*;

use crate::widgets::WindowEvent;

use super::super::traits::SystemDrawableWindow;
use super::SystemWindow;

#[cfg(target_arch = "x86_64")]
unsafe fn get_window_long(window: HWND) -> usize {
    GetWindowLongPtrW(window, GWLP_USERDATA) as usize
}

#[cfg(target_arch = "x86")]
unsafe fn get_window_long(window: HWND) -> LONG {
    GetWindowLongW(window, GWLP_USERDATA) as usize
}

// User Events
// const WM_USER_NONE: UINT = WM_USER + 0;

pub unsafe extern "system" fn window_proc(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let win = get_window_long(h_wnd);
    if win != 0 {
        let win: &mut SystemWindow = mem::transmute(win);
        match msg {
            WM_CHAR => match std::char::from_u32(w_param as u32) {
                Some(c) => {
                    win.window_events.push(WindowEvent::CharInput(0, c));
                    return 0;
                }
                None => {}
            },
            WM_DESTROY => {
                DestroyWindow(h_wnd);
                return 0;
            }
            WM_MOUSEMOVE => {
                let x = (l_param as u32) & 0xFFFF;
                let y = (l_param as u32) >> 16;
                win.window_events.push(WindowEvent::MouseMove(x, y));
                return 0;
            }
            WM_LBUTTONDOWN => {
                let x = (l_param as u32) & 0xFFFF;
                let y = (l_param as u32) >> 16;
                win.window_events.push(WindowEvent::MouseDown(x, y));
                return 0;
            }
            WM_LBUTTONUP => {
                let x = (l_param as u32) & 0xFFFF;
                let y = (l_param as u32) >> 16;
                win.window_events.push(WindowEvent::MouseUp(x, y));
                return 0;
            }
            WM_KEYDOWN => {
                win.window_events.push(WindowEvent::KeyDown(w_param));
                return 0;
            }
            WM_KEYUP => {
                win.window_events.push(WindowEvent::KeyUp(w_param));
                return 0;
            }
            WM_MOVE => {
                let x = ((l_param as u32) & 0xFFFF) as i32;
                let y = ((l_param as u32) >> 16) as i32;
                win.set_pos_x(x);
                win.set_pos_y(y);
                win.window_events.push(WindowEvent::WindowMove(x, y));
            }
            WM_SIZE => {
                let w = (l_param as u32) & 0xFFFF;
                let h = (l_param as u32) >> 16;
                win.window_events.push(WindowEvent::WindowResize(w, h));
            }
            WM_NCLBUTTONDOWN => match w_param as isize {
                HTCLOSE => {
                    win.window_events.push(WindowEvent::CloseWindow);
                    return 0;
                }
                HTMINBUTTON => {
                    win.window_events.push(WindowEvent::HideWindow);
                    return 0;
                }
                _ => {}
            },
            WM_NCHITTEST => {
                let x = ((l_param as u32) & 0xFFFF) as i32;
                let y = ((l_param as u32) >> 16) as i32;
                return match win.hit_test(x - win.pos_x(), y - win.pos_y()) {
                    crate::system::enums::HitResult::None => HTNOWHERE,
                    crate::system::enums::HitResult::Client => HTCLIENT,
                    crate::system::enums::HitResult::Caption => HTCAPTION,
                    crate::system::enums::HitResult::MinButton => HTMINBUTTON,
                    crate::system::enums::HitResult::Close => HTCLOSE,
                };
            }
            _ => {}
        }
    }
    return DefWindowProcW(h_wnd, msg, w_param, l_param);
}
