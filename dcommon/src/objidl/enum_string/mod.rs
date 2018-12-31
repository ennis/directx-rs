use wio::com::ComPtr;
use winapi::um::objidlbase::IEnumString,

pub struct EnumString {
    ptr: ComPtr<IEnumString>,
}
