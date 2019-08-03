use com_wrapper::ComWrapper;
use winapi::um::wincodec::IWICBitmapFrameEncode;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct BitmapFrameEncode {
    ptr: ComPtr<IWICBitmapFrameEncode>,
}
