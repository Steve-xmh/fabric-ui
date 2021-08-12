//! Some context that can get useful data

use tiny_skia::PixmapMut;

use crate::{
    events::UserEvent,
    utils::{DrawTargetExt, WidgetUid},
};

pub struct EventCtx {
    event: UserEvent,
}

pub struct DrawCtx<'a> {
    pub pixmapmut: &'a mut PixmapMut<'a>,
    pub transform: tiny_skia::Transform,
    pub widget_size: (f32, f32),
}

trait CommonCtx {
    fn widget_id() -> WidgetUid;
    fn set_widget_id() -> WidgetUid;
    fn widget_size() -> (f32, f32);
}

impl<'a> DrawCtx<'a> {
    pub(crate) fn new(pixmapmut: &'a mut PixmapMut<'a>) -> Self {
        Self {
            pixmapmut,
            transform: tiny_skia::Transform::default(),
            widget_size: (f32::MAX, f32::MAX),
        }
    }

    pub fn fork(&'a mut self, mut f: impl FnMut(&mut DrawCtx) + 'static) {
        let mut new_ctx = DrawCtx {
            pixmapmut: &mut self.pixmapmut,
            transform: self.transform.to_owned(),
            widget_size: (self.widget_size.0, self.widget_size.1),
        };
        f(&mut new_ctx);
    }
}
