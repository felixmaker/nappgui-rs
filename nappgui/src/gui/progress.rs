use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{progress_create, progress_undefined, progress_value, progress_width};

use crate::gui::{global_get, global_record};

pub(crate) struct ProgressInner {
    ptr: NonNull<nappgui_sys::Progress>,
}

impl ProgressInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::Progress) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to ProgressInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Progress {
        self.ptr.as_ptr()
    }
}

/// The progress bar control.
///
/// # Remark
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct Progress(Weak<ProgressInner>);

impl Progress {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Progress) -> Self {
        let object = global_record(ptr as _, ProgressInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::Progress) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Progress {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

    /// Set the natural width of control. By default 100px.
    pub fn set_width(&self, width: f32) {
        unsafe { progress_width(self.as_ptr(), width) };
    }

    /// Create a progress bar.
    pub fn new() -> Self {
        let progress = unsafe { progress_create() };
        unsafe { Self::from_raw(progress) }
    }

    /// Set the progress bar as undefined.
    pub fn set_undefined(&self, running: bool) {
        unsafe { progress_undefined(self.as_ptr(), running as _) };
    }

    /// Set the progress position.
    pub fn set_value(&self, value: f32) {
        unsafe { progress_value(self.as_ptr(), value) };
    }
}
