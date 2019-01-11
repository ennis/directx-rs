use crate::bitmap_source::BitmapSource;
use crate::color_context::ColorContext;
use crate::metadata_query_reader::MetadataQueryReader;

use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::um::wincodec::IWICBitmapFrameDecode;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct BitmapFrameDecode {
    ptr: ComPtr<IWICBitmapFrameDecode>,
}

impl BitmapFrameDecode {
    pub fn metadata_query_reader(&mut self) -> Result<MetadataQueryReader, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetMetadataQueryReader(&mut ptr);
            Error::map_if(hr, || MetadataQueryReader::from_raw(ptr))
        }
    }

    pub fn thumbnail(&mut self) -> Result<BitmapSource, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetThumbnail(&mut ptr);
            Error::map_if(hr, || BitmapSource::from_raw(ptr))
        }
    }

    pub fn color_contexts(&mut self) -> Result<Vec<ColorContext>, Error> {
        unsafe {
            let mut count = 0;
            let hr = self
                .ptr
                .GetColorContexts(0, std::ptr::null_mut(), &mut count);
            Error::map_status(hr)?;
            let mut buf = Vec::with_capacity(count as usize);
            let len = count;
            let hr = self
                .ptr
                .GetColorContexts(len, buf.as_mut_ptr() as _, &mut count);
            Error::map_if(hr, || {
                buf.set_len(count as usize);
                buf
            })
        }
    }

    pub fn into_source(self) -> BitmapSource {
        unsafe { BitmapSource::from_raw(self.ptr.into_raw() as _) }
    }
}

impl std::ops::Deref for BitmapFrameDecode {
    type Target = BitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for BitmapFrameDecode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper_mut(self) }
    }
}
