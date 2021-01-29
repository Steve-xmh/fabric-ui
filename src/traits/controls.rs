use crate::utils::{ControlUid, DrawTargetExt};
use crate::widgets::{EventResult, WindowEvent};
use crate::{system::enums::HitResult, widgets::UserEvent};

pub trait Control {
    fn uid(&self) -> ControlUid;
    fn update(&mut self) -> bool;
    fn set_g_pos(&mut self, pos: (i32, i32));
    fn draw(&mut self, parent_x: f32, parent_y: f32, f: &mut DrawTargetExt);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_width(&mut self, v: u32);
    fn set_height(&mut self, v: u32);
    fn pos_x(&self) -> i32;
    fn pos_y(&self) -> i32;
    fn set_pos_x(&mut self, v: i32);
    fn set_pos_y(&mut self, v: i32);
    // Events
    fn emit(&mut self, evt: WindowEvent, user_evts: &mut Vec<UserEvent>) -> EventResult;
}

impl PartialEq for dyn Control {
    fn eq(&self, other: &dyn Control) -> bool {
        self.uid() == other.uid()
    }
}

pub trait SubControl: Control + Clone {}

pub trait Container {
    fn update_positions(&mut self);
    fn add_child<T: 'static + SubControl>(&mut self, child: T);
    fn remove_child<T: 'static + SubControl>(&mut self, child: T);
}

pub trait TopControl: Control {
    fn destroy(&mut self);
    fn is_destroyed(&self) -> bool;
    fn hit_test(&mut self, x: i32, y: i32) -> HitResult;
    fn need_update(&self) -> bool;
    fn cancel_update(&mut self);
    fn real_width(&self) -> u32;
    fn real_height(&self) -> u32;
}
