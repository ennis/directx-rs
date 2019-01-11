use crate::idltypes::vartype::VarType;
use crate::{Error, Status};

use std::marker::PhantomData;

use winapi::ctypes::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SafeArray {
    pub dims: u16,
    pub features: u16,
    pub element_size: u32,
    pub locks: u32,
    pub data: *mut c_void,
    pub bounds: [SafeArrayBound; 1],
    _rest: PhantomData<SafeArrayBound>,
}

impl SafeArray {
    pub unsafe fn create(vt: VarType, dims: &[SafeArrayBound]) -> *mut SafeArray {
        SafeArrayCreate(vt, dims.len() as u32, dims.as_ptr() as _)
    }

    pub unsafe fn destroy(array: *mut SafeArray) -> Result<Status, Error> {
        Error::map_status(SafeArrayDestroy(array))
    }

    pub unsafe fn get_elem<T>(array: *mut SafeArray, index: &[i32]) -> Result<T, Error>
    where
        T: Copy,
    {
        assert_eq!(
            (*array).dims as usize,
            index.len(),
            "Wrong number of dimensions"
        );
        assert_eq!(
            (*array).element_size as usize,
            std::mem::size_of::<T>(),
            "T is the wrong size"
        );
        let mut data = std::mem::zeroed();
        let p = (&mut data) as *mut T as *mut c_void;
        let hr = SafeArrayGetElement(array, index.as_ptr(), p);
        Error::map(hr, data)
    }

    pub unsafe fn put_elem<T>(
        array: *mut SafeArray,
        index: &[i32],
        data: &T,
    ) -> Result<Status, Error>
    where
        T: Copy,
    {
        assert_eq!(
            (*array).dims as usize,
            index.len(),
            "Wrong number of dimensions"
        );
        assert_eq!(
            (*array).element_size as usize,
            std::mem::size_of::<T>(),
            "T is the wrong size"
        );
        let hr = SafeArrayPutElement(array, index.as_ptr(), data as *const T as *const c_void);
        Error::map_status(hr)
    }

    pub unsafe fn lock(array: *mut SafeArray) -> Result<Status, Error> {
        Error::map_status(SafeArrayLock(array))
    }

    pub unsafe fn unlock(array: *mut SafeArray) -> Result<Status, Error> {
        Error::map_status(SafeArrayUnlock(array))
    }

    pub unsafe fn slice<'a, T>(array: *mut SafeArray, len: usize) -> &'a [T]
    where
        T: Copy,
    {
        assert_eq!(
            (*array).element_size as usize,
            std::mem::size_of::<T>(),
            "T is the wrong size"
        );
        let ptr = (*array).data as *const T;
        std::slice::from_raw_parts(ptr, len)
    }

    pub unsafe fn slice_mut<'a, T>(array: *mut SafeArray, len: usize) -> &'a mut [T]
    where
        T: Copy,
    {
        assert_eq!(
            (*array).element_size as usize,
            std::mem::size_of::<T>(),
            "T is the wrong size"
        );
        let ptr = (*array).data as *mut T;
        std::slice::from_raw_parts_mut(ptr, len)
    }

    pub unsafe fn access<T>(array: *mut SafeArray) -> Result<*mut T, Error>
    where
        T: Copy,
    {
        let mut data = std::ptr::null_mut();
        let hr = SafeArrayAccessData(array, &mut data);
        Error::map(hr, data as *mut T)
    }

    pub unsafe fn unaccess(array: *mut SafeArray) -> Result<Status, Error> {
        Error::map_status(SafeArrayUnaccessData(array))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SafeArrayBound {
    pub elements: u32,
    pub lbound: u32,
}

impl SafeArrayBound {
    pub fn new(c: u32) -> SafeArrayBound {
        SafeArrayBound {
            elements: c,
            lbound: 0,
        }
    }
}

// The import lib is already added from winapi
extern "system" {
    fn SafeArrayCreate(vt: VarType, dims: u32, bounds: *const SafeArrayBound) -> *mut SafeArray;
    fn SafeArrayDestroy(array: *mut SafeArray) -> i32;

    fn SafeArrayGetElement(array: *mut SafeArray, indices: *const i32, data: *mut c_void) -> i32;
    fn SafeArrayPutElement(array: *mut SafeArray, indices: *const i32, data: *const c_void) -> i32;

    fn SafeArrayLock(array: *mut SafeArray) -> i32;
    fn SafeArrayUnlock(array: *mut SafeArray) -> i32;

    fn SafeArrayAccessData(array: *mut SafeArray, data: *mut *mut c_void) -> i32;
    fn SafeArrayUnaccessData(array: *mut SafeArray) -> i32;
}

#[cfg(test)]
#[test]
fn basic_safearray_use() {
    unsafe {
        let dims = [SafeArrayBound::new(10)];
        let ary = SafeArray::create(VarType::I4, &dims);

        assert!(!ary.is_null());
        assert_eq!((*ary).dims, 1);
        assert_eq!((*ary).bounds, dims);

        let res = SafeArray::lock(ary);

        assert_eq!(res, Ok(Status::OK));
        assert_eq!((*ary).locks, 1);
        assert!(!(*ary).data.is_null());

        let data: &mut [i32] = SafeArray::slice_mut(ary, 10);

        assert_eq!(data.len(), 10);

        for (i, v) in data.iter_mut().enumerate() {
            *v = i as i32;
        }

        let res = SafeArray::unlock(ary);

        assert_eq!(res, Ok(Status::OK));
        assert_eq!((*ary).locks, 0);

        for i in 0..10 {
            let v: i32 = SafeArray::get_elem(ary, &[i]).unwrap();
            assert_eq!(v, i);
        }

        assert_eq!(
            SafeArray::get_elem::<i32>(ary, &[10]),
            Err(Error::DISP_BADINDEX)
        );
    }
}
