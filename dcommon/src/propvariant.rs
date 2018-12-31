use crate::minwindef::filetime::FileTime;
use crate::GUID;

use std::marker::PhantomData;
use std::ptr::NonNull;

use winapi::ctypes::c_void;
use winapi::shared::ntdef::{LPSTR, LPWSTR};
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::IDispatch;
use winapi::um::objidl::IStorage;
use winapi::um::objidlbase::IStream;
use winapi::um::unknwnbase::IUnknown;

#[derive(Copy, Clone)]
pub union PropVariant {
    pub pv: InnerPropVariant,
    pub dec: Decimal,
}

#[derive(Copy, Clone)]
pub struct InnerPropVariant {
    pub vartype: VarType,
    pub reserved1: u16,
    pub reserved2: u16,
    pub reserved3: u16,
    pub data: InnerPropVariantData,
}

#[derive(Copy, Clone)]
pub union InnerPropVariantData {
    pub c_val: i8,
    pub b_val: u8,
    pub i_val: i16,
    pub ui_val: u16,
    pub l_val: i32,
    pub ul_val: u32,
    pub int_val: i32,
    pub uint_val: u32,
    pub h_val: i64,
    pub uh_val: u64,
    pub flt_val: f32,
    pub dbl_val: f64,
    pub bool_val: i16,
    pub scode: i32,
    pub cy_val: i64,
    pub date: f64,
    pub filetime: FileTime,
    pub puuid: *mut GUID,
    pub pclipdata: *mut ClipData,
    pub bstr_val: BSTR,
    pub blob: Blob,
    pub psz_val: LPSTR,
    pub pwsz_val: LPWSTR,
    pub punk_val: *mut IUnknown,
    pub pdisp_val: *mut IDispatch,
    pub pstream: *mut IStream,
    pub pstorage: *mut IStorage,
    pub pversioned_stream: *mut VersionedStream,
    pub parray: *mut SafeArray,
    pub cac: CountedArray<i8>,
    pub caub: CountedArray<u8>,
    pub cai: CountedArray<i16>,
    pub caui: CountedArray<u16>,
    pub cal: CountedArray<i32>,
    pub caul: CountedArray<u32>,
    pub cah: CountedArray<i64>,
    pub cauh: CountedArray<u64>,
    pub caflt: CountedArray<f32>,
    pub cadbl: CountedArray<f64>,
    pub cabool: CountedArray<i16>,
    pub cascode: CountedArray<i32>,
    pub cacy: CountedArray<i64>,
    pub cadate: CountedArray<f64>,
    pub cafiletime: CountedArray<FileTime>,
    pub cauuid: CountedArray<GUID>,
    pub caclipdata: CountedArray<ClipData>,
    pub cabstr: CountedArray<BSTR>,
    pub calpstr: CountedArray<LPSTR>,
    pub calpwstr: CountedArray<LPWSTR>,
    pub capropvar: CountedArray<PropVariant>,
    pub pc_val: *mut i8,
    pub pb_val: *mut u8,
    pub pi_val: *mut i16,
    pub pui_val: *mut u16,
    pub pl_val: *mut i32,
    pub pul_val: *mut u32,
    pub pint_val: *mut i32,
    pub puint_val: *mut u32,
    pub pflt_val: *mut f32,
    pub pdbl_val: *mut f64,
    pub pbool_val: *mut i16,
    pub pdec_val: *mut Decimal,
    pub pscode: *mut i32,
    pub pcy_val: *mut i64,
    pub pdate: *mut f64,
    pub pbstr_val: *mut BSTR,
    pub ppunk_val: *mut *mut IUnknown,
    pub ppdisp_val: *mut *mut IDispatch,
    pub pparray: *mut *mut SafeArray,
    pub pvar_val: *mut PropVariant,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VarType(pub u16);

impl VarType {
    pub const EMPTY: VarType = VarType(0);
    pub const NULL: VarType = VarType(1);
    pub const I2: VarType = VarType(2);
    pub const I4: VarType = VarType(3);
    pub const R4: VarType = VarType(4);
    pub const R8: VarType = VarType(5);
    pub const CY: VarType = VarType(6);
    pub const DATE: VarType = VarType(7);
    pub const BSTR: VarType = VarType(8);
    pub const DISPATCH: VarType = VarType(9);
    pub const ERROR: VarType = VarType(10);
    pub const BOOL: VarType = VarType(11);
    pub const VARIANT: VarType = VarType(12);
    pub const UNKNOWN: VarType = VarType(13);
    pub const DECIMAL: VarType = VarType(14);
    pub const I1: VarType = VarType(16);
    pub const UI1: VarType = VarType(17);
    pub const UI2: VarType = VarType(18);
    pub const UI4: VarType = VarType(19);
    pub const I8: VarType = VarType(20);
    pub const UI8: VarType = VarType(21);
    pub const INT: VarType = VarType(22);
    pub const UINT: VarType = VarType(23);
    pub const VOID: VarType = VarType(24);
    pub const HRESULT: VarType = VarType(25);
    pub const PTR: VarType = VarType(26);
    pub const SAFEARRAY: VarType = VarType(27);
    pub const CARRAY: VarType = VarType(28);
    pub const USERDEFINED: VarType = VarType(29);
    pub const LPSTR: VarType = VarType(30);
    pub const LPWSTR: VarType = VarType(31);
    pub const RECORD: VarType = VarType(36);
    pub const INT_PTR: VarType = VarType(37);
    pub const UINT_PTR: VarType = VarType(38);
    pub const FILETIME: VarType = VarType(64);
    pub const BLOB: VarType = VarType(65);
    pub const STREAM: VarType = VarType(66);
    pub const STORAGE: VarType = VarType(67);
    pub const STREAMED_OBJECT: VarType = VarType(68);
    pub const STORED_OBJECT: VarType = VarType(69);
    pub const BLOB_OBJECT: VarType = VarType(70);
    pub const CF: VarType = VarType(71);
    pub const CLSID: VarType = VarType(72);
    pub const VERSIONED_STREAM: VarType = VarType(73);
    pub const BSTR_BLOB: VarType = VarType(0xfff);
    pub const VECTOR: VarType = VarType(0x1000);
    pub const ARRAY: VarType = VarType(0x2000);
    pub const BYREF: VarType = VarType(0x4000);
    pub const RESERVED: VarType = VarType(0x8000);
    pub const ILLEGAL: VarType = VarType(0xffff);
    pub const ILLEGALMASKED: VarType = VarType(0xfff);
    pub const TYPEMASK: VarType = VarType(0xfff);
}

impl std::ops::BitOr for VarType {
    type Output = VarType;
    fn bitor(self, rhs: VarType) -> VarType {
        VarType(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for VarType {
    type Output = VarType;
    fn bitand(self, rhs: VarType) -> VarType {
        VarType(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for VarType {
    type Output = VarType;
    fn bitxor(self, rhs: VarType) -> VarType {
        VarType(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for VarType {
    type Output = VarType;
    fn not(self) -> VarType {
        VarType(!self.0)
    }
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
pub struct SafeArray {
    pub dims: u16,
    pub features: u16,
    pub elements: u32,
    pub locks: u32,
    pub data: *mut c_void,
    pub bounds: [SafeArrayBound; 1],
    _rest: PhantomData<SafeArrayBound>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SafeArrayBound {
    pub elements: u32,
    pub lbound: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Decimal {
    pub reserved: u16,
    pub scale: u8,
    pub sign: u8,
    pub high: u32,
    pub low: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CountedArray<T> {
    pub elements: u32,
    pub data: *mut T,
}
