use crate::bitmap_codec_info::BitmapCodecInfo;

use com_wrapper::ComWrapper;
use winapi::um::wincodec::IWICBitmapEncoderInfo;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct BitmapEncoderInfo {
    ptr: ComPtr<IWICBitmapEncoderInfo>,
}

impl std::ops::Deref for BitmapEncoderInfo {
    type Target = BitmapCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for BitmapEncoderInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper_mut(self) }
    }
}
