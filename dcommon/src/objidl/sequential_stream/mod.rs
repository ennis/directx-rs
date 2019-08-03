use crate::Error;

use com_wrapper::ComWrapper;
use winapi::um::objidlbase;
use wio::com::ComPtr;

pub mod custom;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
/// A stream which can be read from and written to on a very basic level.
///
/// std::io::Write is not implemented for SequentialStream because `flush` cannot be
/// implemented on this type.
pub struct SequentialStream {
    ptr: ComPtr<objidlbase::ISequentialStream>,
}

pub unsafe trait ISequentialStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        assert!(
            buf.len() <= std::u32::MAX as usize,
            "buf.len() = {}",
            buf.len()
        );
        unsafe {
            let mut read_bytes = 0;
            let hr = self.raw_sstream().Read(
                buf.as_mut_ptr() as *mut _,
                buf.len() as u32,
                &mut read_bytes,
            );

            Error::map(hr, read_bytes as usize)
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let len = std::cmp::min(buf.len(), std::u32::MAX as usize) as u32;
        unsafe {
            let mut written_bytes = 0;
            let hr = self
                .raw_sstream()
                .Write(buf.as_ptr() as *const _, len, &mut written_bytes);

            Error::map(hr, written_bytes as usize)
        }
    }

    unsafe fn raw_sstream(&self) -> &objidlbase::ISequentialStream;
}

unsafe impl ISequentialStream for SequentialStream {
    unsafe fn raw_sstream(&self) -> &objidlbase::ISequentialStream {
        &self.ptr
    }
}

impl std::io::Read for SequentialStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        ISequentialStream::read(self, buf).map_err(|e| e.into())
    }
}
