use tiny_skia::Pixmap;

use crate::DrawCtx;
use crate::traits::{Widget};
use crate::utils::uid::gen_uid;
use crate::utils::{WidgetUid};

pub struct ImageControl {
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
            pos: (0, 0),
            size: (0, 0),
            g_pos: (0, 0),
            need_update: true,
            img: Pixmap::new(1, 1).unwrap(),
        }
    }
}
impl<D> Widget<D> for ImageControl {
    fn update(&mut self, _data: &D) {
        
    }
    fn draw(&mut self, _ctx: &mut DrawCtx<'_>, _data: &D) {
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

}

impl<'a, 'b> Clone for ImageControl {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            pos: self.pos,
            g_pos: self.g_pos,
            need_update: true,
            img: self.img.clone(),
        }
    }
}
