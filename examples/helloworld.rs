use fabric_ui::prelude::*;

fn main() {
    Application::new(
        Box::new(widgets::WindowControl::new(Box::new(
            widgets::TextLabelControl::new("text"),
        ))),
        (),
    )
    .run();
}
