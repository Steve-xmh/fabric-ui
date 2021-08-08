use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut;

use font_kit::properties::Weight;
use font_kit::{font::Font, properties::Properties, source::SystemSource};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::wingdi::{
    CreateCompatibleDC, CreateDIBSection, DeleteDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER,
    BI_RGB, DIB_RGB_COLORS, RGBQUAD,
};
use winapi::um::winuser::{GetDC, ReleaseDC};
use winapi::{
    shared::windef::{HBITMAP, HDC, HWND},
    um::wingdi::DeleteObject,
};

use crate::utils::DrawTargetExt;

pub struct Fabric {
    pub width: u32,
    pub height: u32,
    pub raw: Box<[u32]>,
    pub dt: DrawTargetExt,
    pub m_hdc: HDC,
    pub m_hbmp: HBITMAP,
    pub info: BITMAPINFO,
    pub font: Font,
    pub font_bold: Font,
    hwnd: HWND,
}

fn create_bitmap_hdc(h_wnd: HWND, info: &BITMAPINFO) -> (HDC, HBITMAP, Box<[u32]>) {
    unsafe {
        let mut ppv_bits: *mut c_void = null_mut();
        let dc = GetDC(h_wnd);
        let buf_dc = CreateCompatibleDC(dc);
        ReleaseDC(h_wnd, dc);
        if buf_dc == null_mut() {
            let err = GetLastError();
            panic!("Error buf_dc {}", err);
        }
        let buf_bitmap =
            CreateDIBSection(buf_dc, info, DIB_RGB_COLORS, &mut ppv_bits, null_mut(), 0);
        if buf_bitmap == null_mut() {
            let err = GetLastError();
            panic!("Error buf_bitmap {}", err);
        }
        if ppv_bits == null_mut() {
            let err = GetLastError();
            panic!("Error ppv_bits {}", err);
        }
        // let mut raw_slice: [usize; 2] = [ppv_bits as usize, (info.bmiHeader.biWidth * info.bmiHeader.biHeight) as usize * 4];
        let raw = Box::from_raw(std::slice::from_raw_parts_mut(
            ppv_bits as *mut u32,
            (info.bmiHeader.biWidth * info.bmiHeader.biHeight) as usize,
        ));
        SelectObject(buf_dc, buf_bitmap as *mut c_void);
        (buf_dc, buf_bitmap, raw)
    }
}

// RGBA
impl Fabric {
    pub fn new(h_wnd: HWND, width: u32, height: u32) -> Self {
        let info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: height as i32,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }],
        };
        let (m_hdc, m_hbmp, raw) = create_bitmap_hdc(h_wnd, &info);
        let fonts = [
            font_kit::family_name::FamilyName::Title("Microsoft YaHei".to_string()),
            font_kit::family_name::FamilyName::Title("Helvetica".to_string()),
            font_kit::family_name::FamilyName::Title("Arial".to_string()),
        ];
        let f = Self {
            raw,
            width,
            height,
            info,
            m_hdc,
            m_hbmp,
            hwnd: h_wnd,
            font: SystemSource::new()
                .select_best_match(&fonts, &Properties::new())
                .unwrap()
                .load()
                .unwrap(),
            font_bold: SystemSource::new()
                .select_best_match(&fonts, Properties::new().weight(Weight::BOLD))
                .unwrap()
                .load()
                .unwrap(),
            dt: DrawTargetExt::new(width as i32, height as i32),
        };
        f
    }

    pub fn sync(&mut self) {
        for i in 0..(self.width * self.height) as usize {
            let pos =
                ((self.height as usize - (i - i % self.width as usize) / self.width as usize) - 1)
                    * self.width as usize
                    + i % self.width as usize;
            self.set_pixel(pos, self.dt.pixmap.pixels()[i].get());
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, pos: usize, color: u32) {
        let raw = self.raw.as_mut();
        raw[pos as usize] = (color & 0xff000000)
            | (color & 0xff00)
            | ((color & 0xff) << 16)
            | ((color & 0xff0000) >> 16);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        /*
        let prev_width = self.dt.pixmap.width();
        let prev_height = self.dt.pixmap.height();
        let prev_data = self.dt.pixmap.data();
        */
        self.dt = DrawTargetExt::new(width as i32, height as i32);
        self.info.bmiHeader.biWidth = width as i32;
        self.info.bmiHeader.biHeight = height as i32;
        self.width = width;
        self.height = height;

        unsafe {
            DeleteObject(self.m_hbmp as *mut c_void);
            DeleteDC(self.m_hdc);
        }
        // println!("Delete Old Bitmap {}", DeleteObject(self.m_hbmp as *mut c_void));
        // println!("Release Old DC {}", DeleteDC(self.m_hdc));
        let (a, b, c) = create_bitmap_hdc(self.hwnd, &self.info);
        self.m_hdc = a;
        self.m_hbmp = b;
        let old_raw = std::mem::replace(&mut self.raw, c);
        Box::leak(old_raw);
        /*
                self.dt.draw_image_with_size_at(
                    width as f32,
                    height as f32,
                    0.,
                    0.,
                    &Image {
                        width: prev_width,
                        height: prev_height,
                        data: &prev_data,
                    },
                    &Default::default(),
                );
        */
    }
}

impl super::super::traits::Fabric for Fabric {
    
}

impl Drop for Fabric {
    fn drop(&mut self) {
        unsafe {
            DeleteDC(self.m_hdc);
        }
    }
}
