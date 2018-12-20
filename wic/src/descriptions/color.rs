#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub value: u32,
}

pub unsafe trait ColorType: Sized {}
unsafe impl ColorType for Color {}
unsafe impl ColorType for u32 {}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { value: 0 }.with_r(r).with_g(g).with_b(b).with_a(a)
    }

    pub fn r(self) -> u8 {
        (self.value >> 16) as u8
    }
    pub fn g(self) -> u8 {
        (self.value >> 8) as u8
    }
    pub fn b(self) -> u8 {
        (self.value >> 0) as u8
    }
    pub fn a(self) -> u8 {
        (self.value >> 24) as u8
    }

    pub fn set_r(&mut self, val: u8) {
        self.value = (self.value & 0xFF_00_FF_FF) | ((val as u32) << 16);
    }
    pub fn set_g(&mut self, val: u8) {
        self.value = (self.value & 0xFF_FF_00_FF) | ((val as u32) << 8);
    }
    pub fn set_b(&mut self, val: u8) {
        self.value = (self.value & 0xFF_FF_FF_00) | ((val as u32) << 0);
    }
    pub fn set_a(&mut self, val: u8) {
        self.value = (self.value & 0x00_FF_FF_FF) | ((val as u32) << 24);
    }

    pub fn with_r(mut self, val: u8) -> Self {
        self.set_r(val);
        self
    }
    pub fn with_g(mut self, val: u8) -> Self {
        self.set_g(val);
        self
    }
    pub fn with_b(mut self, val: u8) -> Self {
        self.set_b(val);
        self
    }
    pub fn with_a(mut self, val: u8) -> Self {
        self.set_a(val);
        self
    }
}
