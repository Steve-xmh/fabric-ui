use tiny_skia::Color;
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::dwmapi::DwmGetColorizationColor;

pub fn get_theme_color_skia_color() -> Color {
    let c = get_theme_color();
    Color::from_rgba8(
        (c >> 24) as u8,
        (c >> 16) as u8,
        (c >> 8) as u8,
        (c & 0xFF) as u8,
    )
}

// DwmGetColorizationParameters

/// Get user rgba theme color, in RGBA order.
pub fn get_theme_color() -> u32 {
    unsafe {
        let mut c: DWORD = 0;
        let mut o: BOOL = 0;
        DwmGetColorizationColor(&mut c, &mut o);
        (c << 8) | (c >> 24)
        /*
        let c = GetThemeSysColor(null_mut(), COLOR_ACTIVECAPTION);
        ((c & 0xFF) << 24) |
        ((c & 0xFF00) << 8) |
        ((c & 0xFF0000) >> 8) |
        0xFF
        */
    }
}
