use tiny_skia::Pixmap;

use crate::traits::{Control, SubControl};
use crate::utils::uid::gen_uid;
use crate::utils::{ControlUid, DrawTargetExt};

use super::{EventResult, UserEvent, WindowEvent};

pub struct ImageControl {
    _uid: ControlUid,
    pos: (i32, i32),
    g_pos: (i32, i32),
    size: (u32, u32),
    img: Pixmap,
    need_update: bool,
}
impl ImageControl {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        ImageControl {
            pos: (x, y),
            size: (w, h),
            g_pos: (x as i32, y as i32),
            ..Default::default()
        }
    }
    pub fn set_img(&mut self, img: Pixmap) {
        self.img = img;
    }
}
// 默认值
impl Default for ImageControl {
    fn default() -> Self {
        Self {
            _uid: gen_uid(),
            pos: (0, 0),
            size: (0, 0),
            g_pos: (0, 0),
            need_update: true,
            img: Pixmap::new(1, 1).unwrap(),
        }
    }
}
impl Control for ImageControl {
    fn update(&mut self) -> bool {
        self.need_update
    }
    fn draw(&mut self, _x: f32, _y: f32, _f: &mut DrawTargetExt) {
        /*
        let draw_x = x as f32;
        let draw_y = y as f32;
        let width = self.size.0 as f32;
        let height = self.size.1 as f32;
        // 绘图在这里
        f.draw_image_with_size_at(
            width,
            height,
            draw_x,
            draw_y,
            &self.img,
            &Default::default(),
        );
         */
        self.need_update = false;
    }

    fn emit(&mut self, _evt: WindowEvent, _user_evts: &mut Vec<UserEvent>) -> EventResult {
        // 处理事件
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

    #[inline]
    fn uid(&self) -> crate::utils::ControlUid {
        self._uid
    }
}

impl<'a, 'b> Clone for ImageControl {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            pos: self.pos,
            g_pos: self.g_pos,
            need_update: true,
            img: self.img.clone(),
            _uid: gen_uid(),
        }
    }
}
impl<'a, 'b> SubControl for ImageControl {}
