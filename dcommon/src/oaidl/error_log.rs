use winapi::um::oaidl::IErrorLog;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send)]
pub struct ErrorLog {
    ptr: ComPtr<IErrorLog>,
}

impl ErrorLog {
    
}
