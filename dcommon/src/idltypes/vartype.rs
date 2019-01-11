#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VarType(pub u16);

impl VarType {
    pub fn primitive_type(self) -> VarType {
        self & VarType::TYPEMASK
    }

    pub fn container_type(self) -> VarType {
        self & !VarType::TYPEMASK
    }
}

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
