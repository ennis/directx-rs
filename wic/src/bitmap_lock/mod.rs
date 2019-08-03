use crate::descriptions::{pixel_format::PixelFormatDescription, PixelFormat};

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::Sizeu;
use winapi::um::wincodec::IWICBitmapLock;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct BitmapLock {
    ptr: ComPtr<IWICBitmapLock>,
}

impl BitmapLock {
    pub fn size(&self) -> Result<Sizeu, Error> {
        unsafe {
            let mut w = 0;
            let mut h = 0;
            let hr = self.ptr.GetSize(&mut w, &mut h);
            Error::map(hr, (w, h).into())
        }
    }

    pub fn pixel_format(&self) -> Result<PixelFormat, Error> {
        unsafe {
            let mut guid = std::mem::zeroed();
            let hr = self.ptr.GetPixelFormat(&mut guid);
            Error::map(hr, PixelFormat { guid })
        }
    }

    pub fn pixel_format_desc(&self) -> Option<&'static PixelFormatDescription> {
        self.pixel_format().ok().and_then(|f| f.description())
    }

    pub unsafe fn data_ptr(&mut self) -> Result<(*mut u8, u32), Error> {
        let mut len = 0;
        let mut ptr = std::ptr::null_mut();
        let hr = self.ptr.GetDataPointer(&mut len, &mut ptr);
        Error::map(hr, (ptr, len))
    }
}
