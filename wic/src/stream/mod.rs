use crate::imaging_factory::ImagingFactory;

use std::ffi::OsStr;
use std::ptr::NonNull;

use com_wrapper::ComWrapper;
use dcommon::helpers::{deref_com_wrapper, deref_com_wrapper_mut};
use dcommon::objidl::{self, enums::AccessMask};
use dcommon::{Error, Status};
use winapi::um::wincodec::IWICStream;
use wio::com::ComPtr;
use wio::wide::ToWide;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, debug)]
pub struct Stream {
    ptr: ComPtr<IWICStream>,
}

impl Stream {
    pub fn create(factory: &ImagingFactory) -> Result<Self, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory.get_raw()).CreateStream(&mut ptr);
            Error::map_if(hr, || Self::from_raw(ptr))
        }
    }

    pub fn initialize_from_stream(&mut self, stream: objidl::Stream) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.InitializeFromIStream(stream.get_raw());
            Error::map_status(hr)
        }
    }

    pub fn initialize_from_stream_region(
        &mut self,
        stream: objidl::Stream,
        offset: u64,
        len: u64,
    ) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.InitializeFromIStreamRegion(
                stream.get_raw(),
                std::mem::transmute(offset),
                std::mem::transmute(len),
            );
            Error::map_status(hr)
        }
    }

    pub fn intialize_from_filename(
        &mut self,
        filename: impl AsRef<OsStr>,
        access: AccessMask,
    ) -> Result<Status, Error> {
        let filename = filename.as_ref().to_wide_null();
        unsafe {
            let hr = self.ptr.InitializeFromFilename(filename.as_ptr(), access.0);
            Error::map_status(hr)
        }
    }

    /// The caller is responsible for ensuring the memory passed lives for the life of the stream.
    /// It may be recommended to use a custom implementation of IStream that owns your memory
    /// instead and use initialize_from_stream. You must also ensure `data` is not longer than
    /// `u32::MAX`.
    pub unsafe fn initialize_from_memory(
        &mut self,
        mut data: NonNull<[u8]>,
    ) -> Result<Status, Error> {
        let data = data.as_mut();
        let hr = self
            .ptr
            .InitializeFromMemory(data.as_mut_ptr(), data.len() as u32);
        Error::map_status(hr)
    }
}

impl std::ops::Deref for Stream {
    type Target = objidl::Stream;
    fn deref(&self) -> &Self::Target {
        unsafe { deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Stream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { deref_com_wrapper_mut(self) }
    }
}
