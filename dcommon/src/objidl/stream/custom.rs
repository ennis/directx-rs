use crate::objidl::sequential_stream::custom::SequentialStream;
use crate::Error;

use com_impl::Refcount;
use com_impl::VTable;
use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::winerror::{E_FAIL, S_OK};
use winapi::um::objidlbase::{ISequentialStream, ISequentialStreamVtbl, IStream, IStreamVtbl};

pub trait Stream: SequentialStream + 'static {
    fn try_clone(&self) -> Result<super::Stream, Error>;
}

#[repr(C)]
#[derive(ComImpl)]
pub struct CustomStream<S>
where
    S: Stream,
{
    vtbl: VTable<IStreamVtbl>,
    refcount: Refcount,
    stream: S,
}

#[com_impl]
unsafe impl<S> ISequentialStream for CustomStream<S>
where
    S: Stream,
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

#[com_impl]
unsafe impl<S> IStream for CustomStream<S>
where
    S: Stream,
{
    #[panic(result = "E_FAIL")]
    unsafe fn clone(&self, ppstm: *mut *mut IStream) -> i32 {
        *ppstm = std::ptr::null_mut();
        let stream = match self.stream.try_clone() {
            Ok(stream) => stream,
            Err(e) => return e.0,
        };

        *ppstream = stream.into_raw();
        S_OK
    }
}
