use crate::objidl::{
    enums::{CommitFlags, LockType, StatFlags},
    ISequentialStream,
};
use crate::{Error, Status};

use std::io::SeekFrom;

use com_wrapper::ComWrapper;
use winapi::um::objidlbase::{self, STREAM_SEEK_CUR, STREAM_SEEK_END, STREAM_SEEK_SET};
use winapi::um::winnt::{LARGE_INTEGER, ULARGE_INTEGER};
use wio::com::ComPtr;

pub use self::stat::Stat;

pub mod custom;
pub mod stat;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct Stream {
    ptr: ComPtr<objidlbase::IStream>,
}

pub unsafe trait IStream {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        let (pos, flag) = match pos {
            SeekFrom::Start(pos) => (pos as i64, STREAM_SEEK_SET),
            SeekFrom::Current(pos) => (pos, STREAM_SEEK_CUR),
            SeekFrom::End(pos) => (pos, STREAM_SEEK_END),
        };

        unsafe {
            let mut lpos: LARGE_INTEGER = std::mem::zeroed();
            *lpos.QuadPart_mut() = pos;

            let mut result = std::mem::zeroed();
            let hr = self.raw_stream().Seek(lpos, flag, &mut result);

            Error::map(hr, *result.QuadPart())
        }
    }

    fn set_size(&mut self, new_size: u64) -> Result<Status, Error> {
        unsafe {
            let mut lsize: ULARGE_INTEGER = std::mem::zeroed();
            *lsize.QuadPart_mut() = new_size;

            let hr = self.raw_stream().SetSize(lsize);
            Error::map_status(hr)
        }
    }

    fn copy_to(
        &mut self,
        other: &mut dyn IStream,
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

            let other_ptr = other.raw_stream() as *const _ as *mut _;
            let hr = self
                .raw_stream()
                .CopyTo(other_ptr, count, cb_read, cb_written);
            Error::map_status(hr)
        }
    }

    fn commit(&mut self, flags: CommitFlags) -> Result<Status, Error> {
        unsafe {
            let hr = self.raw_stream().Commit(flags.0);
            Error::map_status(hr)
        }
    }

    fn revert(&mut self) -> Result<Status, Error> {
        unsafe {
            let hr = self.raw_stream().Revert();
            Error::map_status(hr)
        }
    }

    fn lock_region(
        &mut self,
        offset: u64,
        count: u64,
        lock_type: LockType,
    ) -> Result<Status, Error> {
        unsafe {
            let hr = self.raw_stream().LockRegion(
                std::mem::transmute(offset),
                std::mem::transmute(count),
                lock_type.0,
            );
            Error::map_status(hr)
        }
    }

    fn unlock_region(
        &mut self,
        offset: u64,
        count: u64,
        lock_type: LockType,
    ) -> Result<Status, Error> {
        unsafe {
            let hr = self.raw_stream().UnlockRegion(
                std::mem::transmute(offset),
                std::mem::transmute(count),
                lock_type.0,
            );
            Error::map_status(hr)
        }
    }

    fn stat(&self, flags: StatFlags) -> Result<Stat, Error> {
        unsafe {
            let mut stat = std::mem::zeroed();
            let hr = self.raw_stream().Stat(&mut stat, flags.0);
            Error::map_if(hr, || std::mem::transmute(stat))
        }
    }

    fn try_clone(&self) -> Result<Stream, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_stream().Clone(&mut ptr);
            Error::map_if(hr, || Stream::from_raw(ptr))
        }
    }

    unsafe fn raw_stream(&self) -> &objidlbase::IStream;
}

unsafe impl ISequentialStream for Stream {
    unsafe fn raw_sstream(&self) -> &objidlbase::ISequentialStream {
        &self.ptr
    }
}

unsafe impl IStream for Stream {
    unsafe fn raw_stream(&self) -> &objidlbase::IStream {
        &self.ptr
    }
}

impl std::io::Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        ISequentialStream::read(self, buf).map_err(|e| e.into())
    }
}

impl std::io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        ISequentialStream::write(self, buf).map_err(|e| e.into())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        IStream::commit(self, CommitFlags::DEFAULT)
            .map(|_status| ())
            .map_err(|e| e.into())
    }
}

impl std::io::Seek for Stream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        IStream::seek(self, pos).map_err(|e| e.into())
    }
}
