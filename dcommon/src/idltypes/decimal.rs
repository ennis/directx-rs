

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Decimal {
    pub reserved: u16,
    pub scale: u8,
    pub sign: u8,
    pub high: u32,
    pub low: u64,
}