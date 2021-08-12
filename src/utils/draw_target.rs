use std::{convert::TryInto, mem::size_of, slice::from_raw_parts};

use font_kit::canvas::{Canvas as FKCanvas, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;
use pathfinder_geometry::rect::RectI;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use tiny_skia::*;
// use tiny_skia::{Canvas as SkiaCanvas, Color, Paint, Pixmap, PixmapPaint, PixmapRef, Shader};

use super::{drawing::muldiv255, math::round};

fn get_fonts() -> [FamilyName; 3] {
    [
        FamilyName::Title("Microsoft YaHei".to_string()),
        FamilyName::Title("Helvetica".to_string()),
        FamilyName::Title("Arial".to_string()),
    ]
}

fn get_font() -> Font {
    SystemSource::new()
        .select_best_match(&get_fonts(), &Properties::new())
        .unwrap()
        .load()
        .unwrap()
}

fn get_bold_font() -> Font {
    SystemSource::new()
        .select_best_match(&get_fonts(), Properties::new().weight(Weight::BOLD))
        .unwrap()
        .load()
        .unwrap()
}

#[inline]
fn union_recti(a: RectI, b: RectI) -> RectI {
    RectI::new(
        Vector2I::new(
            if a.origin_x() < b.origin_x() {
                a.origin_x()
            } else {
                b.origin_x()
            },
            if a.origin_y() < b.origin_y() {
                a.origin_y()
            } else {
                b.origin_y()
            },
        ),
        Vector2I::new(
            if a.width() < b.width() {
                b.width()
            } else {
                a.width()
            },
            if a.height() < b.height() {
                b.height()
            } else {
                a.height()
            },
        ),
    )
}

/// 扩展了 DrawTarget 对于一些绘制操作的支持，如文字，渲染等
pub struct DrawTargetExt {
    pub pixmap: Pixmap,
    font: Font,
    font_bold: Font,
}

pub enum FontType {
    Normal,
    Bold,
}

impl DrawTargetExt {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            pixmap: Pixmap::new(width as u32, height as u32).unwrap(),
            font: get_font(),
            font_bold: get_bold_font(),
        }
    }

    /// 绘制自定义字体文本到指定位置
    pub fn render_text_custom(
        &mut self,
        x: f32,
        y: f32,
        text: &str,
        size: f32,
        color: u32,
        font: FontType,
    ) {
        let font = match font {
            FontType::Normal => get_font(),
            FontType::Bold => get_bold_font(),
        };
        // contract
        let mut combined_bounds = RectI::new(Vector2I::zero(), Vector2I::zero());
        // let rounded_pos = (round(x), round(y));
        // let float_offset = (x - rounded_pos.0, y - rounded_pos.1);
        let mut ids = Vec::new();
        for c in text.chars() {
            let id = font.glyph_for_char(c).unwrap();
            let bounds = self
                .font
                .raster_bounds(
                    id,
                    size,
                    Transform2F::default(),
                    HintingOptions::None,
                    RasterizationOptions::GrayscaleAa,
                )
                .unwrap();
            combined_bounds = union_recti(combined_bounds, bounds);
            ids.push(id);
        }
        let mut canvas = FKCanvas::new(combined_bounds.size(), font_kit::canvas::Format::A8);
        let mut pos = Vector2F::new(round(x), round(y));
        pos -= combined_bounds.origin().to_f32();
        for id in ids {
            canvas.pixels = vec![0; canvas.pixels.len()];
            let bounds = font
                .raster_bounds(
                    id,
                    size,
                    Transform2F::default(),
                    HintingOptions::None,
                    RasterizationOptions::GrayscaleAa,
                )
                .unwrap();
            font.rasterize_glyph(
                &mut canvas,
                id,
                size,
                Transform2F::default().translate(Vector2F::new(0., -bounds.origin_y() as f32)),
                HintingOptions::None,
                RasterizationOptions::GrayscaleAa,
            )
            .unwrap();
            let data: Vec<u32> = canvas
                .pixels
                .iter()
                .map(|&pixel| {
                    let ac = (((color & 0xFF) as f32) / 255.) * (((pixel & 0xFF) as f32) / 255.);
                    let a = (ac * 255.) as u32 & 0xFF;
                    let r = muldiv255(a, (color >> 24) & 0xFF);
                    let g = muldiv255(a, (color >> 16) & 0xFF);
                    let b = muldiv255(a, (color >> 8) & 0xFF);
                    (a << 24) | (r << 16) | (g << 8) | b
                })
                .collect();
            // &data.align_to().1

            let font_pixmap = PixmapRef::from_bytes(
                unsafe {
                    from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<u32>())
                },
                canvas.size.x().try_into().unwrap(),
                canvas.size.y().try_into().unwrap(),
            )
            .unwrap();
            /*
            self.canvas().draw_pixmap(
                pos.x() as i32,
                pos.y() as i32 + (bounds.origin_y()) as i32 - size as i32,
                font_pixmap,
                &PixmapPaint::default(),
            );
            */

            let ad = font.advance(id).unwrap();
            pos += Vector2F::new(ad.x() * size / 2048., ad.y());
        }
    }

    pub fn mesures_text_custom(&self, text: &str, size: f32, font: FontType) -> f32 {
        let font = match font {
            FontType::Normal => &self.font,
            FontType::Bold => &self.font_bold,
        };
        let mut length = 0.;
        for c in text.chars() {
            length += font.advance(font.glyph_for_char(c).unwrap()).unwrap().x() / 2048. * size;
        }
        length
    }

    #[inline]
    pub fn mesures_text(&self, text: &str, size: f32) -> f32 {
        self.mesures_text_custom(text, size, FontType::Normal)
    }
    #[inline]
    pub fn mesures_text_bold(&self, text: &str, size: f32) -> f32 {
        self.mesures_text_custom(text, size, FontType::Bold)
    }

    #[inline]
    pub fn render_text_bold(&mut self, x: f32, y: f32, text: &str, size: f32, color: u32) {
        self.render_text_custom(x, y, text, size, color, FontType::Bold)
    }

    #[inline]
    pub fn render_text(&mut self, x: f32, y: f32, text: &str, size: f32, color: u32) {
        self.render_text_custom(x, y, text, size, color, FontType::Normal)
    }

    /// 根据当前图像的透明色绘制阴影，切勿重复调用！
    pub fn render_shadow(&mut self, _range: f32) {}
}
