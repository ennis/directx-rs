use crate::error::{fixme, Error};

use winapi::shared::winerror::*;

/// Direct2D Error Constants
impl Error {
    /// The object was not in the correct state to process the method.
    pub const D2D_WRONG_STATE: Error = Error(D2DERR_WRONG_STATE);
    /// The object has not yet been initialized.
    pub const D2D_NOT_INITIALIZED: Error = Error(D2DERR_NOT_INITIALIZED);
    /// The requested operation is not supported.
    pub const D2D_UNSUPPORTED_OPERATION: Error = Error(D2DERR_UNSUPPORTED_OPERATION);
    /// The geometry scanner failed to process the data.
    pub const D2D_SCANNER_FAILED: Error = Error(D2DERR_SCANNER_FAILED);
    /// Direct2D could not access the screen.
    pub const D2D_SCREEN_ACCESS_DENIED: Error = Error(D2DERR_SCREEN_ACCESS_DENIED);
    /// A valid display state could not be determined.
    pub const D2D_DISPLAY_STATE_INVALID: Error = Error(D2DERR_DISPLAY_STATE_INVALID);
    /// The supplied vector is zero.
    pub const D2D_ZERO_VECTOR: Error = Error(D2DERR_ZERO_VECTOR);
    /// An internal error (Direct2D bug) occurred. On checked builds, we would assert. The application should close this instance of Direct2D and should consider restarting its process.
    pub const D2D_INTERNAL_ERROR: Error = Error(D2DERR_INTERNAL_ERROR);
    /// The display format Direct2D needs to render is not supported by the hardware device.
    pub const D2D_DISPLAY_FORMAT_NOT_SUPPORTED: Error = Error(D2DERR_DISPLAY_FORMAT_NOT_SUPPORTED);
    /// A call to this method is invalid.
    pub const D2D_INVALID_CALL: Error = Error(D2DERR_INVALID_CALL);
    /// No hardware rendering device is available for this operation.
    pub const D2D_NO_HARDWARE_DEVICE: Error = Error(D2DERR_NO_HARDWARE_DEVICE);
    /// There has been a presentation error that may be recoverable. The caller needs to recreate, rerender the entire frame, and reattempt present.
    pub const D2D_RECREATE_TARGET: Error = Error(D2DERR_RECREATE_TARGET);
    /// Shader construction failed because it was too complex.
    pub const D2D_TOO_MANY_SHADER_ELEMENTS: Error = Error(D2DERR_TOO_MANY_SHADER_ELEMENTS);
    /// Shader compilation failed.
    pub const D2D_SHADER_COMPILE_FAILED: Error = Error(D2DERR_SHADER_COMPILE_FAILED);
    /// Requested DirectX surface size exceeded maximum texture size.
    pub const D2D_MAX_TEXTURE_SIZE_EXCEEDED: Error = Error(D2DERR_MAX_TEXTURE_SIZE_EXCEEDED);
    /// The requested Direct2D version is not supported.
    pub const D2D_UNSUPPORTED_VERSION: Error = Error(D2DERR_UNSUPPORTED_VERSION);
    /// Invalid number.
    pub const D2D_BAD_NUMBER: Error = Error(D2DERR_BAD_NUMBER);
    /// Objects used together must be created from the same factory instance.
    pub const D2D_WRONG_FACTORY: Error = Error(D2DERR_WRONG_FACTORY);
    /// A layer resource can only be in use once at any point in time.
    pub const D2D_LAYER_ALREADY_IN_USE: Error = Error(D2DERR_LAYER_ALREADY_IN_USE);
    /// The pop call did not match the corresponding push call.
    pub const D2D_POP_CALL_DID_NOT_MATCH_PUSH: Error = Error(D2DERR_POP_CALL_DID_NOT_MATCH_PUSH);
    /// The resource was realized on the wrong render target.
    pub const D2D_WRONG_RESOURCE_DOMAIN: Error = Error(D2DERR_WRONG_RESOURCE_DOMAIN);
    /// The push and pop calls were unbalanced.
    pub const D2D_PUSH_POP_UNBALANCED: Error = Error(D2DERR_PUSH_POP_UNBALANCED);
    /// Attempt to copy from a render target while a layer or clip rect is applied.
    pub const D2D_RENDER_TARGET_HAS_LAYER_OR_CLIPRECT: Error =
        Error(D2DERR_RENDER_TARGET_HAS_LAYER_OR_CLIPRECT);
    /// The brush types are incompatible for the call.
    pub const D2D_INCOMPATIBLE_BRUSH_TYPES: Error = Error(D2DERR_INCOMPATIBLE_BRUSH_TYPES);
    /// An unknown win32 failure occurred.
    pub const D2D_WIN32_ERROR: Error = Error(D2DERR_WIN32_ERROR);
    /// The render target is not compatible with GDI.
    pub const D2D_TARGET_NOT_GDI_COMPATIBLE: Error = Error(D2DERR_TARGET_NOT_GDI_COMPATIBLE);
    /// A text client drawing effect object is of the wrong type.
    pub const D2D_TEXT_EFFECT_IS_WRONG_TYPE: Error = Error(D2DERR_TEXT_EFFECT_IS_WRONG_TYPE);
    /// The application is holding a reference to the IDWriteTextRenderer interface after the corresponding DrawText or DrawTextLayout call has returned. The IDWriteTextRenderer instance will be invalid.
    pub const D2D_TEXT_RENDERER_NOT_RELEASED: Error = Error(D2DERR_TEXT_RENDERER_NOT_RELEASED);
    /// The requested size is larger than the guaranteed supported texture size at the Direct3D device's current feature level.
    pub const D2D_EXCEEDS_MAX_BITMAP_SIZE: Error = Error(D2DERR_EXCEEDS_MAX_BITMAP_SIZE);
    /// There was a configuration error in the graph.
    pub const D2D_INVALID_GRAPH_CONFIGURATION: Error = Error(D2DERR_INVALID_GRAPH_CONFIGURATION);
    /// There was a internal configuration error in the graph.
    pub const D2D_INVALID_INTERNAL_GRAPH_CONFIGURATION: Error =
        Error(D2DERR_INVALID_INTERNAL_GRAPH_CONFIGURATION);
    /// There was a cycle in the graph.
    pub const D2D_CYCLIC_GRAPH: Error = Error(D2DERR_CYCLIC_GRAPH);
    /// Cannot draw with a bitmap that has the D2D1_BITMAP_OPTIONS_CANNOT_DRAW option.
    pub const D2D_BITMAP_CANNOT_DRAW: Error = Error(D2DERR_BITMAP_CANNOT_DRAW);
    /// The operation cannot complete while there are outstanding references to the target bitmap.
    pub const D2D_OUTSTANDING_BITMAP_REFERENCES: Error =
        Error(D2DERR_OUTSTANDING_BITMAP_REFERENCES);
    /// The operation failed because the original target is not currently bound as a target.
    pub const D2D_ORIGINAL_TARGET_NOT_BOUND: Error = Error(D2DERR_ORIGINAL_TARGET_NOT_BOUND);
    /// Cannot set the image as a target because it is either an effect or is a bitmap that does not have the D2D1_BITMAP_OPTIONS_TARGET flag set.
    pub const D2D_INVALID_TARGET: Error = Error(D2DERR_INVALID_TARGET);
    /// Cannot draw with a bitmap that is currently bound as the target bitmap.
    pub const D2D_BITMAP_BOUND_AS_TARGET: Error = Error(D2DERR_BITMAP_BOUND_AS_TARGET);
    /// D3D Device does not have sufficient capabilities to perform the requested action.
    pub const D2D_INSUFFICIENT_DEVICE_CAPABILITIES: Error =
        Error(D2DERR_INSUFFICIENT_DEVICE_CAPABILITIES);
    /// The graph could not be rendered with the context's current tiling settings.
    pub const D2D_INTERMEDIATE_TOO_LARGE: Error = Error(D2DERR_INTERMEDIATE_TOO_LARGE);
    /// The CLSID provided to Unregister did not correspond to a registered effect.
    pub const D2D_EFFECT_IS_NOT_REGISTERED: Error = Error(D2DERR_EFFECT_IS_NOT_REGISTERED);
    /// The specified property does not exist.
    pub const D2D_INVALID_PROPERTY: Error = Error(D2DERR_INVALID_PROPERTY);
    /// The specified sub-property does not exist.
    pub const D2D_NO_SUBPROPERTIES: Error = Error(D2DERR_NO_SUBPROPERTIES);
    /// AddPage or Close called after print job is already closed.
    pub const D2D_PRINT_JOB_CLOSED: Error = Error(D2DERR_PRINT_JOB_CLOSED);
    /// Error during print control creation. Indicates that none of the package target types (representing printer formats) are supported by Direct2D print control.
    pub const D2D_PRINT_FORMAT_NOT_SUPPORTED: Error = Error(D2DERR_PRINT_FORMAT_NOT_SUPPORTED);
    /// An effect attempted to use a transform with too many inputs.
    pub const D2D_TOO_MANY_TRANSFORM_INPUTS: Error = Error(D2DERR_TOO_MANY_TRANSFORM_INPUTS);
    /// An error was encountered while decoding or parsing the requested glyph image.
    pub const D2D_INVALID_GLYPH_IMAGE: Error = Error(fixme::D2DERR_INVALID_GLYPH_IMAGE);
}
