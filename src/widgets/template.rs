//! 这里是一个控件的基本模板
//! 以留备用方便做新控件


use crate::traits::{Widget};
use crate::utils::uid::gen_uid;
use crate::utils::{WidgetUid};

pub struct TemplateControl {
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
        }
    }
}

// Defaults
impl Default for TemplateControl {
    fn default() -> Self {
        Self {
            pos: (0, 0),
            size: (0, 0),
            g_pos: (0, 0),
            need_update: true,
        }
    }
}

impl<D> Widget<D> for TemplateControl {
}
