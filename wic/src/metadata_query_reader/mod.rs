use crate::GUID;

use std::ffi::OsStr;

use com_wrapper::ComWrapper;
use dcommon::idltypes::propvariant::PropVariant;
use dcommon::objidl::EnumString;
use dcommon::Error;
use winapi::um::wincodec::IWICMetadataQueryReader;
use wio::com::ComPtr;
use wio::wide::ToWide;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, debug)]
pub struct MetadataQueryReader {
    ptr: ComPtr<IWICMetadataQueryReader>,
}

impl MetadataQueryReader {
    pub fn container_format(&mut self) -> Result<GUID, Error> {
        unsafe {
            let mut guid = std::mem::zeroed();
            let hr = self.ptr.GetContainerFormat(&mut guid);
            Error::map(hr, guid)
        }
    }

    pub fn location(&mut self) -> Result<String, Error> {
        unsafe {
            let mut len = 0;
            let hr = self.ptr.GetLocation(0, 0 as _, &mut len);
            Error::map_status(hr)?;
            let mut data = vec![0; len as usize + 1];
            let hr = self
                .ptr
                .GetLocation(data.len() as u32, data.as_mut_ptr(), &mut len);
            Error::map_if(hr, || String::from_utf16_lossy(&data))
        }
    }

    pub fn metadata_by_name(&mut self, name: impl AsRef<OsStr>) -> Result<PropVariant, Error> {
        let prop_name = name.as_ref().to_wide_null();
        unsafe {
            let mut prop = std::mem::zeroed();
            let hr = self.ptr.GetMetadataByName(prop_name.as_ptr(), &mut prop);
            Error::map_if(hr, || std::mem::transmute(prop))
        }
    }

    pub fn enumerator(&mut self) -> Result<EnumString, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetEnumerator(&mut ptr);
            Error::map_if(hr, || EnumString::from_raw(ptr))
        }
    }
}
