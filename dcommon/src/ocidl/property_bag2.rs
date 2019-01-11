use winapi::um::ocidl::IPropertyBag2;
use wio::com::ComPtr;

pub mod proptype;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct PropertyBag2 {
    ptr: ComPtr<IPropertyBag2>,
}

impl PropertyBag2 {
    
}

