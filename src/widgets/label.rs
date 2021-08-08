use crate::traits::Control;
use crate::utils::drawing::TextAlign;
use crate::utils::uid::gen_uid;
use crate::utils::{ControlUid};


pub struct TextLabelControl {
    _uid: ControlUid,
    text: String,
    text_color: u32,
    text_size: f32,
    text_align: TextAlign,
}

impl Default for TextLabelControl {
    fn default() -> Self {
        Self {
            _uid: gen_uid(),
            text: "".to_string(),
            text_size: 13.,
            text_align: TextAlign::Left,
            text_color: 0x202020FF,
        }
    }
}

impl TextLabelControl {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn set_color(&mut self, color: u32) {
        self.text_color = color;
    }

    #[inline]
    pub fn set_text_size(&mut self, size: f32) {
        self.text_size = size;
    }

    #[inline]
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.text_align = align;
    }
}
impl Clone for TextLabelControl {
    fn clone(&self) -> Self {
        Self {
            _uid: gen_uid(),
            text_size: self.text_size,
            text_color: self.text_color,
            text_align: self.text_align,
            text: self.text.to_owned(),
        }
    }
}

impl<D> Control<D> for TextLabelControl {

    #[inline]
    fn uid(&self) -> crate::utils::ControlUid {
        self._uid
    }
}
