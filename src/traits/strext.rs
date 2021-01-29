pub trait StrExt {
    fn to_w(&self) -> Vec<u16>;
}

pub fn w_str(s: &str) -> Vec<u16> {
    s.to_w()
}

impl StrExt for &str {
    fn to_w(&self) -> Vec<u16> {
        self.encode_utf16().chain(Some(0)).collect::<Vec<u16>>()
    }
}
