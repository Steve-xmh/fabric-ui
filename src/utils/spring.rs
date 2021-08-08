//! 一个一维弹簧算法类

use std::f32::consts::E;
use std::time::Instant;

use super::math::round;

pub struct Spring {
    start_time: Instant,
    pub damper: f32,
    pub velocity: f32,
    pub speed: f32,
    pub target: f32,
    pub position: f32,
}

impl Spring {
    pub fn new(start_position: f32) -> Self {
        Self {
            start_time: Instant::now(),
            position: start_position,
            damper: 1.,
            velocity: 0.,
            speed: 1.,
            target: start_position,
        }
    }

    fn position_velocity(&mut self) -> (f32, f32) {
        let x = self.start_time.elapsed().as_secs_f32();
        let c0 = self.position - self.target;
        
        if self.speed == 0. {
            (self.position, 0.)
        } else if self.damper < 1. {
            let c = (1. - self.damper.powi(2)).sqrt();
            let c1 = (self.velocity / self.speed + self.damper * c0) / c;
            let co = (c * self.speed * x).cos();
            let si = (c * self.speed * x).sin();
            let e = E.powf(self.damper * self.speed * x);
            (
                self.target + (c0 * co + c1 * si) / e,
                self.speed * ((c * c1 - self.damper * c0) * co - (c * c0 + self.damper * c1) * si)
                    / e,
            )
        } else {
            let c1 = self.velocity / self.speed + c0;
            let e = E.powf(self.speed * x);
            (
                self.target + (c0 + c1 * self.speed * x) / e,
                self.speed * (c1 - c0 - c1 * self.speed * x) / e,
            )
        }
    }

    pub fn arrived(&mut self) -> bool {
        let (pos, vel) = self.position_velocity();
        round(pos * 10.) == round(self.target * 10.) && round(vel * 10.) == 0.
    }

    pub fn position(&mut self) -> f32 {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        r.0
    }

    pub fn position_rounded(&mut self) -> f32 {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        round(r.0)
    }

    pub fn velocity(&mut self) -> f32 {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        r.1
    }

    pub fn acceleration(&self) -> f32 {
        let x = self.start_time.elapsed().as_secs_f32();
        let c0 = self.position - x;
        if self.speed == 0. {
            0.
        } else if self.damper < 1. {
            let c = (1. - self.damper.powi(2)).sqrt();
            let c1 = (self.velocity / self.speed + self.damper * c0) / c;
            self.speed.powi(2)
                * ((self.damper.powi(2) * c0 - 2. * c * self.damper * c1 - c.powi(2) * c0)
                    * (c * self.speed * x).cos()
                    + (self.damper * self.damper * c1 + 2. * c * self.damper * c0 - c.powi(2) * c1)
                        * (c * self.speed * x).cos())
                / E.powf(self.damper * self.speed * x)
        } else {
            let c1 = self.velocity / self.speed + c0;
            self.speed.powi(2) * (c0 - 2. * c1 + c1 * self.speed * x) / E.powf(self.speed * x)
        }
    }

    fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn set_position(&mut self, value: f32) {
        let r = self.position_velocity();
        self.position = value;
        self.velocity = r.1;
        self.reset_time();
    }

    pub fn set_velocity(&mut self, value: f32) {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = value;
        self.reset_time();
    }

    pub fn set_damper(&mut self, value: f32) {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        self.damper = value;
        self.reset_time();
    }

    pub fn set_speed(&mut self, value: f32) {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        self.speed = value;
        self.reset_time();
    }

    pub fn set_target(&mut self, value: f32) {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        self.target = value;
        self.reset_time();
    }
}

pub struct Spring2D {
    sx: Spring,
    sy: Spring,
}

impl Spring2D {
    pub fn new(start_pos: (f32, f32)) -> Self {
        Self {
            sx: Spring::new(start_pos.0),
            sy: Spring::new(start_pos.1),
        }
    }

    pub fn position(&mut self) -> (f32, f32) {
        (self.sx.position(), self.sy.position())
    }

    pub fn position_rounded(&mut self) -> (f32, f32) {
        (self.sx.position_rounded(), self.sy.position_rounded())
    }

    pub fn velocity(&mut self) -> (f32, f32) {
        (self.sx.velocity(), self.sy.velocity())
    }

    pub fn damper(&self) -> f32 {
        self.sx.damper
    }

    pub fn speed(&self) -> f32 {
        self.sx.speed
    }

    pub fn target(&self) -> (f32, f32) {
        (self.sx.target, self.sy.target)
    }

    pub fn set_position(&mut self, value: (f32, f32)) {
        self.sx.position = value.0;
        self.sy.position = value.1;
    }

    pub fn set_velocity(&mut self, value: (f32, f32)) {
        self.sx.velocity = value.0;
        self.sy.velocity = value.1;
    }

    pub fn set_damper(&mut self, value: f32) {
        self.sx.damper = value;
        self.sy.damper = value;
    }

    pub fn set_speed(&mut self, value: f32) {
        self.sx.speed = value;
        self.sy.speed = value;
    }

    pub fn set_target(&mut self, value: (f32, f32)) {
        self.sx.set_target(value.0);
        self.sy.set_target(value.1);
    }

    pub fn arrived(&mut self) -> bool {
        self.sx.arrived() && self.sy.arrived()
    }
}
