use crate::error::{fixme, Error};

use winapi::shared::winerror::*;

/// WIC Error Constants
impl Error {
    /// The codec is in the wrong state.
    pub const WIC_WRONGSTATE: Error = Error(WINCODEC_ERR_WRONGSTATE);
    /// The value is out of range.
    pub const WIC_VALUEOUTOFRANGE: Error = Error(WINCODEC_ERR_VALUEOUTOFRANGE);
    /// The image format is unknown.
    pub const WIC_UNKNOWNIMAGEFORMAT: Error = Error(WINCODEC_ERR_UNKNOWNIMAGEFORMAT);
    /// The SDK version is unsupported.
    pub const WIC_UNSUPPORTEDVERSION: Error = Error(WINCODEC_ERR_UNSUPPORTEDVERSION);
    /// The component is not initialized.
    pub const WIC_NOTINITIALIZED: Error = Error(WINCODEC_ERR_NOTINITIALIZED);
    /// There is already an outstanding read or write lock.
    pub const WIC_ALREADYLOCKED: Error = Error(WINCODEC_ERR_ALREADYLOCKED);
    /// The specified bitmap property cannot be found.
    pub const WIC_PROPERTYNOTFOUND: Error = Error(WINCODEC_ERR_PROPERTYNOTFOUND);
    /// The bitmap codec does not support the bitmap property.
    pub const WIC_PROPERTYNOTSUPPORTED: Error = Error(WINCODEC_ERR_PROPERTYNOTSUPPORTED);
    /// The bitmap property size is invalid.
    pub const WIC_PROPERTYSIZE: Error = Error(WINCODEC_ERR_PROPERTYSIZE);
    /// An unknown error has occurred.
    pub const WIC_CODECPRESENT: Error = Error(WINCODEC_ERR_CODECPRESENT);
    /// The bitmap codec does not support a thumbnail.
    pub const WIC_CODECNOTHUMBNAIL: Error = Error(WINCODEC_ERR_CODECNOTHUMBNAIL);
    /// The bitmap palette is unavailable.
    pub const WIC_PALETTEUNAVAILABLE: Error = Error(WINCODEC_ERR_PALETTEUNAVAILABLE);
    /// Too many scanlines were requested.
    pub const WIC_CODECTOOMANYSCANLINES: Error = Error(WINCODEC_ERR_CODECTOOMANYSCANLINES);
    /// An internal error occurred.
    pub const WIC_INTERNALERROR: Error = Error(WINCODEC_ERR_INTERNALERROR);
    /// The bitmap bounds do not match the bitmap dimensions.
    pub const WIC_SOURCERECTDOESNOTMATCHDIMENSIONS: Error = Error(WINCODEC_ERR_SOURCERECTDOESNOTMATCHDIMENSIONS);
    /// The component cannot be found.
    pub const WIC_COMPONENTNOTFOUND: Error = Error(WINCODEC_ERR_COMPONENTNOTFOUND);
    /// The bitmap size is outside the valid range.
    pub const WIC_IMAGESIZEOUTOFRANGE: Error = Error(WINCODEC_ERR_IMAGESIZEOUTOFRANGE);
    /// There is too much metadata to be written to the bitmap.
    pub const WIC_TOOMUCHMETADATA: Error = Error(WINCODEC_ERR_TOOMUCHMETADATA);
    /// The image is unrecognized.
    pub const WIC_BADIMAGE: Error = Error(WINCODEC_ERR_BADIMAGE);
    /// The image header is unrecognized.
    pub const WIC_BADHEADER: Error = Error(WINCODEC_ERR_BADHEADER);
    /// The bitmap frame is missing.
    pub const WIC_FRAMEMISSING: Error = Error(WINCODEC_ERR_FRAMEMISSING);
    /// The image metadata header is unrecognized.
    pub const WIC_BADMETADATAHEADER: Error = Error(WINCODEC_ERR_BADMETADATAHEADER);
    /// The stream data is unrecognized.
    pub const WIC_BADSTREAMDATA: Error = Error(WINCODEC_ERR_BADSTREAMDATA);
    /// Failed to write to the stream.
    pub const WIC_STREAMWRITE: Error = Error(WINCODEC_ERR_STREAMWRITE);
    /// Failed to read from the stream.
    pub const WIC_STREAMREAD: Error = Error(WINCODEC_ERR_STREAMREAD);
    /// The stream is not available.
    pub const WIC_STREAMNOTAVAILABLE: Error = Error(WINCODEC_ERR_STREAMNOTAVAILABLE);
    /// The bitmap pixel format is unsupported.
    pub const WIC_UNSUPPORTEDPIXELFORMAT: Error = Error(WINCODEC_ERR_UNSUPPORTEDPIXELFORMAT);
    /// The operation is unsupported.
    pub const WIC_UNSUPPORTEDOPERATION: Error = Error(WINCODEC_ERR_UNSUPPORTEDOPERATION);
    /// The component registration is invalid.
    pub const WIC_INVALIDREGISTRATION: Error = Error(WINCODEC_ERR_INVALIDREGISTRATION);
    /// The component initialization has failed.
    pub const WIC_COMPONENTINITIALIZEFAILURE: Error = Error(WINCODEC_ERR_COMPONENTINITIALIZEFAILURE);
    /// The buffer allocated is insufficient.
    pub const WIC_INSUFFICIENTBUFFER: Error = Error(WINCODEC_ERR_INSUFFICIENTBUFFER);
    /// Duplicate metadata is present.
    pub const WIC_DUPLICATEMETADATAPRESENT: Error = Error(WINCODEC_ERR_DUPLICATEMETADATAPRESENT);
    /// The bitmap property type is unexpected.
    pub const WIC_PROPERTYUNEXPECTEDTYPE: Error = Error(WINCODEC_ERR_PROPERTYUNEXPECTEDTYPE);
    /// The size is unexpected.
    pub const WIC_UNEXPECTEDSIZE: Error = Error(WINCODEC_ERR_UNEXPECTEDSIZE);
    /// The property query is invalid.
    pub const WIC_INVALIDQUERYREQUEST: Error = Error(WINCODEC_ERR_INVALIDQUERYREQUEST);
    /// The metadata type is unexpected.
    pub const WIC_UNEXPECTEDMETADATATYPE: Error = Error(WINCODEC_ERR_UNEXPECTEDMETADATATYPE);
    /// The specified bitmap property is only valid at root level.
    pub const WIC_REQUESTONLYVALIDATMETADATAROOT: Error = Error(WINCODEC_ERR_REQUESTONLYVALIDATMETADATAROOT);
    /// The query string contains an invalid character.
    pub const WIC_INVALIDQUERYCHARACTER: Error = Error(WINCODEC_ERR_INVALIDQUERYCHARACTER);
    /// Windows Codecs received an error from the Win32 system.
    pub const WIC_WIN32ERROR: Error = Error(WINCODEC_ERR_WIN32ERROR);
    /// The requested level of detail is not present.
    pub const WIC_INVALIDPROGRESSIVELEVEL: Error = Error(WINCODEC_ERR_INVALIDPROGRESSIVELEVEL);
    /// The scan index is invalid.
    pub const WIC_INVALIDJPEGSCANINDEX: Error = Error(fixme::WINCODEC_ERR_INVALIDJPEGSCANINDEX);
}
