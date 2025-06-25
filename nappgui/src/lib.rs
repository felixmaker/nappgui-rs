#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub(crate) mod util;

/// Just as a building needs a strong foundation, any software project must be supported by robust
/// and efficient pillars. For this purpose, the core library has been developed, which provides
/// commonly used non-graphical utilities.
pub mod core;
/// The Draw2D library integrates all the functionality necessary to create two dimensions vector
/// graphics. It depends directly on Geom2D and, as we will see later, drawing does not
/// imply having a graphical user interface in the program.
pub mod draw_2d;
/// The Gui library allows you to create graphical user interfaces in a simple and intuitive way.
/// Only available for desktop applications for obvious reasons, unlike the rest of libraries
/// that can also be used in command line applications.
pub mod gui;
/// The OSApp library starts and manages the message cycle of a desktop application.
pub mod osapp;

/// Enums and types
pub mod prelude {
    use std::{fmt, io, string::FromUtf8Error};

    use nappgui_sys::{dbindst_t, ferror_t};

    /// State values.
    pub type GuiState = nappgui_sys::gui_state_t;
    /// Orientation.
    pub type GuiOrient = nappgui_sys::gui_orient_t;
    /// Cursors.
    pub type GuiCursor = nappgui_sys::gui_cursor_t;
    /// Result when changing the keyboard focus.
    pub type GuiFocus = nappgui_sys::gui_focus_t;
    /// Window creation attributes.
    pub type WindowFlag = nappgui_sys::window_flag_t;
    /// Alignment values.
    pub type Align = nappgui_sys::align_t;
    /// Style in typographic fonts. Multiple values can be combined with the OR operator ('|').
    pub type FStyle = nappgui_sys::fstyle_t;
    /// Pixel format in an image. Number of bits per pixel and color model.
    pub type PixFormat = nappgui_sys::pixformat_t;
    /// Keyboard codes.
    pub type Vkey = nappgui_sys::vkey_t;
    /// 2D affine transformation.
    pub type T2Df = nappgui_sys::T2Df;
    /// Represents a 2D size.
    pub type S2Df = nappgui_sys::S2Df;
    /// Represents a 2d vector or point.
    pub type V2Df = nappgui_sys::V2Df;
    /// Behavior of the divider in a SplitView.
    pub type SplitMode = nappgui_sys::split_mode_t;
    /// Return values in dbind.
    pub type DBindState = nappgui_sys::dbindst_t;

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
        /// Internal fltk error
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
    }

    impl From<ferror_t> for NappguiError {
        fn from(err: ferror_t) -> NappguiError {
            let err_kind = match err {
                ferror_t::ekFEXISTS => NappguiErrorKind::FExists,
                ferror_t::ekFNOPATH => NappguiErrorKind::FNoPath,
                ferror_t::ekFNOFILE => NappguiErrorKind::FNoFile,
                ferror_t::ekFBIGNAME => NappguiErrorKind::FBigName,
                ferror_t::ekFNOFILES => NappguiErrorKind::FNoFiles,
                ferror_t::ekFNOEMPTY => NappguiErrorKind::FNoEmpty,
                ferror_t::ekFNOACCESS => NappguiErrorKind::FNoAccess,
                ferror_t::ekFLOCK => NappguiErrorKind::FLock,
                ferror_t::ekFUNDEF => NappguiErrorKind::FUnDef,
                ferror_t::ekFBIG => NappguiErrorKind::FBig,
                ferror_t::ekFSEEKNEG => NappguiErrorKind::FSeekNeg,
                _ => NappguiErrorKind::UndefinedError,
            };
            NappguiError::Internal(err_kind)
        }
    }

    impl From<dbindst_t> for NappguiError {
        fn from(err: dbindst_t) -> Self {
            let err_kind = match err {
                dbindst_t::ekDBIND_MEMBER_EXISTS => NappguiErrorKind::DbindMemberExists,
                dbindst_t::ekDBIND_TYPE_EXISTS => NappguiErrorKind::DbindTypeExists,
                dbindst_t::ekDBIND_TYPE_USED => NappguiErrorKind::DbindTypeUsed,
                dbindst_t::ekDBIND_ALIAS_SIZE => NappguiErrorKind::DbindAliasSize,
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
}
