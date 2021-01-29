//! 这里是一个控件的基本模板
//! 以留备用方便做新控件

use tiny_skia::{Paint, Shader};

use crate::traits::{Control, SubControl};
use crate::utils::drawing::gen_rect_path;
use crate::utils::theme::get_theme_color_skia_color;
use crate::utils::{is_in_area, uid::gen_uid};
use crate::utils::{ControlUid, DrawTargetExt};

use super::{EventResult, UserEvent, WindowEvent};

pub struct InputControl {
    _uid: ControlUid,
    pos: (i32, i32),
    g_pos: (i32, i32),
    size: (u32, u32),
    value: String,
    need_update: bool,
}
impl InputControl {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        InputControl {
            pos: (x, y),
            size: (w, h),
            g_pos: (x as i32, y as i32),
            ..Default::default()
        }
    }

    #[inline]
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    #[inline]
    pub fn value(&mut self) -> &String {
        &self.value
    }
}
// 克隆
impl Clone for InputControl {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            pos: self.pos,
            g_pos: self.g_pos,
            need_update: true,
            value: self.value.clone(),
            _uid: gen_uid(),
        }
    }
}

// 默认值
impl Default for InputControl {
    fn default() -> Self {
        Self {
            _uid: gen_uid(),
            pos: (0, 0),
            size: (0, 0),
            g_pos: (0, 0),
            value: String::with_capacity(256),
            need_update: true,
        }
    }
}
impl Control for InputControl {
    fn update(&mut self) -> bool {
        self.need_update
    }
    fn draw(&mut self, x: f32, y: f32, f: &mut DrawTargetExt) {
        let _draw_x = x as f32;
        let _draw_y = y as f32;
        let _width = self.size.0 as f32;
        let _height = self.size.1 as f32;
        let mut canvas = f.canvas();
        // 绘图在这里
        canvas.stroke_path(
            &gen_rect_path(x, y, _width, _height),
            &Paint {
                anti_alias: true,
                shader: Shader::SolidColor(get_theme_color_skia_color()),
                ..Default::default()
            },
            &Default::default(),
        );
        let text_len = f.mesures_text(&self.value, 13.);
        f.render_text(
            x + (_width - text_len) / 2.,
            y + (_height - 6.5),
            &self.value,
            13.,
            0x000000FF,
        );

        self.need_update = false;
    }

    fn emit(&mut self, evt: WindowEvent, user_evts: &mut Vec<UserEvent>) -> EventResult {
        match evt {
            WindowEvent::CharInput(u, c) => {
                println!("CI {} {} {}", u, self.uid(), c);
                if u == self.uid() {
                    if c == '\x08' {
                        self.value.pop();
                    } else {
                        self.value.push(c);
                    }
                    self.need_update = true;
                    super::EventResult::NoBubbling
                } else {
                    super::EventResult::Bubble
                }
            }
            WindowEvent::MousePress(x, y) => {
                // println!("MP {} {} GP {} {}", x, y, self.g_pos.0, self.g_pos.1);
                let is_in_area = is_in_area(
                    x as i32,
                    y as i32,
                    self.g_pos.0,
                    self.g_pos.1,
                    self.size.0 as i32,
                    self.size.1 as i32,
                );
                // println!("{}", is_in_area);
                if is_in_area {
                    user_evts.push(UserEvent::ControlClicked(self.uid()));
                    super::EventResult::NoBubbling
                } else {
                    super::EventResult::Bubble
                }
            }
            _ => super::EventResult::Bubble,
        }
    }

    fn set_g_pos(&mut self, pos: (i32, i32)) {
        self.g_pos = pos;
    }

    #[inline]
    fn width(&self) -> u32 {
        self.size.0
    }
    #[inline]
    fn height(&self) -> u32 {
        self.size.1
    }

    #[inline]
    fn set_width(&mut self, v: u32) {
        self.size.0 = v;
    }

    #[inline]
    fn set_height(&mut self, v: u32) {
        self.size.1 = v;
    }
    #[inline]
    fn pos_x(&self) -> i32 {
        self.pos.0
    }
    #[inline]
    fn pos_y(&self) -> i32 {
        self.pos.1
    }

    #[inline]
    fn set_pos_x(&mut self, v: i32) {
        self.pos.0 = v;
    }

    #[inline]
    fn set_pos_y(&mut self, v: i32) {
        self.pos.1 = v;
    }

    fn uid(&self) -> ControlUid {
        self._uid
    }
}

impl SubControl for InputControl {}
