#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FileTime {
    pub low_part: u32,
    pub high_part: u32,
}

impl From<u64> for FileTime {
    fn from(u: u64) -> FileTime {
        FileTime {
            low_part: u as u32,
            high_part: (u >> 32) as u32,
        }
    }
}

impl From<FileTime> for u64 {
    fn from(ft: FileTime) -> u64 {
        ((ft.high_part as u64) << 32) | (ft.low_part as u64)
    }
}
