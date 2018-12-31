use crate::metadata_query_reader::MetadataQueryReader;
use crate::palette::Palette;
use crate::enums::{BitmapDecoderCapabilities, DecodeOptions};
use crate::GUID;

use com_wrapper::ComWrapper;
use dcommon::objidl;
use dcommon::{Error, Status};
use winapi::um::wincodec::IWICBitmapDecoder;
use wio::com::ComPtr;

pub use self::info::BitmapDecoderInfo;

pub mod info;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, debug)]
pub struct BitmapDecoder {
    ptr: ComPtr<IWICBitmapDecoder>,
}

impl BitmapDecoder {
    pub fn query_capabilities(
        &self,
        stream: &mut objidl::Stream,
    ) -> Result<BitmapDecoderCapabilities, Error> {
        unsafe {
            let mut caps = 0;
            let hr = self.ptr.QueryCapability(stream.get_raw(), &mut caps);

            Error::map(hr, BitmapDecoderCapabilities(caps))
        }
    }

    pub fn initialize(
        &mut self,
        stream: objidl::Stream,
        options: DecodeOptions,
    ) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.Initialize(stream.get_raw(), options as u32);
            Error::map_status(hr)
        }
    }

    pub fn container_format(&mut self) -> Result<GUID, Error> {
        unsafe {
            let mut guid = std::mem::zeroed();
            let hr = self.ptr.GetContainerFormat(&mut guid);
            Error::map(hr, guid)
        }
    }

    pub fn decoder_info(&self) -> Result<BitmapDecoderInfo, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetDecoderInfo(&mut ptr);
            Error::map_if(hr, || BitmapDecoderInfo::from_raw(ptr))
        }
    }

    pub fn copy_palette(&self, palette: &mut Palette) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.CopyPalette(palette.get_raw());
            Error::map_status(hr)
        }
    }

    pub fn metadata_query_reader(&mut self) -> Result<MetadataQueryReader, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetMetadataQueryReader(&mut ptr);
            Error::map_if(hr, || MetadataQueryReader::from_raw(ptr))
        }
    }
}
