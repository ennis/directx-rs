use crate::helpers::wide::CoTaskWString;
use crate::minwindef::FileTime;
use crate::objidl::enums::{LockType, StorageType};

use checked_enum::UncheckedEnum;
use winapi::shared::guiddef::CLSID;

#[repr(C)]
pub struct Stat {
    pub name: Option<CoTaskWString>,
    pub kind: UncheckedEnum<StorageType>,
    pub byte_size: u64,
    pub mtime: FileTime,
    pub ctime: FileTime,
    pub atime: FileTime,
    pub grf_mode: u32,
    pub grf_locks_supported: LockType,
    pub clsid: CLSID,
    pub state_bits: u32,
    pub reserved: u32,
}
