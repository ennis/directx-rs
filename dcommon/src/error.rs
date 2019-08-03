use std::ffi::OsString;

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{LANG_NEUTRAL, LPWSTR, MAKELANGID, SUBLANG_NEUTRAL};
use winapi::shared::winerror::E_FAIL;
use winapi::shared::winerror::{HRESULT, HRESULT_FROM_WIN32, SUCCEEDED};
use winapi::um::winbase::{
    FormatMessageW, LocalFree, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS,
};
use wio::wide::FromWide;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
/// Represents the various failure states that may occur when calling APIs in Windows.
pub struct Error(pub HRESULT);

impl Error {
    #[inline]
    /// Returns Ok if the HRESULT represents a success state, otherwise returns
    /// Err with the HRESULT value.
    pub fn map<T>(hr: HRESULT, success_value: T) -> Result<T, Error> {
        if SUCCEEDED(hr) {
            Ok(success_value)
        } else {
            Err(Error(hr))
        }
    }

    #[inline]
    /// Returns Ok with the HRESULT if the operation was a success,
    /// and Err with the HRESULT if the operation failed.
    pub fn map_status(hr: HRESULT) -> Result<Status, Error> {
        if SUCCEEDED(hr) {
            Ok(Status(hr))
        } else {
            Err(Error(hr))
        }
    }

    #[inline]
    /// Returns Ok with the value returned by the function if the HRESULT
    /// represents a success state, otherwise returns Err with the HRESULT
    /// value.
    pub fn map_if<F, T>(hr: HRESULT, if_success: F) -> Result<T, Error>
    where
        F: FnOnce() -> T,
    {
        if SUCCEEDED(hr) {
            Ok(if_success())
        } else {
            Err(Error(hr))
        }
    }

    #[inline]
    /// Creates an Error from a Win32 error code, such as a raw error code
    /// from `std::io::Error`.
    pub fn from_win32(err: DWORD) -> Error {
        Error(HRESULT_FROM_WIN32(err))
    }

    #[inline]
    /// Gets a formatted error string for the HRESULT value contained within.
    pub fn message(&self) -> String {
        format_err(self.0)
    }
}

impl From<HRESULT> for Error {
    #[inline]
    fn from(hr: HRESULT) -> Error {
        assert!(hr < 0);
        Error(hr)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(err: std::io::Error) -> Error {
        let hr = err
            .raw_os_error()
            .map(|i| i as _)
            .map(HRESULT_FROM_WIN32)
            .unwrap_or_else(|| {
                err.get_ref()
                    .and_then(|e| e.downcast_ref::<Error>())
                    .map(|e| e.0)
                    .unwrap_or(E_FAIL)
            });
        Error(hr)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        struct HexErr(i32);
        impl std::fmt::Debug for HexErr {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(fmt, "0x{:x}", self.0 as u32)
            }
        }
        fmt.debug_tuple("Error")
            .field(&HexErr(self.0))
            .field(&self.message())
            .finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&self.message())
    }
}

impl From<Error> for std::io::Error {
    fn from(error: Error) -> std::io::Error {
        use std::io::{Error as IoError, ErrorKind};
        use winapi::shared::winerror::*;
        const ERROR_BIT: i32 = -1 << 31;
        const WIN32_BITS: i32 = ERROR_BIT | (FACILITY_WIN32 << 16);
        const CATMASK: i32 = 0x7FFF0000;
        const ERRMASK: i32 = 0x0000FFFF;
        let code = 'code: loop {
            let err = error.0;
            if err & ERROR_BIT == 0 {
                break 'code 0; // ERROR: SUCCESS :D
            }
            if err & CATMASK == WIN32_BITS {
                break 'code err & ERRMASK;
            }
            return IoError::new(ErrorKind::Other, error);
        };
        IoError::from_raw_os_error(code)
    }
}

impl std::error::Error for Error {}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
/// Represents the various success states that may occur when calling APIs in Windows.
pub struct Status(pub HRESULT);

impl Status {
    #[inline]
    /// Gets a formatted error string for the HRESULT value contained within.
    pub fn message(&self) -> String {
        format_err(self.0)
    }
}

impl From<HRESULT> for Status {
    #[inline]
    fn from(hr: HRESULT) -> Status {
        assert!(hr >= 0);
        Status(hr)
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_tuple("Error")
            .field(&self.0)
            .field(&self.message())
            .finish()
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&self.message())
    }
}

fn format_err(hr: HRESULT) -> String {
    unsafe {
        let flags = FORMAT_MESSAGE_FROM_SYSTEM
            | FORMAT_MESSAGE_ALLOCATE_BUFFER
            | FORMAT_MESSAGE_IGNORE_INSERTS;

        let mut msg: LPWSTR = std::ptr::null_mut();
        let len = FormatMessageW(
            flags,
            std::ptr::null_mut(),
            hr as u32,
            MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL) as u32,
            (&mut msg) as *mut _ as *mut _,
            0,
            std::ptr::null_mut(),
        );

        if len == 0 {
            return format!("Unknown Error 0x{:x}", hr);
        }

        let os = OsString::from_wide_ptr(msg, len as usize);
        LocalFree(msg as *mut _);

        os.to_string_lossy().into_owned()
    }
}

mod fixme;

#[doc(hidden)]
pub mod common;
#[doc(hidden)]
pub mod d2d1;
#[doc(hidden)]
pub mod d3d11;
#[doc(hidden)]
pub mod disp;
#[doc(hidden)]
pub mod dwrite;
#[doc(hidden)]
pub mod dxgi;
#[doc(hidden)]
pub mod wic;
#[doc(hidden)]
pub mod win32;

#[cfg(test)]
#[test]
fn roundtrip_error_io() {
    fn rt(e: Error) {
        let err: std::io::Error = e.into();
        let err: crate::Error = err.into();
        assert_eq!(err, e);
    }

    rt(Error::FAIL);
    rt(Error::WIN32_FILE_NOT_FOUND);
    rt(Error::D2D_WRONG_STATE);
    rt(Error::WIC_PROPERTYSIZE);
}
