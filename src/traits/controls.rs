use crate::{DrawCtx, EventCtx};
use crate::r#box::AreaBox;
use crate::system::enums::HitResult;
use crate::utils::{ControlUid};

pub trait Control<D> {
    fn uid(&self) -> ControlUid;
    fn event(&mut self, _ctx: &mut EventCtx, _data: &mut D) {}
    fn update(&mut self, _data: &D) {}
    fn draw(&mut self, _ctx: &mut DrawCtx, _data: &D) {}
    fn layout(&self, _max_box: AreaBox) -> AreaBox {
        AreaBox::ZERO
    }
}

impl<D> PartialEq for dyn Control<D> {
    fn eq(&self, other: &dyn Control<D>) -> bool {
        self.uid() == other.uid()
    }
}

pub trait TopControl<D>: Control<D> {
    fn destroy(&mut self);
    fn is_destroyed(&self) -> bool;
    fn hit_test(&mut self, _x: i32, _y: i32) -> HitResult {
        HitResult::Client
    }
    fn real_width(&self) -> u32;
    fn real_height(&self) -> u32;
}
