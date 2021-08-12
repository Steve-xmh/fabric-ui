pub struct HSV(u8, f32, f32);
/// RGBA
pub type Color = (u8, u8, u8, u8);

#[inline]
pub fn split_color(color: u32) -> Color {
    let r = (color >> 24) as u8;
    let g = ((color >> 16) & 0xFF) as u8;
    let b = ((color >> 8) & 0xFF) as u8;
    let a = (color & 0xFF) as u8;
    (r, g, b, a)
}

#[inline]
pub fn fix_alpha(a0: u8, a1: u8, f: f32) -> u8 {
    let a0 = (a0 as f32) / 255.0;
    let a1 = (a1 as f32) / 255.0;
    if a0 * a1 == 1.0 {
        0xFF
    } else if a0 == 0.0 {
        (a1 * f * 255.0) as u8
    } else if a1 == 0.0 {
        (a0 * f * 255.0) as u8
    } else if f == 0.0 {
        (a0 * a1 * 255.0) as u8
    } else {
        (a0 * a1 * f * 255.0) as u8
    }
}

#[inline]
pub fn concat_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | a as u32
}

#[inline]
pub fn fix_color(c0: u8, c1: u8, f: f32) -> u8 {
    let c0 = c0 as f32;
    let c1 = c1 as f32;
    (c0 + (c1 - c0) * f) as u8
}

#[inline]
pub fn premultiply_color(color: u32) -> u32 {
    let (r, g, b, a) = split_color(color);
    let a = a as f32;
    let r = (r as f32 * a / 255.) as u8;
    let g = (g as f32 * a / 255.) as u8;
    let b = (b as f32 * a / 255.) as u8;
    concat_color(r, g, b, a as u8)
}

/// Linear interpolation between c0 and c1.
pub fn mix_color(c0: u32, c1: u32, faction: f32) -> u32 {
    if faction <= 0.0 {
        c0
    } else if faction >= 1.0 {
        c1
    } else {
        let (r0, g0, b0, a0) = split_color(c0);
        let (r1, g1, b1, a1) = split_color(c1);
        ((fix_color(r0, r1, faction) as u32) << 24)
            | ((fix_color(g0, g1, faction) as u32) << 16)
            | ((fix_color(b0, b1, faction) as u32) << 8)
            | (fix_alpha(a0, a1, faction) as u32)
    }
}
