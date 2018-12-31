use crate::bitmap_source::BitmapSource;
use crate::imaging_factory::ImagingFactory;

use com_wrapper::ComWrapper;
use dcommon::helpers::{deref_com_wrapper, deref_com_wrapper_mut};
use dcommon::{Error, Status};
use math2d::Recti;
use winapi::um::wincodec::IWICBitmapClipper;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct BitmapClipper {
    ptr: ComPtr<IWICBitmapClipper>,
}

impl BitmapClipper {
    pub fn create(factory: &ImagingFactory) -> Result<Self, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory.get_raw()).CreateBitmapClipper(&mut ptr);
            Error::map_if(hr, || Self::from_raw(ptr))
        }
    }

    pub fn initialize(
        &mut self,
        source: &BitmapSource,
        rect: impl Into<Recti>,
    ) -> Result<Status, Error> {
        let rect = rect.into();
        unsafe {
            let hr = self.ptr.Initialize(source.get_raw(), &rect.into());
            Error::map_status(hr)
        }
    }
}

impl std::ops::Deref for BitmapClipper {
    type Target = BitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for BitmapClipper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { deref_com_wrapper_mut(self) }
    }
}
