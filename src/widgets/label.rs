use crate::traits::{Control, SubControl};
use crate::utils::drawing::TextAlign;
use crate::utils::uid::gen_uid;
use crate::utils::{ControlUid, DrawTargetExt};

use super::{EventResult, UserEvent, WindowEvent};

pub struct TextLabelControl {
    _uid: ControlUid,
    pos: (i32, i32),
    size: (u32, u32),
    text: String,
    text_color: u32,
    text_size: f32,
    text_align: TextAlign,
    need_update: bool,
}

impl Default for TextLabelControl {
    fn default() -> Self {
        Self {
            _uid: gen_uid(),
            pos: (0, 0),
            size: (100, 50),
            text: "".to_string(),
            text_size: 13.,
            text_align: TextAlign::Left,
            text_color: 0x202020FF,
            need_update: true,
        }
    }
}

impl TextLabelControl {
    pub fn new(x: i32, y: i32, w: u32, h: u32, text: String) -> Self {
        Self {
            pos: (x, y),
            size: (w, h),
            text,
            ..Default::default()
        }
    }

    #[inline]
    pub fn set_color(&mut self, color: u32) {
        self.text_color = color;
    }

    #[inline]
    pub fn set_text_size(&mut self, size: f32) {
        self.text_size = size;
    }

    #[inline]
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.text_align = align;
    }
}
impl Clone for TextLabelControl {
    fn clone(&self) -> Self {
        Self {
            _uid: gen_uid(),
            size: self.size,
            pos: self.pos,
            text_size: self.text_size,
            text_color: self.text_color,
            text_align: self.text_align,
            text: self.text.clone(),
            need_update: true,
        }
    }
}

impl SubControl for TextLabelControl {}
impl Control for TextLabelControl {
    fn update(&mut self) -> bool {
        self.need_update
    }
    fn set_g_pos(&mut self, _pos: (i32, i32)) {}
    fn draw(&mut self, x: f32, y: f32, f: &mut DrawTargetExt) {
        let draw_x = x as f32;
        let draw_y = y as f32;
        let width = self.size.0 as f32;
        let height = self.size.1 as f32;
        // let p = gen_round_rect_path(draw_x, draw_y, width, height, 2.);
        match self.text_align {
            TextAlign::Left => {
                f.render_text(
                    draw_x,
                    draw_y + (height - self.text_size) / 2. + self.text_size,
                    &self.text,
                    self.text_size,
                    self.text_color,
                );
            }
            TextAlign::Center => {
                f.render_text(
                    draw_x + (width - f.mesures_text(&self.text, self.text_size)) / 2.,
                    draw_y + (height - self.text_size) / 2. + self.text_size,
                    &self.text,
                    self.text_size,
                    self.text_color,
                );
            }
            TextAlign::Right => {
                f.render_text(
                    draw_x + (width - f.mesures_text(&self.text, self.text_size)),
                    draw_y + (height - self.text_size) / 2. + self.text_size,
                    &self.text,
                    self.text_size,
                    self.text_color,
                );
            }
        }
        self.need_update = false;
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

    fn emit(&mut self, _evt: WindowEvent, _user_evts: &mut Vec<UserEvent>) -> EventResult {
        EventResult::Bubble
    }

    #[inline]
    fn uid(&self) -> crate::utils::ControlUid {
        self._uid
    }
}
