use crate::error::{fixme, Error};

use winapi::shared::winerror::*;

/// DWrite Error Constants
impl Error {
    /// Indicates an error in an input file such as a font file.
    pub const DWRITE_FILEFORMAT: Error = Error(DWRITE_E_FILEFORMAT);
    /// Indicates an error originating in DirectWrite code, which is not expected to occur but is safe to recover from.
    pub const DWRITE_UNEXPECTED: Error = Error(DWRITE_E_UNEXPECTED);
    /// Indicates the specified font does not exist.
    pub const DWRITE_NOFONT: Error = Error(DWRITE_E_NOFONT);
    /// A font file could not be opened because the file, directory, network location, drive, or other storage location does not exist or is unavailable.
    pub const DWRITE_FILENOTFOUND: Error = Error(DWRITE_E_FILENOTFOUND);
    /// A font file exists but could not be opened due to access denied, sharing violation, or similar error.
    pub const DWRITE_FILEACCESS: Error = Error(DWRITE_E_FILEACCESS);
    /// A font collection is obsolete due to changes in the system.
    pub const DWRITE_FONTCOLLECTIONOBSOLETE: Error = Error(DWRITE_E_FONTCOLLECTIONOBSOLETE);
    /// The given interface is already registered.
    pub const DWRITE_ALREADYREGISTERED: Error = Error(DWRITE_E_ALREADYREGISTERED);
    /// The font cache contains invalid data.
    pub const DWRITE_CACHEFORMAT: Error = Error(DWRITE_E_CACHEFORMAT);
    /// A font cache file corresponds to a different version of DirectWrite.
    pub const DWRITE_CACHEVERSION: Error = Error(DWRITE_E_CACHEVERSION);
    /// The operation is not supported for this type of font.
    pub const DWRITE_UNSUPPORTEDOPERATION: Error = Error(DWRITE_E_UNSUPPORTEDOPERATION);
    /// The version of the text renderer interface is not compatible.
    pub const DWRITE_TEXTRENDERERINCOMPATIBLE: Error = Error(DWRITE_E_TEXTRENDERERINCOMPATIBLE);
    /// The flow direction conflicts with the reading direction. They must be perpendicular to each other.
    pub const DWRITE_FLOWDIRECTIONCONFLICTS: Error = Error(DWRITE_E_FLOWDIRECTIONCONFLICTS);
    /// The font or glyph run does not contain any colored glyphs.
    pub const DWRITE_NOCOLOR: Error = Error(DWRITE_E_NOCOLOR);
    /// A font resource could not be accessed because it is remote.
    pub const DWRITE_REMOTEFONT: Error = Error(fixme::DWRITE_E_REMOTEFONT);
    /// A font download was canceled.
    pub const DWRITE_DOWNLOADCANCELLED: Error = Error(fixme::DWRITE_E_DOWNLOADCANCELLED);
    /// A font download failed.
    pub const DWRITE_DOWNLOADFAILED: Error = Error(fixme::DWRITE_E_DOWNLOADFAILED);
    /// A font download request was not added or a download failed because there are too many active downloads.
    pub const DWRITE_TOOMANYDOWNLOADS: Error = Error(fixme::DWRITE_E_TOOMANYDOWNLOADS);
}
