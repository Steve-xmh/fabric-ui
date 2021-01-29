//! 这里是一个控件的基本模板
//! 以留备用方便做新控件

use crate::traits::{Control, SubControl};
use crate::utils::uid::gen_uid;
use crate::utils::{ControlUid, DrawTargetExt};

use super::{EventResult, UserEvent, WindowEvent};

pub struct TemplateControl {
    _uid: ControlUid,
    pos: (i32, i32),
    g_pos: (i32, i32),
    size: (u32, u32),
    need_update: bool,
}

impl TemplateControl {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        TemplateControl {
            pos: (x, y),
            size: (w, h),
            g_pos: (x as i32, y as i32),
            ..Default::default()
        }
    }
}

// Clone
impl Clone for TemplateControl {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            pos: self.pos,
            g_pos: self.g_pos,
            need_update: true,
            _uid: gen_uid(),
        }
    }
}

// Defaults
impl Default for TemplateControl {
    fn default() -> Self {
        Self {
            _uid: gen_uid(),
            pos: (0, 0),
            size: (0, 0),
            g_pos: (0, 0),
            need_update: true,
        }
    }
}

impl Control for TemplateControl {
    fn update(&mut self) -> bool {
        self.need_update
    }
    fn draw(&mut self, x: f32, y: f32, _f: &mut DrawTargetExt) {
        let _draw_x = x as f32;
        let _draw_y = y as f32;
        let _width = self.size.0 as f32;
        let _height = self.size.1 as f32;
        // Draw anything here
        self.need_update = false;
    }

    fn emit(&mut self, _evt: WindowEvent, _user_evts: &mut Vec<UserEvent>) -> EventResult {
        // Processing Events
        super::EventResult::Bubble
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

impl SubControl for TemplateControl {}
