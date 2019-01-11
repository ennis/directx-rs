use crate::bitmap_codec_info::BitmapCodecInfo;
use crate::descriptions::BitmapPattern;

use com_wrapper::ComWrapper;
use dcommon::objidl::Stream;
use dcommon::Error;
use winapi::um::wincodec::IWICBitmapDecoderInfo;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct BitmapDecoderInfo {
    ptr: ComPtr<IWICBitmapDecoderInfo>,
}

impl BitmapDecoderInfo {
    pub fn patterns(&self) -> Result<Vec<BitmapPattern>, Error> {
        unsafe {
            let mut count = 0;
            let hr = self.ptr.GetPatterns(0, 0 as _, 0 as _, &mut count);
            Error::map_status(hr)?;
            let mut buf = Vec::with_capacity(count as usize);
            let len = count;
            let mut recv = 0;
            let hr = self
                .ptr
                .GetPatterns(len, buf.as_mut_ptr(), &mut recv, &mut count);
            Error::map_if(hr, || {
                buf.set_len(recv as usize);
                buf.iter().map(|p| BitmapPattern::from_raw(p)).collect()
            })
        }
    }

    pub fn matches_pattern(&self, stream: &mut Stream) -> Result<bool, Error> {
        unsafe {
            let mut b = 0;
            let hr = self.ptr.MatchesPattern(stream.get_raw(), &mut b);
            Error::map(hr, b != 0)
        }
    }
}

impl std::ops::Deref for BitmapDecoderInfo {
    type Target = BitmapCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for BitmapDecoderInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper_mut(self) }
    }
}
