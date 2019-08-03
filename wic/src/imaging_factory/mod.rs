use com_wrapper::ComWrapper;
use winapi::um::wincodec::IWICImagingFactory;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct ImagingFactory {
    ptr: ComPtr<IWICImagingFactory>,
}
