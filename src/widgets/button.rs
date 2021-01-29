use tiny_skia::{FillRule, Paint, Shader, Stroke};

use crate::utils::drawing::gen_round_rect_path;
use crate::utils::theme::{get_theme_color, get_theme_color_skia_color};
use crate::utils::uid::gen_uid;
use crate::utils::{ControlUid, DrawTargetExt};
use crate::{
    traits::{Control, SubControl},
    utils::is_in_area,
};

use super::WindowEvent;
use super::{events::UserEvent, EventResult};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ButtonStatus {
    Normal,
    Hovering,
    Active,
    // Blocked,
}

pub struct ButtonControl {
    _uid: ControlUid,
    pos: (i32, i32),
    g_pos: (i32, i32),
    size: (u32, u32),
    text: String,
    text_size: f32,
    status: ButtonStatus,
    need_update: bool,
    click_callback: Box<dyn Fn() -> ()>,
}

impl ButtonControl {
    pub fn new(x: i32, y: i32, w: u32, h: u32, text: String) -> Self {
        ButtonControl {
            _uid: gen_uid(),
            pos: (x, y),
            size: (w, h),
            g_pos: (x as i32, y as i32),
            text,
            text_size: 14.,
            status: ButtonStatus::Normal,
            need_update: true,
            click_callback: Box::new(|| {}),
        }
    }

    pub fn on_click<F: Fn() -> () + 'static>(mut self, v: F) -> Self {
        self.click_callback = Box::new(v);
        self
    }
}
impl Clone for ButtonControl {
    fn clone(&self) -> Self {
        Self {
            _uid: gen_uid(),
            size: self.size,
            pos: self.pos,
            g_pos: self.g_pos,
            status: self.status,
            text_size: self.text_size,
            text: self.text.clone(),
            need_update: true,
            click_callback: Box::new(|| {}),
        }
    }
}
impl SubControl for ButtonControl {}
impl Control for ButtonControl {
    fn update(&mut self) -> bool {
        self.need_update
    }
    fn set_g_pos(&mut self, pos: (i32, i32)) {
        self.g_pos = pos;
    }
    fn draw(&mut self, x: f32, y: f32, f: &mut DrawTargetExt) {
        let draw_x = x;
        let draw_y = y;
        let width = self.size.0 as f32;
        let height = self.size.1 as f32;
        let p = gen_round_rect_path(draw_x, draw_y, width, height, 2.);
        let mut canvas = f.canvas();
        canvas.stroke_path(
            &p,
            &Paint {
                anti_alias: true,
                shader: Shader::SolidColor(get_theme_color_skia_color()),
                ..Default::default()
            },
            &Stroke {
                width: 2.,
                line_cap: tiny_skia::LineCap::Round,
                line_join: tiny_skia::LineJoin::Round,
                ..Default::default()
            },
        );
        match self.status {
            ButtonStatus::Hovering => {
                canvas.fill_path(
                    &p,
                    &Paint {
                        shader: Shader::SolidColor({
                            let mut c = get_theme_color_skia_color();
                            c.apply_opacity(0.1);
                            c
                        }),
                        ..Default::default()
                    },
                    FillRule::default(),
                );
            }
            ButtonStatus::Active => {
                canvas.fill_path(
                    &p,
                    &Paint {
                        shader: Shader::SolidColor({
                            let mut c = get_theme_color_skia_color();
                            c.apply_opacity(0.2);
                            c
                        }),
                        ..Default::default()
                    },
                    FillRule::default(),
                );
            }
            _ => {}
        }
        f.render_text(
            draw_x + (width - f.mesures_text_bold(&self.text, self.text_size)) / 2.,
            draw_y + (height - self.text_size) / 2. + self.text_size,
            &self.text,
            14.,
            get_theme_color(),
        );
        self.need_update = false;
    }

    #[inline]
    fn width(&self) -> u32 {
        self.size.0
    }
    #[inline]
    fn height(&self) -> u32 {
        self.size.1
    }

    #[inline]
    fn set_width(&mut self, v: u32) {
        self.size.0 = v;
    }

    #[inline]
    fn set_height(&mut self, v: u32) {
        self.size.1 = v;
    }
    #[inline]
    fn pos_x(&self) -> i32 {
        self.pos.0
    }
    #[inline]
    fn pos_y(&self) -> i32 {
        self.pos.1
    }

    #[inline]
    fn set_pos_x(&mut self, v: i32) {
        self.pos.0 = v;
    }

    #[inline]
    fn set_pos_y(&mut self, v: i32) {
        self.pos.1 = v;
    }

    fn emit(&mut self, evt: WindowEvent, user_evts: &mut Vec<UserEvent>) -> EventResult {
        let last_status = self.status;
        let result = match evt {
            WindowEvent::MouseMove(x, y) => {
                // println!("M {} {} GP {} {}", x, y, self.g_pos.0, self.g_pos.1);
                self.status = if is_in_area(
                    x as i32,
                    y as i32,
                    self.g_pos.0,
                    self.g_pos.1,
                    self.size.0 as i32,
                    self.size.1 as i32,
                ) {
                    ButtonStatus::Hovering
                } else {
                    ButtonStatus::Normal
                };
                super::EventResult::NoBubbling
            }
            WindowEvent::MouseDown(x, y) => {
                // println!("M {} {} GP {} {}", x, y, self.g_pos.0, self.g_pos.1);
                if is_in_area(
                    x as i32,
                    y as i32,
                    self.g_pos.0,
                    self.g_pos.1,
                    self.size.0 as i32,
                    self.size.1 as i32,
                ) {
                    self.status = ButtonStatus::Active;
                };
                super::EventResult::NoBubbling
            }
            WindowEvent::MouseUp(x, y) => {
                // println!("M {} {} GP {} {}", x, y, self.g_pos.0, self.g_pos.1);
                self.status = if is_in_area(
                    x as i32,
                    y as i32,
                    self.g_pos.0,
                    self.g_pos.1,
                    self.size.0 as i32,
                    self.size.1 as i32,
                ) {
                    ButtonStatus::Hovering
                } else {
                    ButtonStatus::Normal
                };
                super::EventResult::NoBubbling
            }
            WindowEvent::MousePress(x, y) => {
                // println!("MP {} {} GP {} {}", x, y, self.g_pos.0, self.g_pos.1);
                let is_in_area = is_in_area(
                    x as i32,
                    y as i32,
                    self.g_pos.0,
                    self.g_pos.1,
                    self.size.0 as i32,
                    self.size.1 as i32,
                );
                // println!("{}", is_in_area);
                if is_in_area {
                    user_evts.push(UserEvent::ControlClicked(self.uid()));
                    super::EventResult::NoBubbling
                } else {
                    super::EventResult::Bubble
                }
            }
            _ => super::EventResult::Bubble,
        };
        if last_status != self.status {
            self.need_update = true;
        }
        result
    }

    #[inline]
    fn uid(&self) -> crate::utils::ControlUid {
        self._uid
    }
}
