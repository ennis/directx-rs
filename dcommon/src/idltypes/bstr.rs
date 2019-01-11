use winapi::shared::wtypes::BSTR;

#[repr(transparent)]
pub struct BStr {
    bstr: BSTR,
}

impl BStr {
    pub fn new(s: &str) -> BStr {
        let len = s.encode_utf16().count();
        assert!(len <= std::u32::MAX as usize);

        unsafe {
            let bstr = SysAllocStringLen(0 as _, len as u32);
            let slice = std::slice::from_raw_parts_mut(bstr, len);

            for (dst, src) in slice.iter_mut().zip(s.encode_utf16()) {
                *dst = src;
            }

            BStr { bstr }
        }
    }

    pub unsafe fn from_raw(bstr: BSTR) -> Self {
        assert!(!bstr.is_null());
        BStr { bstr }
    }

    pub unsafe fn get_raw(&self) -> BSTR {
        self.bstr
    }

    pub unsafe fn into_raw(self) -> BSTR {
        let bstr = self.get_raw();
        std::mem::forget(self);
        bstr
    }

    pub fn len(&self) -> usize {
        unsafe { SysStringLen(self.bstr) as usize }
    }

    pub fn as_slice(&self) -> &[u16] {
        unsafe { std::slice::from_raw_parts(self.bstr, self.len()) }
    }
}

impl PartialEq for BStr {
    fn eq(&self, other: &BStr) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Drop for BStr {
    fn drop(&mut self) {
        unsafe {
            let hr = SysFreeString(self.bstr);
            assert!(hr >= 0);
        }
    }
}

extern "system" {
    fn SysAllocStringLen(str_in: *const u16, ui: u32) -> BSTR;
    fn SysFreeString(b: BSTR) -> i32;

    fn SysStringLen(b: BSTR) -> u32;
}
