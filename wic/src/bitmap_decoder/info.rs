use winapi::um::wincodec::IWICBitmapDecoderInfo;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
pub struct BitmapDecoderInfo {
    ptr: ComPtr<IWICBitmapDecoderInfo>,
}

impl BitmapDecoderInfo {
    
}
