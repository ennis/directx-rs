use crate::idltypes::safearray::SafeArray;
use crate::idltypes::decimal::Decimal;
use crate::idltypes::vartype::VarType;
use crate::minwindef::filetime::FileTime;
use crate::GUID;

use std::ptr::NonNull;

use winapi::shared::ntdef::{LPSTR, LPWSTR};
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::IDispatch;
use winapi::um::objidl::IStorage;
use winapi::um::objidlbase::IStream;
use winapi::um::unknwnbase::IUnknown;

#[repr(C)]
#[derive(Copy, Clone)]
pub union PropVariant {
    pub pv: InnerPropVariant,
    pub dec: Decimal,
}

impl PropVariant {
    pub fn vartype(&self) -> VarType {
        unsafe { self.pv.vartype }
    }

    pub fn new_i8(val: i8) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::I1;
            v.pv.data.c_val = val;
            v
        }
    }

    pub fn new_u8(val: u8) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::UI1;
            v.pv.data.b_val = val;
            v
        }
    }

    pub fn new_i16(val: i16) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::I2;
            v.pv.data.i_val = val;
            v
        }
    }

    pub fn new_u16(val: u16) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::UI2;
            v.pv.data.ui_val = val;
            v
        }
    }

    pub fn new_i32(val: i32) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::I4;
            v.pv.data.l_val = val;
            v
        }
    }

    pub fn new_u32(val: u32) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::UI4;
            v.pv.data.ul_val = val;
            v
        }
    }

    pub fn new_i64(val: i64) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::I8;
            v.pv.data.h_val = val;
            v
        }
    }

    pub fn new_u64(val: u64) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::UI8;
            v.pv.data.uh_val = val;
            v
        }
    }

    pub fn new_f32(val: f32) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::R4;
            v.pv.data.flt_val = val;
            v
        }
    }

    pub fn new_f64(val: f64) -> PropVariant {
        unsafe {
            let mut v: PropVariant = std::mem::zeroed();
            v.pv.vartype = VarType::R8;
            v.pv.data.dbl_val = val;
            v
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InnerPropVariant {
    pub vartype: VarType,
    pub reserved1: u16,
    pub reserved2: u16,
    pub reserved3: u16,
    pub data: InnerPropVariantData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union InnerPropVariantData {
    /// VarType::I1
    pub c_val: i8,
    /// VarType::UI1
    pub b_val: u8,
    /// VarType::I2
    pub i_val: i16,
    /// VarType::UI2
    pub ui_val: u16,
    /// VarType::I4
    pub l_val: i32,
    /// VarType::UI4
    pub ul_val: u32,
    /// VarType::INT
    pub int_val: i32,
    /// VarType::UINT
    pub uint_val: u32,
    /// VarType::I8
    pub h_val: i64,
    /// VarType::UI8
    pub uh_val: u64,
    /// VarType::R4
    pub flt_val: f32,
    /// VarType::R8
    pub dbl_val: f64,
    /// VarType::BOOL
    pub bool_val: i16,
    /// VarType::ERROR
    pub scode: i32,
    /// VarType::CY
    pub cy_val: i64,
    /// VarType::DATE
    pub date: f64,
    /// VarType::FILETIME
    pub filetime: FileTime,
    /// VarType::CLSID
    pub puuid: *mut GUID,
    /// VarType::CF
    pub pclipdata: *mut ClipData,
    /// VarType::BSTR
    pub bstr_val: BSTR,
    /// VarType::BLOB
    pub blob: Blob,
    /// VarType::LPSTR
    pub psz_val: LPSTR,
    /// VarType::LPWSTR
    pub pwsz_val: LPWSTR,
    /// VarType::UNKNOWN
    pub punk_val: *mut IUnknown,
    /// VarType::DISPATCH
    pub pdisp_val: *mut IDispatch,
    /// VarType::STREAM, VarType::STREAMED_OBJECT
    pub pstream: *mut IStream,
    /// VarType::STORAGE
    pub pstorage: *mut IStorage,
    /// VarType::VERSIONED_STREAM
    pub pversioned_stream: *mut VersionedStream,
    /// VarType::ARRAY
    pub parray: *mut SafeArray,
    /// VarType::VECTOR|I1
    pub ca_c: CountedArray<i8>,
    /// VarType::VECTOR|UI1
    pub ca_ub: CountedArray<u8>,
    /// VarType::VECTOR|I2
    pub ca_i: CountedArray<i16>,
    /// VarType::VECTOR|UI2
    pub ca_ui: CountedArray<u16>,
    /// VarType::VECTOR|I4
    pub ca_l: CountedArray<i32>,
    /// VarType::VECTOR|UI4
    pub ca_ul: CountedArray<u32>,
    /// VarType::VECTOR|I8
    pub ca_h: CountedArray<i64>,
    /// VarType::VECTOR|UI8
    pub ca_uh: CountedArray<u64>,
    /// VarType::VECTOR|R4
    pub ca_flt: CountedArray<f32>,
    /// VarType::VECTOR|R8
    pub ca_dbl: CountedArray<f64>,
    /// VarType::VECTOR|BOOL
    pub ca_bool: CountedArray<i16>,
    /// VarType::VECTOR|ERROR
    pub ca_scode: CountedArray<i32>,
    /// VarType::VECTOR|CY
    pub ca_cy: CountedArray<i64>,
    /// VarType::VECTOR|DATE
    pub ca_date: CountedArray<f64>,
    /// VarType::VECTOR|FILETIME
    pub ca_filetime: CountedArray<FileTime>,
    /// VarType::VECTOR|CLSID
    pub ca_uuid: CountedArray<GUID>,
    /// VarType::VECTOR|CF
    pub ca_clipdata: CountedArray<ClipData>,
    /// VarType::VECTOR|BSTR
    pub ca_bstr: CountedArray<BSTR>,
    /// VarType::VECTOR|LPSTR
    pub ca_lpstr: CountedArray<LPSTR>,
    /// VarType::VECTOR|LPWSTR
    pub ca_lpwstr: CountedArray<LPWSTR>,
    /// VarType::VECTOR|PropVariant
    pub ca_propvar: CountedArray<PropVariant>,
    /// VarType::BYREF|I1
    pub pc_val: *mut i8,
    /// VarType::BYREF|UI1
    pub pb_val: *mut u8,
    /// VarType::BYREF|I2
    pub pi_val: *mut i16,
    /// VarType::BYREF|UI2
    pub pui_val: *mut u16,
    /// VarType::BYREF|I4
    pub pl_val: *mut i32,
    /// VarType::BYREF|UI4
    pub pul_val: *mut u32,
    /// VarType::BYREF|INT
    pub pint_val: *mut i32,
    /// VarType::BYREF|UINT
    pub puint_val: *mut u32,
    /// VarType::BYREF|R4
    pub pflt_val: *mut f32,
    /// VarType::BYREF|R8
    pub pdbl_val: *mut f64,
    /// VarType::BYREF|BOOL
    pub pbool_val: *mut i16,
    /// VarType::BYREF|DECIMAL
    pub pdec_val: *mut Decimal,
    /// VarType::BYREF|ERROR
    pub pscode: *mut i32,
    /// VarType::BYREF|CY
    pub pcy_val: *mut i64,
    /// VarType::BYREF|DATE
    pub pdate: *mut f64,
    /// VarType::BYREF|BSTR
    pub pbstr_val: *mut BSTR,
    /// VarType::BYREF|UNKNOWN
    pub ppunk_val: *mut *mut IUnknown,
    /// VarType::BYREF|DISPATCH
    pub ppdisp_val: *mut *mut IDispatch,
    /// VarType::BYREF|ARRAY
    pub pparray: *mut *mut SafeArray,
    /// VarType::BYREF|PropVariant
    pub pvar_val: *mut PropVariant,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClipData {
    pub size: u32,
    pub format: i32,
    pub data: Option<NonNull<u8>>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Blob {
    pub size: u32,
    pub data: Option<NonNull<u8>>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VersionedStream {
    pub version: GUID,
    pub stream: *mut IStream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CountedArray<T> {
    pub elements: u32,
    pub data: *mut T,
}
