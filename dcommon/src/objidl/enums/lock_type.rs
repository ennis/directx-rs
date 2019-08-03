#[auto_enum::enum_flags]
pub enum LockType {
    WRITE = 1,
    EXCLUSIVE = 2,
    ONLY_ONCE = 4,
}
