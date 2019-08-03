use std::slice::from_raw_parts;

use winapi::um::wincodec::WICBitmapPattern;

#[derive(Copy, Clone, Debug)]
pub struct BitmapPattern<'a> {
    pub position: u64,
    pub pattern: &'a [u8],
    pub mask: &'a [u8],
    pub end_of_stream: bool,
}

impl<'a> BitmapPattern<'a> {
    pub unsafe fn from_raw(raw: &WICBitmapPattern) -> BitmapPattern<'a> {
        BitmapPattern {
            position: *raw.Position.QuadPart(),
            pattern: from_raw_parts(raw.Pattern, raw.Length as usize),
            mask: from_raw_parts(raw.Mask, raw.Length as usize),
            end_of_stream: raw.EndOfStream != 0,
        }
    }
}
