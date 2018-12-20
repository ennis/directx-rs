use crate::error::{fixme, Error, Status};

use winapi::shared::winerror::*;

/// DXGI Error Constants
impl Error {
    /// The object was not found. If calling IDXGIFactory::EnumAdaptes, there is no adapter with the specified ordinal.
    pub const DXGI_NOT_FOUND: Error = Error(DXGI_ERROR_NOT_FOUND);
    /// The caller did not supply a sufficiently large buffer.
    pub const DXGI_MORE_DATA: Error = Error(DXGI_ERROR_MORE_DATA);
    /// The specified device interface or feature level is not supported on this system.
    pub const DXGI_UNSUPPORTED: Error = Error(DXGI_ERROR_UNSUPPORTED);
    /// The GPU device instance has been suspended. Use GetDeviceRemovedReason to determine the appropriate action.
    pub const DXGI_DEVICE_REMOVED: Error = Error(DXGI_ERROR_DEVICE_REMOVED);
    /// The GPU will not respond to more commands, most likely because of an invalid command passed by the calling application.
    pub const DXGI_DEVICE_HUNG: Error = Error(DXGI_ERROR_DEVICE_HUNG);
    /// The GPU was busy at the moment when the call was made, and the call was neither executed nor scheduled.
    pub const DXGI_WAS_STILL_DRAWING: Error = Error(DXGI_ERROR_WAS_STILL_DRAWING);
    /// Fullscreen mode could not be achieved because the specified output was already in use.
    pub const DXGI_GRAPHICS_VIDPN_SOURCE_IN_USE: Error =
        Error(DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE);
    /// A global counter resource was in use, and the specified counter cannot be used by this Direct3D device at this time.
    pub const DXGI_NONEXCLUSIVE: Error = Error(DXGI_ERROR_NONEXCLUSIVE);
    /// A resource is not available at the time of the call, but may become available later.
    pub const DXGI_NOT_CURRENTLY_AVAILABLE: Error = Error(DXGI_ERROR_NOT_CURRENTLY_AVAILABLE);
    /// The device has been removed during a remote session because the remote computer ran out of memory.
    pub const DXGI_REMOTE_OUTOFMEMORY: Error = Error(DXGI_ERROR_REMOTE_OUTOFMEMORY);
    /// The keyed mutex was abandoned.
    pub const DXGI_ACCESS_LOST: Error = Error(DXGI_ERROR_ACCESS_LOST);
    /// The timeout value has elapsed and the resource is not yet available.
    pub const DXGI_WAIT_TIMEOUT: Error = Error(DXGI_ERROR_WAIT_TIMEOUT);
    /// The DXGI outuput (monitor) to which the swapchain content was restricted, has been disconnected or changed.
    pub const DXGI_RESTRICT_TO_OUTPUT_STALE: Error = Error(DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE);
    /// The application is trying to create a shared handle using a name that is already associated with some other resource.
    pub const DXGI_NAME_ALREADY_EXISTS: Error = Error(DXGI_ERROR_NAME_ALREADY_EXISTS);
    /// The application requested an operation that depends on an SDK component that is missing or mismatched.
    pub const DXGI_SDK_COMPONENT_MISSING: Error = Error(DXGI_ERROR_SDK_COMPONENT_MISSING);
    /// The DXGI objects that the application has created are no longer current & need to be recreated for this operation to be performed.
    pub const DXGI_NOT_CURRENT: Error = Error(fixme::DXGI_ERROR_NOT_CURRENT);
    /// Insufficient HW protected memory exits for proper function.
    pub const DXGI_HW_PROTECTION_OUTOFMEMORY: Error =
        Error(fixme::DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY);
    /// Creating this device would violate the process's dynamic code policy.
    pub const DXGI_DYNAMIC_CODE_POLICY_VIOLATION: Error =
        Error(fixme::DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION);
    /// The operation failed because the compositor is not in control of the output.
    pub const DXGI_NON_COMPOSITED_UI: Error = Error(fixme::DXGI_ERROR_NON_COMPOSITED_UI);
    /// An on-going mode change prevented completion of the call. The call may succeed if attempted later.
    pub const DXGI_MODE_CHANGE_IN_PROGRESS: Error = Error(DXGI_ERROR_MODE_CHANGE_IN_PROGRESS);
    /// The cache is corrupt and either could not be opened or could not be reset.
    pub const DXGI_CACHE_CORRUPT: Error = Error(fixme::DXGI_ERROR_CACHE_CORRUPT);
    /// This entry would cause the cache to exceed its quota. On a load operation, this may indicate exceeding the maximum in-memory size.
    pub const DXGI_CACHE_FULL: Error = Error(fixme::DXGI_ERROR_CACHE_FULL);
    /// A cache entry was found, but the key provided does not match the key stored in the entry.
    pub const DXGI_CACHE_HASH_COLLISION: Error = Error(fixme::DXGI_ERROR_CACHE_HASH_COLLISION);
    /// The desired element already exists.
    pub const DXGI_ALREADY_EXISTS: Error = Error(fixme::DXGI_ERROR_ALREADY_EXISTS);
}

/// DXGI Status Constants
impl Status {
    /// The Present operation was invisible to the user.
    pub const DXGI_OCCLUDED: Status = Status(DXGI_STATUS_OCCLUDED);
    /// The Present operation was partially invisible to the user.
    pub const DXGI_CLIPPED: Status = Status(DXGI_STATUS_CLIPPED);
    /// The driver is requesting that the DXGI runtime not use shared resources to communicate with the Desktop Window Manager.
    pub const DXGI_NO_REDIRECTION: Status = Status(DXGI_STATUS_NO_REDIRECTION);
    /// The Present operation was not visible because the Windows session has switched to another desktop (for example, ctrl-alt-del).
    pub const DXGI_NO_DESKTOP_ACCESS: Status = Status(DXGI_STATUS_NO_DESKTOP_ACCESS);
    /// The Present operation was not visible because the target monitor was being used for some other purpose.
    pub const DXGI_GRAPHICS_VIDPN_SOURCE_IN_USE: Status =
        Status(DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE);
    /// The Present operation was not visible because the display mode changed. DXGI will have re-attempted the presentation.
    pub const DXGI_MODE_CHANGED: Status = Status(DXGI_STATUS_MODE_CHANGED);
    /// The Present operation was not visible because another Direct3D device was attempting to take fullscreen mode at the time.
    pub const DXGI_MODE_CHANGE_IN_PROGRESS: Status = Status(DXGI_STATUS_MODE_CHANGE_IN_PROGRESS);
    /// The swapchain has become unoccluded.
    pub const DXGI_UNOCCLUDED: Status = Status(DXGI_STATUS_UNOCCLUDED);
    /// The adapter did not have access to the required resources to complete the Desktop Duplication Present() call, the Present() call needs to be made again.
    pub const DXGI_DDA_WAS_STILL_DRAWING: Status = Status(DXGI_STATUS_DDA_WAS_STILL_DRAWING);
    /// The present succeeded but the caller should present again on the next V-sync, even if there are no changes to the content.
    pub const DXGI_PRESENT_REQUIRED: Status = Status(fixme::DXGI_STATUS_PRESENT_REQUIRED);
}
