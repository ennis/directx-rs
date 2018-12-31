use crate::error::Error;

use winapi::shared::winerror::*;

impl Error {
    /// Catastrophic failure
    pub const UNEXPECTED: Error = Error(E_UNEXPECTED);
    /// Not implemented
    pub const NOTIMPL: Error = Error(E_NOTIMPL);
    /// Ran out of memory
    pub const OUTOFMEMORY: Error = Error(E_OUTOFMEMORY);
    /// One or more arguments are invalid
    pub const INVALIDARG: Error = Error(E_INVALIDARG);
    /// No such interface supported
    pub const NOINTERFACE: Error = Error(E_NOINTERFACE);
    /// Invalid pointer
    pub const POINTER: Error = Error(E_POINTER);
    /// Invalid handle
    pub const HANDLE: Error = Error(E_HANDLE);
    /// Operation aborted
    pub const ABORT: Error = Error(E_ABORT);
    /// Unspecified error
    pub const FAIL: Error = Error(E_FAIL);
    /// General access denied error
    pub const ACCESSDENIED: Error = Error(E_ACCESSDENIED);
    /// The data necessary to complete this operation is not yet available.
    pub const PENDING: Error = Error(E_PENDING);
    /// The operation attempted to access data outside the valid range
    pub const BOUNDS: Error = Error(E_BOUNDS);
    /// A concurrent or interleaved operation changed the state of the object, invalidating this operation.
    pub const CHANGED_STATE: Error = Error(E_CHANGED_STATE);
    /// An illegal state change was requested.
    pub const ILLEGAL_STATE_CHANGE: Error = Error(E_ILLEGAL_STATE_CHANGE);
    /// A method was called at an unexpected time.
    pub const ILLEGAL_METHOD_CALL: Error = Error(E_ILLEGAL_METHOD_CALL);
    /// String not null terminated.
    pub const STRING_NOT_NULL_TERMINATED: Error = Error(E_STRING_NOT_NULL_TERMINATED);
    /// A delegate was assigned when not allowed.
    pub const ILLEGAL_DELEGATE_ASSIGNMENT: Error = Error(E_ILLEGAL_DELEGATE_ASSIGNMENT);
    /// An async operation was not properly started.
    pub const ASYNC_OPERATION_NOT_STARTED: Error = Error(E_ASYNC_OPERATION_NOT_STARTED);
    /// The application is exiting and cannot service this request
    pub const APPLICATION_EXITING: Error = Error(E_APPLICATION_EXITING);
    /// The application view is exiting and cannot service this request
    pub const APPLICATION_VIEW_EXITING: Error = Error(E_APPLICATION_VIEW_EXITING);
}
