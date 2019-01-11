use std::ffi::OsStr;

use dcommon::helpers::wstrnlen;
use dcommon::Error;
use dcommon::GUID;
use winapi::um::wincodec::IWICBitmapCodecInfo;
use wio::com::ComPtr;
use wio::wide::ToWide;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct BitmapCodecInfo {
    ptr: ComPtr<IWICBitmapCodecInfo>,
}

impl BitmapCodecInfo {
    pub fn container_format(&self) -> Result<GUID, Error> {
        unsafe {
            let mut guid = std::mem::zeroed();
            let hr = self.ptr.GetContainerFormat(&mut guid);
            Error::map(hr, guid)
        }
    }

    pub fn pixel_formats(&self) -> Result<Vec<GUID>, Error> {
        unsafe {
            let mut count = 0;
            let hr = self
                .ptr
                .GetPixelFormats(0, std::ptr::null_mut(), &mut count);
            Error::map_status(hr)?;
            let mut buf = Vec::with_capacity(count as usize);
            let len = count;
            let hr = self.ptr.GetPixelFormats(len, buf.as_mut_ptr(), &mut count);
            Error::map_if(hr, || {
                buf.set_len(count as usize);
                buf
            })
        }
    }

    fn loadstr(f: impl Fn(u32, *mut u16, &mut u32) -> i32) -> Result<String, Error> {
        unsafe {
            let mut count = 0;
            let hr = f(0, std::ptr::null_mut(), &mut count);
            Error::map_status(hr)?;
            let mut buf = Vec::with_capacity(count as usize);
            let len = count;
            let hr = f(len, buf.as_mut_ptr(), &mut count);
            Error::map_if(hr, || {
                buf.set_len(count as usize);
                let len = wstrnlen(&buf);
                String::from_utf16_lossy(&buf[..len])
            })
        }
    }

    pub fn color_management_version(&self) -> Result<String, Error> {
        Self::loadstr(|c, p, ac| unsafe { self.ptr.GetColorManagementVersion(c, p, ac) })
    }

    pub fn device_manufacturer(&self) -> Result<String, Error> {
        Self::loadstr(|c, p, ac| unsafe { self.ptr.GetDeviceManufacturer(c, p, ac) })
    }

    pub fn device_models(&self) -> Result<String, Error> {
        Self::loadstr(|c, p, ac| unsafe { self.ptr.GetDeviceModels(c, p, ac) })
    }

    pub fn mime_types(&self) -> Result<String, Error> {
        Self::loadstr(|c, p, ac| unsafe { self.ptr.GetMimeTypes(c, p, ac) })
    }

    pub fn file_extensions(&self) -> Result<String, Error> {
        Self::loadstr(|c, p, ac| unsafe { self.ptr.GetFileExtensions(c, p, ac) })
    }

    pub fn supports_animation(&self) -> bool {
        unsafe {
            let mut b = 0;
            let hr = self.ptr.DoesSupportAnimation(&mut b);
            hr >= 0 && b != 0
        }
    }

    pub fn supports_chromakey(&self) -> bool {
        unsafe {
            let mut b = 0;
            let hr = self.ptr.DoesSupportChromakey(&mut b);
            hr >= 0 && b != 0
        }
    }

    pub fn supports_lossless(&self) -> bool {
        unsafe {
            let mut b = 0;
            let hr = self.ptr.DoesSupportLossless(&mut b);
            hr >= 0 && b != 0
        }
    }

    pub fn supports_multiframe(&self) -> bool {
        unsafe {
            let mut b = 0;
            let hr = self.ptr.DoesSupportMultiframe(&mut b);
            hr >= 0 && b != 0
        }
    }

    pub fn matches_mimetype(&self, s: impl AsRef<OsStr>) -> bool {
        let s = s.as_ref().to_wide_null();

        unsafe {
            let mut b = 0;
            let hr = self.ptr.MatchesMimeType(s.as_ptr(), &mut b);
            hr >= 0 && b != 0
        }
    }
}
