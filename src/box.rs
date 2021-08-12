//! A struct type to store an area for rendering

#[derive(Debug, Clone, Copy)]
pub struct AreaBox {
    pub left: isize,
    pub right: isize,
    pub top: isize,
    pub down: isize,
}

impl AreaBox {
    pub const ZERO: AreaBox = AreaBox {
        left: 0,
        right: 0,
        top: 0,
        down: 0,
    };
    
    pub const MAX: AreaBox = AreaBox {
        left: isize::MIN,
        right: isize::MAX,
        top: isize::MIN,
        down: isize::MAX,
    };

    pub fn expand(self, offset: isize) -> Self {
        Self {
            left: self.left + offset,
            right: self.right + offset,
            top: self.top + offset,
            down: self.down + offset,
        }
    }

    pub fn fit_in(self, area_box: &AreaBox) -> Self {
        Self {
            left: self.left.min(area_box.left),
            right: self.right.max(area_box.right),
            top: self.top.min(area_box.top),
            down: self.down.max(area_box.down),
        }
    }

    pub fn to_size_f32(&self) -> (f32, f32) {
        (
            (self.right - self.left) as _,
            (self.down - self.top) as _
        )
    }
}
