#[auto_enum(u32, checked)]
pub enum StorageType {
    Storage = 1,
    Stream = 2,
    LockBytes = 3,
    Property = 4,
}