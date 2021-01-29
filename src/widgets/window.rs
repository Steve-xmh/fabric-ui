use std::time::SystemTime;

use tiny_skia::{Color, Paint, PathBuilder, PremultipliedColorU8, Rect, Shader, Stroke};

use crate::system::enums::HitResult;
use crate::utils::theme::get_theme_color_skia_color;
use crate::utils::uid::gen_uid;
use crate::utils::{drawing::gen_round_rect_path, spring::Spring2D};
use crate::utils::{ControlUid, DrawTargetExt};
use crate::{
    traits::{Container, Control, SubControl, TopControl},
    utils::is_in_area,
};

use super::events::WindowEvent;
use super::events::{EventResult, UserEvent};

#[derive(PartialEq, Debug, Clone, Copy)]
enum WindowTitleHovering {
    None,
    MinBtn,
    CloseBtn,
}

pub struct WindowControl {
    _uid: ControlUid,
    need_update: bool,
    draw_size_spring: Spring2D,
    children: Vec<Box<dyn Control>>,
    start_time: SystemTime,
    mouse_down_time: SystemTime,
    btn_hovering: WindowTitleHovering,
    pos: (i32, i32),
    g_pos: (i32, i32),
    size: (u32, u32),
    current_size: (u32, u32),
    destroyed: bool,
    title: String,
}

const RADIUS: f32 = 2.;
const SHADOW_SIZE: f32 = 16.;

impl WindowControl {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut r = Self {
            _uid: gen_uid(),
            need_update: true,
            title: title.to_string(),
            start_time: SystemTime::now(),
            mouse_down_time: SystemTime::now(),
            btn_hovering: WindowTitleHovering::None,
            children: Vec::new(),
            draw_size_spring: Spring2D::new((width as f32, height as f32)),
            pos: (0, 0),
            g_pos: (0, 0),
            size: (width, height),
            destroyed: false,
            current_size: (width, height),
        };
        r.draw_size_spring.set_damper(0.8);
        r
    }

    pub fn set_title(&mut self, v: String) {
        self.title = v.clone();
        self.need_update = true;
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn show(&self) {}

    #[inline]
    fn resize_fabric_to_fit(&mut self) {}
}

impl Container for WindowControl {
    /// 更新自身及子控件的绝对坐标，用于鼠标碰撞检测
    fn update_positions(&mut self) {
        for child in self.children.iter_mut() {
            child.set_g_pos((
                child.pos_x() as i32 + (SHADOW_SIZE as i32),
                (child.pos_y() as i32) + (SHADOW_SIZE as i32) + 32,
            ));
        }
    }

    fn add_child<T: 'static + SubControl>(&mut self, child: T) {
        self.children.push(Box::new(child));
        self.update_positions();
    }

    fn remove_child<T: 'static + SubControl>(&mut self, _child: T) {
        todo!()
    }
}

impl Control for WindowControl {
    fn update(&mut self) -> bool {
        let mut r = self.need_update;
        if !self.draw_size_spring.arrived() {
            r = true;
        } else if {
            let p = self.draw_size_spring.position_rounded();
            self.current_size = (p.0 as u32, p.1 as u32);
            self.width() != self.current_size.0 || self.height() != self.current_size.1
        } {
            self.resize_fabric_to_fit();
            r = true;
        }
        if self.start_time.elapsed().unwrap().as_millis() < 300 {
            r = true;
        }
        for child in self.children.iter_mut() {
            r = r || child.update();
        }
        r
    }
    fn set_g_pos(&mut self, _pos: (i32, i32)) {}

    fn draw(&mut self, _x: f32, _y: f32, fabric: &mut DrawTargetExt) {
        // let st = SystemTime::now();
        let (width, height) = self.draw_size_spring.position_rounded();
        for p in fabric.pixmap.pixels_mut().iter_mut() {
            *p = PremultipliedColorU8::from_rgba(0, 0, 0, 0).unwrap();
        }
        {
            let mut canvas = fabric.canvas();
            // Draw rounded rect
            let p = gen_round_rect_path(SHADOW_SIZE, SHADOW_SIZE, width, height, RADIUS);
            for i in 0..SHADOW_SIZE as i32 {
                canvas.stroke_path(
                    &p,
                    &Paint {
                        anti_alias: true,
                        shader: Shader::SolidColor(Color::from_rgba8(0, 0, 0, 4)),
                        ..Default::default()
                    },
                    &Stroke {
                        width: i as f32,
                        line_cap: tiny_skia::LineCap::Round,
                        line_join: tiny_skia::LineJoin::Round,
                        miter_limit: 2.,
                        ..Default::default()
                    },
                );
            }
            canvas.fill_path(
                &p,
                &Paint {
                    anti_alias: true,
                    shader: Shader::SolidColor(Color::WHITE),
                    ..Default::default()
                },
                Default::default(),
            );
            canvas.set_clip_path(&p, Default::default(), true);
            canvas.fill_rect(
                Rect::from_xywh(SHADOW_SIZE, SHADOW_SIZE, width, 32.).unwrap(),
                &Paint {
                    anti_alias: true,
                    shader: Shader::SolidColor(get_theme_color_skia_color()),
                    ..Default::default()
                },
            );
            // Close Button
            if self.btn_hovering == WindowTitleHovering::CloseBtn {
                canvas.fill_rect(
                    Rect::from_xywh(SHADOW_SIZE + width as f32 - 32., SHADOW_SIZE, 32., 32.)
                        .unwrap(),
                    &Paint {
                        anti_alias: true,
                        shader: Shader::SolidColor(Color::from_rgba8(0xff, 0, 0, 0xff)),
                        ..Default::default()
                    },
                );
            }
            let mut p = PathBuilder::new();
            p.move_to(SHADOW_SIZE + width as f32 - 22., SHADOW_SIZE + 10.);
            p.line_to(SHADOW_SIZE + width as f32 - 10., SHADOW_SIZE + 22.);
            p.move_to(SHADOW_SIZE + width as f32 - 10., SHADOW_SIZE + 10.);
            p.line_to(SHADOW_SIZE + width as f32 - 22., SHADOW_SIZE + 22.);
            let p = p.finish().unwrap();
            canvas.stroke_path(
                &p,
                &Paint {
                    anti_alias: true,
                    shader: Shader::SolidColor(Color::WHITE),
                    ..Default::default()
                },
                &Stroke {
                    width: 2.,
                    ..Default::default()
                },
            );
            // Minisize Button
            if self.btn_hovering == WindowTitleHovering::MinBtn {
                canvas.fill_rect(
                    Rect::from_xywh(SHADOW_SIZE + width as f32 - 64., SHADOW_SIZE, 32., 32.)
                        .unwrap(),
                    &Paint {
                        anti_alias: true,
                        shader: Shader::SolidColor(Color::from_rgba8(0, 0, 0, 63)),
                        ..Default::default()
                    },
                );
            }
            let mut p = PathBuilder::new();
            p.move_to(SHADOW_SIZE + width as f32 - 54., SHADOW_SIZE + 16.);
            p.line_to(SHADOW_SIZE + width as f32 - 44., SHADOW_SIZE + 16.);
            let p = p.finish().unwrap();
            canvas.stroke_path(
                &p,
                &Paint {
                    anti_alias: true,
                    shader: Shader::SolidColor(Color::WHITE),
                    ..Default::default()
                },
                &Stroke {
                    width: 2.,
                    ..Default::default()
                },
            );
        }
        fabric.render_text_bold(
            SHADOW_SIZE + 10.,
            SHADOW_SIZE + 26.,
            &self.title,
            16.,
            0xFFFFFFFF,
        );
        for child in self.children.iter_mut() {
            child.draw(
                (SHADOW_SIZE) + child.pos_x() as f32,
                (SHADOW_SIZE + 32.) + child.pos_y() as f32,
                fabric,
            );
        }
        let mut canvas = fabric.canvas();
        canvas.reset_clip();
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
        let prev = self.width();
        self.need_update = self.need_update || prev != v;
        self.size.0 = v;
        if prev < self.size.0 {
            self.resize_fabric_to_fit();
        }
        self.draw_size_spring
            .set_target((self.size.0 as f32, self.size.1 as f32));
        self.need_update = self.need_update || !self.draw_size_spring.arrived();
    }

    #[inline]
    fn set_height(&mut self, v: u32) {
        let prev = self.height();
        self.need_update = self.need_update || prev != v;
        self.size.1 = v;
        if prev < self.size.1 {
            self.resize_fabric_to_fit();
        }
        self.draw_size_spring
            .set_target((self.size.0 as f32, self.size.1 as f32));
        self.need_update = self.need_update || !self.draw_size_spring.arrived();
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
        let mut children_result = EventResult::Bubble;
        for child in self.children.iter_mut() {
            if child.emit(evt, user_evts) == EventResult::NoBubbling {
                children_result = EventResult::NoBubbling;
            }
        }
        if children_result == EventResult::Bubble {
            match evt {
                WindowEvent::MousePress(x, y) => {
                    let x = x as i32;
                    let y = y as i32;
                    let gx = self.g_pos.0 + SHADOW_SIZE as i32;
                    let gy = self.g_pos.1 + SHADOW_SIZE as i32;
                    if is_in_area(x, y, gx + self.size.0 as i32 - 32, gy, 32, 32) {
                        self.destroy();
                        user_evts.push(UserEvent::Quit);
                    } else if is_in_area(
                        x,
                        y,
                        SHADOW_SIZE as i32 + 284,
                        32,
                        16,
                        self.height() as i32,
                    ) {
                        self.set_width(if self.width() == 750 { 300 } else { 750 });
                        user_evts.insert(
                            0,
                            UserEvent::WindowResize(self.real_width(), self.real_height()),
                        );
                    }
                    EventResult::NoBubbling
                }
                WindowEvent::MouseDown(_, _) => {
                    self.mouse_down_time = SystemTime::now();
                    EventResult::Bubble
                }
                WindowEvent::MouseUp(x, y) => {
                    if self.mouse_down_time.elapsed().unwrap().as_millis() < 200 {
                        self.mouse_down_time = SystemTime::UNIX_EPOCH;
                        self.emit(WindowEvent::MousePress(x, y), user_evts);
                    }
                    EventResult::Bubble
                }
                WindowEvent::MouseMove(_, _) => {
                    self.update_positions();
                    EventResult::Bubble
                }
                _ => EventResult::Bubble,
            }
        } else {
            EventResult::Bubble
        }
    }

    fn uid(&self) -> ControlUid {
        self._uid
    }
}

impl TopControl for WindowControl {
    fn real_width(&self) -> u32 {
        self.width() + (SHADOW_SIZE * 2.) as u32
    }

    fn real_height(&self) -> u32 {
        self.height() + (SHADOW_SIZE * 2.) as u32
    }

    fn destroy(&mut self) {
        if self.destroyed {
            return;
        }
        self.destroyed = true;
        self.start_time = SystemTime::now();
        self.need_update = true;
    }

    fn hit_test(&mut self, x: i32, y: i32) -> HitResult {
        let test = (
            (self.g_pos.0 as i32) + (SHADOW_SIZE as i32),
            (self.g_pos.1 as i32) + (SHADOW_SIZE as i32),
        );
        let result = {
            if (x > test.0)
                && (x < test.0 + (self.width() as i32))
                && (y > test.1)
                && (y < test.1 + (SHADOW_SIZE as i32) + (self.height() as i32))
            {
                if y < (self.g_pos.1 as i32) + (SHADOW_SIZE as i32) + 32 {
                    if x > (test.0 + ((self.width() - 32) as i32)) {
                        HitResult::Close
                    } else if x > (test.0 + ((self.width() - 64) as i32)) {
                        HitResult::MinButton
                    } else {
                        HitResult::Caption
                    }
                } else {
                    HitResult::Client
                }
            } else {
                HitResult::None
            }
        };
        let h = match result {
            HitResult::Close => WindowTitleHovering::CloseBtn,
            HitResult::MinButton => WindowTitleHovering::MinBtn,
            _ => WindowTitleHovering::None,
        };
        if h != self.btn_hovering {
            self.btn_hovering = h;
            self.need_update = true;
        }
        return result;
    }

    fn need_update(&self) -> bool {
        self.need_update
    }

    fn cancel_update(&mut self) {
        self.need_update = false;
    }

    fn is_destroyed(&self) -> bool {
        self.destroyed && self.start_time.elapsed().unwrap().as_millis() > 300
    }
}
