use winapi::um::wincodec::IWICColorContext;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct ColorContext {
    ptr: ComPtr<IWICColorContext>,
}
