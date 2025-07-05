use std::ffi::CString;

use nappgui_sys::{stm_from_block, stm_from_file, stm_memory, stm_to_file};

use crate::error::NappguiError;

/// A stream is a data flow that runs from a source to a destination. Think of a phone call. We have an
/// origin (the person who speaks), a destination (the person who listens) and a channel (the line itself).
/// In programming, the stream is the equivalent to the telephone line, it is the pipe that joins the
/// application with a data source or destination (Figure 1) and through which binary information, bit
/// sequences, run. As with any other communication channel, the information is volatile, available for a
/// very limited time. Once it reaches the receiver, it disappears.
pub struct Stream {
    pub(crate) inner: *mut nappgui_sys::Stream,
}

impl Stream {
    pub(crate) fn new(ptr: *mut nappgui_sys::Stream) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a read stream from an existing memory block.
    ///
    /// # Remark
    /// The original block will not be modified (read only). When the end of the block is reached stm_state will
    /// return ekSTEND. Block stream.
    pub fn from_block(data: *const u8, size: u32) -> Self {
        let ptr = unsafe { stm_from_block(data, size) };
        Self::new(ptr)
    }

    /// Create a read/write memory stream.
    ///
    /// # Remark
    /// It can be used as an internal pipeline for the information exchange between functions or threads. It behaves
    /// like a FIFO (First In Fist Out) buffer. For multi-threaded access you must be protected with a Mutex.
    /// Memory stream.
    pub fn memory(size: u32) -> Self {
        let ptr = unsafe { stm_memory(size) };
        Self::new(ptr)
    }

    /// Create a stream to read from a file on disk.
    pub fn from_file(pathname: &str) -> Result<Self, NappguiError> {
        let error = std::ptr::null_mut();
        let pathname = CString::new(pathname).unwrap();
        let ptr = unsafe { stm_from_file(pathname.as_ptr(), error) };
        let error = unsafe { *error };
        if !ptr.is_null() {
            Ok(Self::new(ptr))
        } else {
            Err(NappguiError::from_ferror_t(error))
        }
    }

    /// Create a stream to write data to a file on disk.
    ///
    /// # Remark
    /// If the file already exists it will be overwritten.
    pub fn to_file(pathname: &str) -> Result<Self, NappguiError> {
        let error = std::ptr::null_mut();
        let pathname = CString::new(pathname).unwrap();
        let ptr = unsafe { stm_to_file(pathname.as_ptr(), error) };
        let error = unsafe { *error };
        if !ptr.is_null() {
            Ok(Self::new(ptr))
        } else {
            Err(NappguiError::from_ferror_t(error))
        }
    }

    /// Create a stream to write data to the end of an existing file.
    ///
    /// # Remark
    /// It will fail if the file does not exist (do not create it). File stream.
    pub fn append_file(pathname: &str) -> Result<Self, NappguiError> {
        let error = std::ptr::null_mut();
        let pathname = CString::new(pathname).unwrap();
        let ptr = unsafe { stm_to_file(pathname.as_ptr(), error) };
        let error = unsafe { *error };
        if !ptr.is_null() {
            Ok(Self::new(ptr))
        } else {
            Err(NappguiError::from_ferror_t(error))
        }
    }
}
