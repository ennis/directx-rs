use crate::helpers::wrap_ref_to_raw_mut_com;
use crate::objidl::enums::{CommitFlags, LockType, StatFlags};
use crate::objidl::sequential_stream::custom::SequentialStream;
use crate::objidl::stream::stat::Stat;
use crate::{Error, Status};

use std::io::SeekFrom;

use com_impl::Refcount;
use com_impl::VTable;
use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::winerror::{E_FAIL, SUCCEEDED, S_OK};
use winapi::um::objidlbase::{
    ISequentialStream, ISequentialStreamVtbl, IStream, IStreamVtbl, STATSTG, STREAM_SEEK_CUR,
    STREAM_SEEK_END, STREAM_SEEK_SET,
};
use winapi::um::winnt::{LARGE_INTEGER, ULARGE_INTEGER};

pub unsafe trait Stream: SequentialStream + 'static {
    // SequentialStream: fn read(&mut self, buf: &mut [u8]) -> Result<u32, Error>;
    // SequentialStream: fn write(&mut self, buf: &[u8]) -> Result<u32, Error>;
    fn try_clone(&self) -> Result<super::Stream, Error>;
    fn commit(&mut self, flags: CommitFlags) -> Result<Status, Error>;
    fn copy_to(
        &mut self,
        other: &mut super::Stream,
        count: u64,
        read: &mut u64,
        written: &mut u64,
    ) -> Result<Status, Error>;
    fn lock_region(
        &mut self,
        offset: u64,
        count: u64,
        lock_type: LockType,
    ) -> Result<Status, Error>;
    fn revert(&mut self) -> Result<Status, Error>;
    fn seek(&mut self, seek: SeekFrom) -> Result<u64, Error>;
    fn set_size(&mut self, newsize: u64) -> Result<Status, Error>;
    fn stat(&mut self, stat: &mut Stat, flags: StatFlags) -> Result<Status, Error>;
    fn unlock_region(
        &mut self,
        offset: u64,
        count: u64,
        lock_type: LockType,
    ) -> Result<Status, Error>;
}

#[repr(C)]
#[derive(ComImpl)]
pub struct CustomStream<S>
where
    S: Stream,
{
    vtbl: VTable<IStreamVtbl>,
    refcount: Refcount,
    stream: S,
}

#[com_impl]
unsafe impl<S> ISequentialStream for CustomStream<S>
where
    S: Stream,
{
    #[panic(result = "E_FAIL")]
    unsafe fn read(&mut self, pv: *mut c_void, cb: u32, pcbRead: *mut u32) -> i32 {
        *pcbRead = 0;

        let slice = std::slice::from_raw_parts_mut(pv as *mut u8, cb as usize);
        match self.stream.read(slice) {
            Ok(cbread) => {
                *pcbRead = cbread;
                S_OK
            }
            Err(e) => e.0,
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn write(&mut self, pv: *const c_void, cb: u32, pcbWritten: *mut u32) -> i32 {
        *pcbWritten = 0;

        let slice = std::slice::from_raw_parts(pv as *const u8, cb as usize);
        match self.stream.write(slice) {
            Ok(cbwritten) => {
                *pcbWritten = cbwritten;
                S_OK
            }
            Err(e) => e.0,
        }
    }
}

#[com_impl]
unsafe impl<S> IStream for CustomStream<S>
where
    S: Stream,
{
    #[panic(result = "E_FAIL")]
    unsafe fn clone(&self, ppstm: *mut *mut IStream) -> i32 {
        *ppstm = std::ptr::null_mut();
        let stream = match self.stream.try_clone() {
            Ok(stream) => stream,
            Err(e) => return e.0,
        };

        *ppstm = stream.into_raw();
        S_OK
    }

    #[panic(result = "E_FAIL")]
    unsafe fn commit(&mut self, flags: u32) -> i32 {
        match self.stream.commit(CommitFlags(flags)) {
            Ok(Status(status)) => {
                assert!(SUCCEEDED(status));
                status
            }
            Err(Error(error)) => {
                assert!(!SUCCEEDED(error));
                error
            }
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn copy_to(
        &mut self,
        mut other: *mut IStream,
        cb: ULARGE_INTEGER,
        pcbread: *mut ULARGE_INTEGER,
        pcbwritten: *mut ULARGE_INTEGER,
    ) -> i32 {
        *(*pcbread).QuadPart_mut() = 0;
        *(*pcbwritten).QuadPart_mut() = 0;

        let count: u64 = *cb.QuadPart();
        let mut read = 0;
        let mut written = 0;
        let other = wrap_ref_to_raw_mut_com(&mut other);

        let result = self.stream.copy_to(other, count, &mut read, &mut written);

        *(*pcbread).QuadPart_mut() = read;
        *(*pcbwritten).QuadPart_mut() = written;

        match result {
            Ok(Status(status)) => {
                assert!(SUCCEEDED(status));
                status
            }
            Err(Error(error)) => {
                assert!(!SUCCEEDED(error));
                error
            }
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn lock_region(
        &mut self,
        offset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        locktype: u32,
    ) -> i32 {
        let offset = *offset.QuadPart();
        let count = *cb.QuadPart();

        let result = self.stream.lock_region(offset, count, LockType(locktype));

        match result {
            Ok(Status(status)) => {
                assert!(SUCCEEDED(status));
                status
            }
            Err(Error(error)) => {
                assert!(!SUCCEEDED(error));
                error
            }
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn revert(&mut self) -> i32 {
        let result = self.stream.revert();

        match result {
            Ok(Status(status)) => {
                assert!(SUCCEEDED(status));
                status
            }
            Err(Error(error)) => {
                assert!(!SUCCEEDED(error));
                error
            }
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn seek(
        &mut self,
        moveamt: LARGE_INTEGER,
        origin: u32,
        newpos: *mut ULARGE_INTEGER,
    ) -> i32 {
        let moveamt = *moveamt.QuadPart();
        let seek = match origin {
            STREAM_SEEK_SET => SeekFrom::Start(moveamt as u64),
            STREAM_SEEK_CUR => SeekFrom::Current(moveamt),
            STREAM_SEEK_END => SeekFrom::End(moveamt),
            _ => panic!("Invalid stream seek origin `0x{:x}`", origin),
        };

        match self.stream.seek(seek) {
            Ok(pos) => {
                if !newpos.is_null() {
                    *(*newpos).QuadPart_mut() = pos;
                }
                S_OK
            }
            Err(e) => e.0,
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn set_size(&mut self, newsize: ULARGE_INTEGER) -> i32 {
        let newsize = *newsize.QuadPart();

        let result = self.stream.set_size(newsize);

        match result {
            Ok(Status(status)) => {
                assert!(SUCCEEDED(status));
                status
            }
            Err(Error(error)) => {
                assert!(!SUCCEEDED(error));
                error
            }
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn stat(&mut self, pstat: *mut STATSTG, statflag: u32) -> i32 {
        let pstat = &mut *(pstat as *mut Stat);
        let result = self.stream.stat(pstat, StatFlags(statflag));

        match result {
            Ok(Status(status)) => {
                assert!(SUCCEEDED(status));
                status
            }
            Err(Error(error)) => {
                assert!(!SUCCEEDED(error));
                error
            }
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn unlock_region(
        &mut self,
        offset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        locktype: u32,
    ) -> i32 {
        let offset = *offset.QuadPart();
        let count = *cb.QuadPart();

        let result = self.stream.unlock_region(offset, count, LockType(locktype));

        match result {
            Ok(Status(status)) => {
                assert!(SUCCEEDED(status));
                status
            }
            Err(Error(error)) => {
                assert!(!SUCCEEDED(error));
                error
            }
        }
    }
}
