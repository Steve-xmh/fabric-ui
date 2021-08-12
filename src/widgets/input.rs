//! 这里是一个控件的基本模板
//! 以留备用方便做新控件




use crate::traits::{Widget};


use crate::utils::{uid::gen_uid};
use crate::utils::{WidgetUid};

pub struct InputControl {
    _uid: WidgetUid,
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
impl<D> Widget<D> for InputControl {
}
