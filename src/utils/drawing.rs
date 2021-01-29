//! 和绘图有关的常用代码都在这里

use tiny_skia::{Path, PathBuilder};

/// 生成一个给 DrawTarget 绘制的矩形路径
pub fn gen_rect_path(x: f32, y: f32, width: f32, height: f32) -> Path {
    let mut pt = PathBuilder::new();
    pt.push_rect(x, y, width, height);
    pt.finish().unwrap()
}

/// 生成一个给 DrawTarget 绘制的圆角矩形路径（圆角为贝塞尔曲线）
pub fn gen_round_rect_path(x: f32, y: f32, width: f32, height: f32, radius: f32) -> Path {
    let mut pt = PathBuilder::new();
    pt.move_to(x + radius, y);
    pt.line_to(x + width - radius, y);

    pt.quad_to(x + width, y, x + width, y + radius);
    pt.line_to(x + width, y + height - radius);
    pt.quad_to(x + width, y + height, x + width - radius, y + height);
    pt.line_to(x + radius, y + height);
    pt.quad_to(x, y + height, x, y + height - radius);
    pt.line_to(x, y + radius);
    pt.quad_to(x, y, x + radius, y);
    pt.close();
    pt.finish().unwrap()
}

// Calculates floor(a*b/255 + 0.5)
#[inline]
pub fn muldiv255(a: u32, b: u32) -> u32 {
    // The deriviation for this formula can be
    // found in "Three Wrongs Make a Right" by Jim Blinn.
    let tmp = a * b + 128;
    (tmp + (tmp >> 8)) >> 8
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// 确认文字如何对齐的枚举
pub enum TextAlign {
    /// 左对齐，这对于文本标签是默认的
    Left,
    /// 右对齐
    Right,
    /// 中心对齐
    Center,
}
