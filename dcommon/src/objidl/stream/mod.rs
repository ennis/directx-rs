use crate::helpers::{deref_com_wrapper, deref_com_wrapper_mut};
use crate::objidl::{
    enums::{CommitFlags, LockType, StatFlags},
    SequentialStream,
};
use crate::{Error, Status};

use std::io::SeekFrom;

use com_wrapper::ComWrapper;
use winapi::um::objidlbase::IStream;
use winapi::um::objidlbase::{STREAM_SEEK_CUR, STREAM_SEEK_END, STREAM_SEEK_SET};
use winapi::um::winnt::{LARGE_INTEGER, ULARGE_INTEGER};
use wio::com::ComPtr;

pub use self::stat::Stat;

//pub mod custom;
pub mod stat;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct Stream {
    ptr: ComPtr<IStream>,
}

impl Stream {
    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        let (pos, flag) = match pos {
            SeekFrom::Start(pos) => (pos as i64, STREAM_SEEK_SET),
            SeekFrom::Current(pos) => (pos, STREAM_SEEK_CUR),
            SeekFrom::End(pos) => (pos, STREAM_SEEK_END),
        };

        unsafe {
            let mut lpos: LARGE_INTEGER = std::mem::zeroed();
            *lpos.QuadPart_mut() = pos;

            let mut result = std::mem::zeroed();
            let hr = self.ptr.Seek(lpos, flag, &mut result);

            Error::map(hr, *result.QuadPart())
        }
    }

    pub fn set_size(&mut self, new_size: u64) -> Result<Status, Error> {
        unsafe {
            let mut lsize: ULARGE_INTEGER = std::mem::zeroed();
            *lsize.QuadPart_mut() = new_size;

            let hr = self.ptr.SetSize(lsize);
            Error::map_status(hr)
        }
    }

    pub fn copy_to(
        &mut self,
        other: &mut Stream,
        count: u64,
        cb_read: Option<&mut u64>,
        cb_written: Option<&mut u64>,
    ) -> Result<Status, Error> {
        unsafe {
            let count = std::mem::transmute::<u64, ULARGE_INTEGER>(count);
            let cb_read = match cb_read {
                Some(ptr) => ptr as *mut _ as *mut ULARGE_INTEGER,
                None => std::ptr::null_mut(),
            };
            let cb_written = match cb_written {
                Some(ptr) => ptr as *mut _ as *mut ULARGE_INTEGER,
                None => std::ptr::null_mut(),
            };

            let hr = self.ptr.CopyTo(other.get_raw(), count, cb_read, cb_written);
            Error::map_status(hr)
        }
    }

    pub fn commit(&mut self, flags: CommitFlags) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.Commit(flags.0);
            Error::map_status(hr)
        }
    }

    pub fn revert(&mut self) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.Revert();
            Error::map_status(hr)
        }
    }

    pub fn lock_region(
        &mut self,
        offset: u64,
        count: u64,
        lock_type: LockType,
    ) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.LockRegion(
                std::mem::transmute(offset),
                std::mem::transmute(count),
                lock_type.0,
            );
            Error::map_status(hr)
        }
    }

    pub fn unlock_region(
        &mut self,
        offset: u64,
        count: u64,
        lock_type: LockType,
    ) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.UnlockRegion(
                std::mem::transmute(offset),
                std::mem::transmute(count),
                lock_type.0,
            );
            Error::map_status(hr)
        }
    }

    pub fn stat(&self, flags: StatFlags) -> Result<Stat, Error> {
        unsafe {
            let mut stat = std::mem::zeroed();
            let hr = self.ptr.Stat(&mut stat, flags.0);
            Error::map_if(hr, || std::mem::transmute(stat))
        }
    }

    pub fn try_clone(&self) -> Result<Self, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.Clone(&mut ptr);
            Error::map_if(hr, || Self::from_raw(ptr))
        }
    }
}

impl std::io::Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        <SequentialStream as std::io::Read>::read(self, buf)
    }
}

impl std::io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        SequentialStream::write(self, buf).map_err(|e| e.into())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Stream::commit(self, CommitFlags::DEFAULT)
            .map(|_status| ())
            .map_err(|e| e.into())
    }
}

impl std::io::Seek for Stream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Stream::seek(self, pos).map_err(|e| e.into())
    }
}

impl std::ops::Deref for Stream {
    type Target = SequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Stream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { deref_com_wrapper_mut(self) }
    }
}
