#[auto_enum::auto_enum(u32, checked)]
pub enum StorageType {
    StorageObj = 1, // Added Obj because of a name conflict ><
    Stream = 2,
    LockBytes = 3,
    Property = 4,
}
