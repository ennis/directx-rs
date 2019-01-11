use crate::idltypes::{decimal::Decimal, safearray::SafeArray, vartype::VarType};

use winapi::ctypes::c_void;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::IDispatch;
use winapi::um::unknwnbase::IUnknown;

#[repr(C)]
#[derive(Copy, Clone)]
pub union Variant {
    pub v: InnerVariant,
    pub dec: Decimal,
}

impl Variant {
    pub fn vartype(&self) -> VarType {
        unsafe { self.v.vartype }
    }

    pub fn new_i8(val: i8) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::I1;
            v.v.data.c_val = val;
            v
        }
    }
    
    pub fn new_u8(val: u8) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::UI1;
            v.v.data.b_val = val;
            v
        }
    }
    
    pub fn new_i16(val: i16) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::I2;
            v.v.data.i_val = val;
            v
        }
    }
    
    pub fn new_u16(val: u16) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::UI2;
            v.v.data.ui_val = val;
            v
        }
    }
    
    pub fn new_i32(val: i32) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::I4;
            v.v.data.l_val = val;
            v
        }
    }
    
    pub fn new_u32(val: u32) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::UI4;
            v.v.data.ul_val = val;
            v
        }
    }
    
    pub fn new_i64(val: i64) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::I8;
            v.v.data.ll_val = val;
            v
        }
    }
    
    pub fn new_u64(val: u64) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::UI8;
            v.v.data.ull_val = val;
            v
        }
    }
    
    pub fn new_f32(val: f32) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::R4;
            v.v.data.flt_val = val;
            v
        }
    }

    pub fn new_f64(val: f64) -> Variant {
        unsafe {
            let mut v: Variant = std::mem::zeroed();
            v.v.vartype = VarType::R8;
            v.v.data.dbl_val = val;
            v
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InnerVariant {
    pub vartype: VarType,
    pub reserved1: u16,
    pub reserved2: u16,
    pub reserved3: u16,
    pub data: InnerVariantData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union InnerVariantData {
    /// VarType::I8
    pub ll_val: i64,
    /// VarType::I4
    pub l_val: i32,
    /// VarType::UI1
    pub b_val: u8,
    /// VarType::I2
    pub i_val: i16,
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
    /// VarType::BSTR
    pub bstr_val: BSTR,
    /// VarType::UNKNOWN
    pub punk_val: *mut IUnknown,
    /// VarType::DISPATCH
    pub pdisp_val: *mut IDispatch,
    /// VarType::ARRAY
    pub parray: *mut SafeArray,
    /// VarType::BYREF|UI1
    pub pb_val: *mut u8,
    /// VarType::BYREF|I2
    pub pi_val: *mut i16,
    /// VarType::BYREF|I4
    pub pl_val: *mut i32,
    /// VarType::BYREF|I8
    pub pll_val: *mut i64,
    /// VarType::BYREF|R4
    pub pflt_val: *mut f32,
    /// VarType::BYREF|R8
    pub pdbl_val: *mut f64,
    /// VarType::BYREF|BOOL
    pub pbool_val: *mut i16,
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
    /// VarType::BYREF|VARIANT
    pub pvar_val: *mut Variant,
    /// VarType::BYREF
    pub byref: *mut c_void,
    /// VarType::I1
    pub c_val: i8,
    /// VarType::UI2
    pub ui_val: u16,
    /// VarType::UI4
    pub ul_val: u32,
    /// VarType::UI8
    pub ull_val: u64,
    /// VarType::INT
    pub int_val: i32,
    /// VarType::UINT
    pub uint_val: u32,
    /// VarType::BYREF|DECIMAL
    pub pdec_val: *mut Decimal,
    /// VarType::BYREF|I1
    pub pc_val: *mut i8,
    /// VarType::BYREF|UI2
    pub pui_val: *mut u16,
    /// VarType::BYREF|UI4
    pub pul_val: *mut u32,
    /// VarType::BYREF|UI8
    pub pull_val: *mut u64,
    /// VarType::BYREF|INT
    pub pint_val: *mut i32,
    /// VarType::BYREF|UINT
    pub puint_val: *mut u32,
}
