

use crate::DrawCtx;


use crate::utils::uid::gen_uid;
use crate::utils::{ControlUid};
use crate::{
    traits::{Control},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ButtonStatus {
    Normal,
    Hovering,
    Active,
    // Blocked,
}

pub struct ButtonControl {
    _uid: ControlUid,
    pos: (i32, i32),
    g_pos: (i32, i32),
    size: (u32, u32),
    text: String,
    text_size: f32,
    status: ButtonStatus,
    need_update: bool,
    click_callback: Box<dyn Fn()>,
}

impl ButtonControl {
    pub fn new(x: i32, y: i32, w: u32, h: u32, text: String) -> Self {
        ButtonControl {
            _uid: gen_uid(),
            pos: (x, y),
            size: (w, h),
            g_pos: (x as i32, y as i32),
            text,
            text_size: 14.,
            status: ButtonStatus::Normal,
            need_update: true,
            click_callback: Box::new(|| {}),
        }
    }

    pub fn on_click<F: Fn() + 'static>(mut self, v: F) -> Self {
        self.click_callback = Box::new(v);
        self
    }
}
impl Clone for ButtonControl {
    fn clone(&self) -> Self {
        Self {
            _uid: gen_uid(),
            size: self.size,
            pos: self.pos,
            g_pos: self.g_pos,
            status: self.status,
            text_size: self.text_size,
            text: self.text.clone(),
            need_update: true,
            click_callback: Box::new(|| {}),
        }
    }
}

impl<D> Control<D> for ButtonControl {
    fn event(&mut self, _ctx: &mut crate::EventCtx, _data: &mut D) {
        
    }

    fn update(&mut self, _data: &D) {
        
    }
    
    fn draw(&mut self, _ctx: &mut DrawCtx<'_>, _data: &D) {

    }

    #[inline]
    fn uid(&self) -> crate::utils::ControlUid {
        self._uid
    }
}
