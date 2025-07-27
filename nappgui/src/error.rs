use std::{fmt, io, string::FromUtf8Error};

// Thanks to https://github.com/fltk-rs/fltk-rs/blob/d6e722c9707990f1315398acc30494b9b3d4ac69/fltk/src/prelude.rs
/// Error types returned by nappgui-rs + wrappers of std errors
#[derive(Debug)]
#[non_exhaustive]
pub enum NappguiError {
    /// i/o error
    IoError(io::Error),
    /// Utf-8 conversion error
    Utf8Error(FromUtf8Error),
    /// Null string conversion error
    NullError(std::ffi::NulError),
    /// Internal error
    Internal(NappguiErrorKind),
    /// Error using an erroneous env variable
    EnvVarError(std::env::VarError),
    /// Parsing error
    ParseIntError(std::num::ParseIntError),
    /// Unknown error
    Unknown(String),
}

/// Error kinds enum for `NappguiError`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum NappguiErrorKind {
    /// The file already exists.
    FExists,
    /// The directory does not exist.
    FNoPath,
    /// The file does not exists.
    FNoFile,
    /// The name of the file exceeds the capacity of the buffer to store it.
    FBigName,
    /// There are no more files when we travel through a directory. bfile_dir_get.
    FNoFiles,
    /// You are trying to delete a non-empty directory. hfile_dir_destroy.
    FNoEmpty,
    /// The file can not be accessed (possibly due to lack of permissions).
    FNoAccess,
    /// The file is being used by another process.
    FLock,
    /// The file is so big. It may appear in functions that can not handle files larger than 4Gb.
    FBig,
    /// Negative position within a file. See bfile_seek.
    FSeekNeg,
    /// There is no more information about the error.
    FUnDef,
    /// The member of a structure is already registered in DBind.
    DbindMemberExists,
    /// The data type is already registered in DBind.
    DbindTypeExists,
    /// The data type to be deleted is in use.
    DbindTypeUsed,
    /// The size of an alias type does not match that of the original type.
    DbindAliasSize,
    /// Undefined error
    UndefinedError,
    /// Undefined enum transmute
    UndefinedEnumTransmute,
}

impl NappguiError {
    pub(crate) fn from_ferror_t(err: i32) -> NappguiError {
        let err_kind = match err {
            nappgui_sys::_ferror_t_ekFEXISTS => NappguiErrorKind::FExists,
            nappgui_sys::_ferror_t_ekFNOPATH => NappguiErrorKind::FNoPath,
            nappgui_sys::_ferror_t_ekFNOFILE => NappguiErrorKind::FNoFile,
            nappgui_sys::_ferror_t_ekFBIGNAME => NappguiErrorKind::FBigName,
            nappgui_sys::_ferror_t_ekFNOFILES => NappguiErrorKind::FNoFiles,
            nappgui_sys::_ferror_t_ekFNOEMPTY => NappguiErrorKind::FNoEmpty,
            nappgui_sys::_ferror_t_ekFNOACCESS => NappguiErrorKind::FNoAccess,
            nappgui_sys::_ferror_t_ekFLOCK => NappguiErrorKind::FLock,
            nappgui_sys::_ferror_t_ekFUNDEF => NappguiErrorKind::FUnDef,
            nappgui_sys::_ferror_t_ekFBIG => NappguiErrorKind::FBig,
            nappgui_sys::_ferror_t_ekFSEEKNEG => NappguiErrorKind::FSeekNeg,
            _ => NappguiErrorKind::UndefinedError,
        };
        NappguiError::Internal(err_kind)
    }

    pub(crate) fn from_dbindst_t(err: i32) -> Self {
        let err_kind = match err {
            nappgui_sys::_dbindst_t_ekDBIND_MEMBER_EXISTS => NappguiErrorKind::DbindMemberExists,
            nappgui_sys::_dbindst_t_ekDBIND_TYPE_EXISTS => NappguiErrorKind::DbindTypeExists,
            nappgui_sys::_dbindst_t_ekDBIND_TYPE_USED => NappguiErrorKind::DbindTypeUsed,
            nappgui_sys::_dbindst_t_ekDBIND_ALIAS_SIZE => NappguiErrorKind::DbindAliasSize,
            _ => NappguiErrorKind::UndefinedError,
        };
        NappguiError::Internal(err_kind)
    }
}

impl std::error::Error for NappguiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NappguiError::IoError(err) => Some(err),
            NappguiError::NullError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for NappguiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NappguiError::IoError(ref err) => err.fmt(f),
            NappguiError::NullError(ref err) => err.fmt(f),
            NappguiError::Internal(ref err) => {
                write!(f, "An internal error occurred {:?}", err)
            }
            NappguiError::EnvVarError(ref err) => {
                write!(f, "An env var error occurred {:?}", err)
            }
            NappguiError::Utf8Error(ref err) => {
                write!(f, "A UTF8 conversion error occurred {:?}", err)
            }
            NappguiError::ParseIntError(ref err) => {
                write!(f, "An int parsing error occurred {:?}", err)
            }
            NappguiError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for NappguiError {
    fn from(err: io::Error) -> NappguiError {
        NappguiError::IoError(err)
    }
}

impl From<std::ffi::NulError> for NappguiError {
    fn from(err: std::ffi::NulError) -> NappguiError {
        NappguiError::NullError(err)
    }
}

impl From<std::env::VarError> for NappguiError {
    fn from(err: std::env::VarError) -> NappguiError {
        NappguiError::EnvVarError(err)
    }
}

impl From<std::string::FromUtf8Error> for NappguiError {
    fn from(err: std::string::FromUtf8Error) -> NappguiError {
        NappguiError::Utf8Error(err)
    }
}

impl From<std::num::ParseIntError> for NappguiError {
    fn from(err: std::num::ParseIntError) -> NappguiError {
        NappguiError::ParseIntError(err)
    }
}
