use crate::idltypes::variant::Variant;
use crate::idltypes::vartype::VarType;

pub unsafe trait PropertyType<'a>
where
    Self: 'a,
{
    const VARTYPES: &'static [VarType];
    fn from_variant(var: &Variant) -> Self;
}

unsafe impl PropertyType<'static> for i8 {
    const VARTYPES: &'static [VarType] = &[
        VarType::I1,
        VarType(VarType::BYREF.0 | VarType::I1.0),
    ];
    fn from_variant(var: &Variant) -> Self {
        unsafe {
            let vt = var.vartype();
            match (vt.container_type(), vt.primitive_type()) {
                (VarType::EMPTY, VarType::I1) => var.v.data.c_val,
                (VarType::BYREF, VarType::I1) => *var.v.data.pc_val,
                _ => panic!(),
            }
        }
    }
}

unsafe impl PropertyType<'static> for u8 {
    const VARTYPES: &'static [VarType] = &[
        VarType::UI1,
        VarType(VarType::BYREF.0 | VarType::UI1.0),
    ];
    fn from_variant(var: &Variant) -> Self {
        unsafe {
            let vt = var.vartype();
            match (vt.container_type(), vt.primitive_type()) {
                (VarType::EMPTY, VarType::UI1) => var.v.data.b_val,
                (VarType::BYREF, VarType::UI1) => *var.v.data.pb_val,
                _ => panic!(),
            }
        }
    }
}

unsafe impl PropertyType<'static> for i16 {
    const VARTYPES: &'static [VarType] = &[
        VarType::I2,
        VarType(VarType::BYREF.0 | VarType::I2.0),
    ];
    fn from_variant(var: &Variant) -> Self {
        unsafe {
            let vt = var.vartype();
            match (vt.container_type(), vt.primitive_type()) {
                (VarType::EMPTY, VarType::I2) => var.v.data.i_val,
                (VarType::BYREF, VarType::I2) => *var.v.data.pi_val,
                _ => panic!(),
            }
        }
    }
}

unsafe impl PropertyType<'static> for u16 {
    const VARTYPES: &'static [VarType] = &[
        VarType::UI2,
        VarType(VarType::BYREF.0 | VarType::UI2.0),
    ];
    fn from_variant(var: &Variant) -> Self {
        unsafe {
            let vt = var.vartype();
            match (vt.container_type(), vt.primitive_type()) {
                (VarType::EMPTY, VarType::UI2) => var.v.data.ui_val,
                (VarType::BYREF, VarType::UI2) => *var.v.data.pui_val,
                _ => panic!(),
            }
        }
    }
}

unsafe impl PropertyType<'static> for i32 {
    const VARTYPES: &'static [VarType] = &[
        VarType::I4,
        VarType::INT,
        VarType(VarType::BYREF.0 | VarType::I4.0),
        VarType(VarType::BYREF.0 | VarType::INT.0),
    ];
    fn from_variant(var: &Variant) -> Self {
        unsafe {
            let vt = var.vartype();
            match (vt.container_type(), vt.primitive_type()) {
                (VarType::EMPTY, VarType::I4) => var.v.data.l_val,
                (VarType::EMPTY, VarType::INT) => var.v.data.int_val,
                (VarType::BYREF, VarType::I4) => *var.v.data.pl_val,
                (VarType::BYREF, VarType::INT) => *var.v.data.pint_val,
                _ => panic!(),
            }
        }
    }
}

unsafe impl PropertyType<'static> for u32 {
    const VARTYPES: &'static [VarType] = &[
        VarType::UI4,
        VarType::UINT,
        VarType(VarType::BYREF.0 | VarType::UI4.0),
        VarType(VarType::BYREF.0 | VarType::UINT.0),
    ];
    fn from_variant(var: &Variant) -> Self {
        unsafe {
            let vt = var.vartype();
            match (vt.container_type(), vt.primitive_type()) {
                (VarType::EMPTY, VarType::UI4) => var.v.data.ul_val,
                (VarType::EMPTY, VarType::UINT) => var.v.data.uint_val,
                (VarType::BYREF, VarType::UI4) => *var.v.data.pul_val,
                (VarType::BYREF, VarType::UINT) => *var.v.data.puint_val,
                _ => panic!(),
            }
        }
    }
}

unsafe impl PropertyType<'static> for i64 {
    const VARTYPES: &'static [VarType] = &[
        VarType::I8,
        VarType(VarType::BYREF.0 | VarType::I8.0),
    ];
    fn from_variant(var: &Variant) -> Self {
        unsafe {
            let vt = var.vartype();
            match (vt.container_type(), vt.primitive_type()) {
                (VarType::EMPTY, VarType::I8) => var.v.data.ll_val,
                (VarType::BYREF, VarType::I8) => *var.v.data.pll_val,
                _ => panic!(),
            }
        }
    }
}

unsafe impl PropertyType<'static> for u64 {
    const VARTYPES: &'static [VarType] = &[
        VarType::UI8,
        VarType(VarType::BYREF.0 | VarType::UI8.0),
    ];
    fn from_variant(var: &Variant) -> Self {
        unsafe {
            let vt = var.vartype();
            match (vt.container_type(), vt.primitive_type()) {
                (VarType::EMPTY, VarType::UI8) => var.v.data.ull_val,
                (VarType::BYREF, VarType::UI8) => *var.v.data.pull_val,
                _ => panic!(),
            }
        }
    }
}
