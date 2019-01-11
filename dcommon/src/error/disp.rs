use crate::Error;

use winapi::shared::winerror::*;

impl Error {
    /// Unknown interface.
    pub const DISP_UNKNOWNINTERFACE: Error = Error(DISP_E_UNKNOWNINTERFACE);

    /// Member not found.
    pub const DISP_MEMBERNOTFOUND: Error = Error(DISP_E_MEMBERNOTFOUND);

    /// Parameter not found.
    pub const DISP_PARAMNOTFOUND: Error = Error(DISP_E_PARAMNOTFOUND);

    /// Type mismatch.
    pub const DISP_TYPEMISMATCH: Error = Error(DISP_E_TYPEMISMATCH);

    /// Unknown name.
    pub const DISP_UNKNOWNNAME: Error = Error(DISP_E_UNKNOWNNAME);

    /// No named arguments.
    pub const DISP_NONAMEDARGS: Error = Error(DISP_E_NONAMEDARGS);

    /// Bad variable type.
    pub const DISP_BADVARTYPE: Error = Error(DISP_E_BADVARTYPE);

    /// Exception occurred.
    pub const DISP_EXCEPTION: Error = Error(DISP_E_EXCEPTION);

    /// Out of present range.
    pub const DISP_OVERFLOW: Error = Error(DISP_E_OVERFLOW);

    /// Invalid index.
    pub const DISP_BADINDEX: Error = Error(DISP_E_BADINDEX);

    /// Unknown language.
    pub const DISP_UNKNOWNLCID: Error = Error(DISP_E_UNKNOWNLCID);

    /// Memory is locked.
    pub const DISP_ARRAYISLOCKED: Error = Error(DISP_E_ARRAYISLOCKED);

    /// Invalid number of parameters.
    pub const DISP_BADPARAMCOUNT: Error = Error(DISP_E_BADPARAMCOUNT);

    /// Parameter not optional.
    pub const DISP_PARAMNOTOPTIONAL: Error = Error(DISP_E_PARAMNOTOPTIONAL);

    /// Invalid callee.
    pub const DISP_BADCALLEE: Error = Error(DISP_E_BADCALLEE);

    /// Does not support a collection.
    pub const DISP_NOTACOLLECTION: Error = Error(DISP_E_NOTACOLLECTION);

    /// Division by zero.
    pub const DISP_DIVBYZERO: Error = Error(DISP_E_DIVBYZERO);

    /// Buffer too small
    pub const DISP_BUFFERTOOSMALL: Error = Error(DISP_E_BUFFERTOOSMALL);
}
