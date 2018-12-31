use crate::Error;

use com_impl::Refcount;
use com_impl::VTable;
use winapi::ctypes::c_void;
use winapi::shared::winerror::{E_FAIL, S_OK};
use winapi::um::objidlbase::{ISequentialStream, ISequentialStreamVtbl};

pub trait SequentialStream: 'static {
    fn read(&mut self, buf: &mut [u8]) -> Result<u32, Error>;
    fn write(&mut self, buf: &[u8]) -> Result<u32, Error>;
}

#[repr(C)]
#[derive(ComImpl)]
pub struct CustomSequentialStream<S>
where
    S: SequentialStream,
{
    vtbl: VTable<ISequentialStreamVtbl>,
    refcount: Refcount,
    stream: S,
}

#[com_impl]
unsafe impl<S> ISequentialStream for CustomSequentialStream<S>
where
    S: SequentialStream,
{
    #[panic(result = "E_FAIL")]
    unsafe fn read(&mut self, pv: *mut c_void, cb: u32, pcbRead: *mut u32) -> i32 {
        *pcbRead = 0;

        let slice = std::slice::from_raw_parts_mut(pv as *mut u8, cb as usize);
        match self.stream.read(slice) {
            Ok(cbread) => {
                *pcbRead = cbread;
                S_OK
            }
            Err(e) => e.0,
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn write(&mut self, pv: *const c_void, cb: u32, pcbWritten: *mut u32) -> i32 {
        *pcbWritten = 0;

        let slice = std::slice::from_raw_parts(pv as *const u8, cb as usize);
        match self.stream.write(slice) {
            Ok(cbwritten) => {
                *pcbWritten = cbwritten;
                S_OK
            }
            Err(e) => e.0,
        }
    }
}
