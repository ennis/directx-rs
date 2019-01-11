use crate::error::{Error, Status};
use crate::helpers::wide::CoTaskWString;

use com_wrapper::ComWrapper;
use winapi::shared::wtypesbase::LPOLESTR;
use winapi::um::objidlbase::IEnumString;
use wio::com::ComPtr;

pub mod custom;

#[derive(ComWrapper)]
#[com(debug)]
pub struct EnumString {
    ptr: ComPtr<IEnumString>,
}

impl EnumString {
    pub fn next_elem(&mut self) -> Result<Option<CoTaskWString>, Error> {
        use std::mem::replace;
        let mut buf = [None];
        Ok(match self.next_elems(&mut buf)? {
            0 => None,
            1 => replace(&mut buf[0], None),
            _ => panic!("Unexpected value returned from `IEnumString::Next`."),
        })
    }

    pub fn next_elems(&mut self, strings: &mut [Option<CoTaskWString>]) -> Result<usize, Error> {
        assert!(strings.len() < std::u32::MAX as usize);

        unsafe {
            let ptr: *mut Option<CoTaskWString> = strings.as_mut_ptr();
            let ptr: *mut LPOLESTR = ptr as _;

            let mut fetched = 0;
            let hr = self.ptr.Next(strings.len() as u32, ptr, &mut fetched);

            if hr < 0 {
                for s in strings.iter_mut().take(fetched as usize) {
                    *s = None;
                }
            }

            Error::map(hr, fetched as usize)
        }
    }

    pub fn skip(&mut self, count: u32) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.Skip(count);
            Error::map_status(hr)
        }
    }

    pub fn reset(&mut self) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.Reset();
            Error::map_status(hr)
        }
    }

    pub fn try_clone(&self) -> Result<EnumString, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.Clone(&mut ptr);
            Error::map_if(hr, || Self::from_raw(ptr))
        }
    }
}

impl Iterator for EnumString {
    type Item = CoTaskWString;
    fn next(&mut self) -> Option<CoTaskWString> {
        self.next_elem().ok().unwrap_or(None)
    }
}
