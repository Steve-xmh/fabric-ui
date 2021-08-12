use std::time::SystemTime;



use crate::core::WidgetPod;
use crate::system::enums::HitResult;
use crate::traits::{Widget, TopWidget};

use crate::utils::uid::gen_uid;

use crate::utils::{WidgetUid};


#[derive(PartialEq, Debug, Clone, Copy)]
enum WindowTitleHovering {
    None,
    MinBtn,
    CloseBtn,
}

pub struct WindowControl<D> {
    _uid: WidgetUid,
    inner: WidgetPod<D>,
    start_time: SystemTime,
    mouse_down_time: SystemTime,
    btn_hovering: WindowTitleHovering,
    size: (u32, u32),
    destroyed: bool,
    title: String,
}

const RADIUS: f32 = 2.;
const SHADOW_SIZE: f32 = 16.;

impl<D> WindowControl<D> {
    pub fn new(inner: Box<dyn Widget<D>>) -> Self {
        
        Self {
            _uid: gen_uid(),
            title: String::new(),
            start_time: SystemTime::now(),
            mouse_down_time: SystemTime::now(),
            btn_hovering: WindowTitleHovering::None,
            inner: inner.into(),
            size: (800, 600),
            destroyed: false,
        }
    }

    pub fn with_width(mut self, v: u32) -> Self {
        self.size.0 = v;
        self
    }

    pub fn with_height(mut self, v: u32) -> Self {
        self.size.1 = v;
        self
    }

    pub fn with_title(mut self, v: String) -> Self {
        self.title = v;
        self
    }

    pub fn set_title(&mut self, v: String) {
        self.title = v;
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn show(&self) {}
}

impl<D> Widget<D> for WindowControl<D> {
    fn draw(&mut self, ctx: &mut crate::DrawCtx, _data: &D) {
        // F74C00
        ctx.pixmapmut.fill(tiny_skia::Color::from_rgba8(0x00, 0x4c, 0xf7, 0xAA));
        println!("Draw");
    }
}

impl<D> TopWidget<D> for WindowControl<D> {
    fn real_width(&self) -> u32 {
        self.size.0 + (SHADOW_SIZE * 2.) as u32
    }

    fn real_height(&self) -> u32 {
        self.size.1 + (SHADOW_SIZE * 2.) as u32
    }

    fn destroy(&mut self) {
        if self.destroyed {
            return;
        }
        self.destroyed = true;
        self.start_time = SystemTime::now();
    }

    fn hit_test(&mut self, _x: i32, _y: i32) -> HitResult {
        HitResult::Client
    }

    fn is_destroyed(&self) -> bool {
        self.destroyed && self.start_time.elapsed().unwrap().as_millis() > 300
    }
}
