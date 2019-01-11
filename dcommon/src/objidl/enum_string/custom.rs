use super::EnumString;
use crate::helpers::CoTaskWString;
use crate::{Error, Status};

use std::iter::Fuse;

use com_impl::Refcount;
use com_impl::VTable;
use com_wrapper::ComWrapper;
use winapi::shared::winerror::E_FAIL;
use winapi::shared::wtypesbase::LPOLESTR;
use winapi::um::objidlbase::{IEnumString, IEnumStringVtbl};

#[repr(C)]
#[derive(ComImpl)]
pub struct CustomEnumString<I>
where
    I: Iterator<Item = CoTaskWString> + 'static,
{
    vtbl: VTable<IEnumStringVtbl>,
    refcount: Refcount,
    iter: Fuse<I>,
    original: Option<Fuse<I>>,
    clone_fn: Option<fn(&Fuse<I>) -> Fuse<I>>,
}

impl<I> CustomEnumString<I>
where
    I: Iterator<Item = CoTaskWString> + Clone + 'static,
{
    pub fn create(iter: I) -> EnumString {
        let iter = iter.fuse();
        let orig = Some(iter.clone());
        let clone_fn: Option<fn(&Fuse<I>) -> Fuse<I>> = Some(<Fuse<I> as Clone>::clone);
        unsafe {
            let raw = Self::create_raw(iter, orig, clone_fn);
            let raw: *mut CustomEnumString<I> = raw;
            let raw: *mut IEnumString = raw as *mut _;
            EnumString::from_raw(raw)
        }
    }
}

impl<I> CustomEnumString<I>
where
    I: Iterator<Item = CoTaskWString> + 'static,
{
    pub fn create_without_clone(iter: I) -> EnumString {
        let iter = iter.fuse();
        unsafe {
            let raw = Self::create_raw(iter, None, None);
            let raw: *mut CustomEnumString<I> = raw;
            let raw: *mut IEnumString = raw as *mut _;
            EnumString::from_raw(raw)
        }
    }
}

#[com_impl]
unsafe impl<I> IEnumString for CustomEnumString<I>
where
    I: Iterator<Item = CoTaskWString> + 'static,
{
    #[panic(result = "E_FAIL")]
    unsafe fn next(&mut self, ce: u32, out: *mut LPOLESTR, pce: *mut u32) -> i32 {
        let mut tot = 0;
        for i in 0..ce {
            let s = match self.iter.next() {
                Some(s) => s,
                None => break,
            };
            *out.offset(i as isize) = s.into_raw();
            tot += 1;
        }
        *pce = tot;
        if tot < ce {
            Status::FALSE.0
        } else {
            Status::OK.0
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn clone(&self, other: *mut *mut IEnumString) -> i32 {
        let clone_fn = match self.clone_fn {
            Some(f) => f,
            None => return Error::NOTIMPL.0,
        };

        let iter = clone_fn(&self.iter);
        let original = self.original.as_ref().map(clone_fn);

        let raw = Self::create_raw(iter, original, Some(clone_fn));
        let raw: *mut CustomEnumString<I> = raw;
        let raw: *mut IEnumString = raw as *mut _;
        *other = raw;

        Status::OK.0
    }

    #[panic(result = "E_FAIL")]
    unsafe fn reset(&mut self) -> i32 {
        let orig = match &self.original {
            Some(orig) => orig,
            None => return Error::NOTIMPL.0,
        };
        let clone_fn = match self.clone_fn {
            Some(f) => f,
            None => return Error::NOTIMPL.0,
        };

        self.iter = clone_fn(orig);
        Status::OK.0
    }

    #[panic(result = "E_FAIL")]
    unsafe fn skip(&mut self, count: u32) -> i32 {
        for _ in 0..count {
            self.iter.next();
        }
        Status::OK.0
    }
}
