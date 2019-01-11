use crate::bitmap_frame_encode::BitmapFrameEncode;
use crate::bitmap_source::BitmapSource;
use crate::color_context::ColorContext;
use crate::enums::BitmapEncoderCacheOptions;
use crate::palette::Palette;

use com_wrapper::ComWrapper;
use dcommon::{
    helpers::{wrap_com, wrap_opt_com},
    objidl,
    ocidl::property_bag2::PropertyBag2,
    Error, Status, GUID,
};
use winapi::um::wincodec::IWICBitmapEncoder;
use wio::com::ComPtr;

pub use self::info::BitmapEncoderInfo;

pub mod info;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct BitmapEncoder {
    ptr: ComPtr<IWICBitmapEncoder>,
}

impl BitmapEncoder {
    pub fn create(info: &BitmapEncoderInfo) -> Result<BitmapEncoder, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*info.get_raw()).CreateInstance(&mut ptr);
            Error::map_if(hr, || BitmapEncoder::from_raw(ptr))
        }
    }

    pub fn initialize(
        &mut self,
        stream: objidl::Stream,
        options: BitmapEncoderCacheOptions,
    ) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.Initialize(stream.get_raw(), options as u32);
            Error::map_status(hr)
        }
    }

    pub fn container_format(&self) -> Result<GUID, Error> {
        unsafe {
            let mut guid = std::mem::zeroed();
            let hr = self.ptr.GetContainerFormat(&mut guid);
            Error::map(hr, guid)
        }
    }

    pub fn encoder_info(&self) -> Result<BitmapEncoderInfo, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetEncoderInfo(&mut ptr);
            Error::map_if(hr, || BitmapEncoderInfo::from_raw(ptr))
        }
    }

    pub fn set_color_contexts(&mut self, ctxs: &[ColorContext]) -> Result<Status, Error> {
        assert!(ctxs.len() < std::u32::MAX as usize);
        unsafe {
            let hr = self
                .ptr
                .SetColorContexts(ctxs.len() as u32, ctxs.as_ptr() as _);
            Error::map_status(hr)
        }
    }

    pub fn set_palette(&mut self, palette: Palette) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.SetPalette(palette.get_raw());
            Error::map_status(hr)
        }
    }

    pub fn set_thumbnail(&mut self, thumb: &BitmapSource) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.SetThumbnail(thumb.get_raw());
            Error::map_status(hr)
        }
    }

    pub fn set_preview(&mut self, prev: &BitmapSource) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.SetPreview(prev.get_raw());
            Error::map_status(hr)
        }
    }

    pub fn create_frame(&mut self) -> Result<(BitmapFrameEncode, Option<PropertyBag2>), Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let mut pbag = std::ptr::null_mut();
            let hr = self.ptr.CreateNewFrame(&mut ptr, &mut pbag);
            Error::map_if(hr, || (wrap_com(ptr), wrap_opt_com(pbag)))
        }
    }

    pub fn commit(&mut self) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.Commit();
            Error::map_status(hr)
        }
    }

    // TODO: GetMetadataQueryWriter
}
