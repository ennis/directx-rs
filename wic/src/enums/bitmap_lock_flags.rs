#[auto_enum::enum_flags]
pub enum BitmapLockFlags {
    NONE = 0,
    READ = 0x1,
    WRITE = 0x2,
}
