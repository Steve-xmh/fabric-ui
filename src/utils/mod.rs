pub mod blur;
pub mod color;
pub mod drawing;
pub mod math;
pub mod spring;
pub mod theme;
pub mod uid;

mod draw_target;
pub use draw_target::*;

#[inline]
pub fn is_in_area(px: i32, py: i32, x: i32, y: i32, w: i32, h: i32) -> bool {
    (px >= x) && (px <= x + w) && (py >= y) && (py <= y + h)
}

pub type ControlUid = usize;
