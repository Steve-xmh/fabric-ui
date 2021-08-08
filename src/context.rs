//! Some context that can get useful data

use crate::{events::UserEvent, utils::DrawTargetExt};

pub struct EventCtx {
    event: UserEvent
}

pub struct DrawCtx<'a> {
    draw_target: &'a mut DrawTargetExt,
    transform: tiny_skia::Transform,
}



impl<'a> DrawCtx<'a> {
    pub(crate) fn new(draw_target: &'a mut DrawTargetExt) -> Self {
        Self {
            draw_target,
            transform: tiny_skia::Transform::default(),
        }
    }

    pub fn fork(&mut self, mut f: impl FnMut(&mut DrawCtx) + 'static) {
        let mut new_ctx = DrawCtx {
            draw_target: self.draw_target,
            transform: self.transform.to_owned(),
        };
        f(&mut new_ctx);
    }
}
