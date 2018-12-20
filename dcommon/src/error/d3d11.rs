use crate::error::Error;

use winapi::shared::winerror::*;

/// D3D11 Error Constants
impl Error {
    /// The application has exceeded the maximum number of unique state objects per Direct3D device.
    /// The limit is 4096 for feature levels up to 11.1.
    pub const D3D11_TOO_MANY_UNIQUE_STATE_OBJECTS: Error =
        Error(D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS);
    /// The specified file was not found.
    pub const D3D11_FILE_NOT_FOUND: Error = Error(D3D11_ERROR_FILE_NOT_FOUND);
    /// The application has exceeded the maximum number of unique view objects per Direct3D device.
    /// The limit is 2^20 for feature levels up to 11.1.
    pub const D3D11_TOO_MANY_UNIQUE_VIEW_OBJECTS: Error =
        Error(D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS);
    /// The application's first call per command list to Map on a deferred context did not use D3D11_MAP_WRITE_DISCARD.
    pub const D3D11_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD: Error =
        Error(D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD);
}
