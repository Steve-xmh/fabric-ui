pub mod window;
pub use window::WindowControl;
pub mod button;
pub use button::ButtonControl;
pub mod label;
pub use label::TextLabelControl;
pub mod image;
pub use self::image::ImageControl;
pub mod input;
pub use input::InputControl;

pub(crate) mod template;
