use crate::utils::WidgetUid;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EventResult {
    NoBubbling,
    Bubble,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindowEvent {
    None,
    Quit,
    CloseWindow,
    HideWindow,
    ShowWindow,
    KeyDown(usize),
    KeyUp(usize),
    KeyPress(usize),
    /// Pos X Y
    MouseMove(u32, u32),
    MouseDown(u32, u32),
    MouseUp(u32, u32),
    MousePress(u32, u32),
    // Resize & Move
    WindowMove(i32, i32),
    WindowResize(u32, u32),
    // Character input
    CharInput(WidgetUid, char),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserEvent {
    None,
    Quit,
    ControlClicked(WidgetUid),
    /// Only for SystemWindow, never return to user.
    WindowResize(u32, u32),
}
