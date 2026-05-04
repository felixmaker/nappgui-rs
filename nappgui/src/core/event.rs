use std::ffi::CStr;

use crate::types::EventType;

/// An event is an action that occurs during the program execution, usually asynchronously or unpredictably and
/// on which a given object must be notified.
pub struct Event {
    pub(crate) inner: *mut nappgui_sys::Event,
}

impl Event {
    pub(crate) fn new(ptr: *mut nappgui_sys::Event) -> Self {
        assert!(!ptr.is_null());
        Self { inner: ptr }
    }

    /// Get the event type.
    pub fn type_(&self) -> EventType {
        let event_type = unsafe { nappgui_sys::event_type(self.inner as _) } as i32;
        EventType::try_from(event_type).unwrap()
    }

    /// Get the event parameters, encapsulated in a structure, which will be different depending on the event type.
    pub unsafe fn params<T>(&self) -> T
    where
        T: NappGUIEventParams,
    {
        let params = unsafe { nappgui_sys::event_params_imp(self.inner, T::TYPE.as_ptr()) as *mut T::CType };
        let params = unsafe { &*params };
        T::from(params)
    }

    /// Set the result of the event. Some events require the return of data by the receiver.
    /// The type of result object will depend on the type of event.
    ///
    /// # Safety
    ///
    /// Make sure the result is correct or undefined behavior may occur.
    pub unsafe fn result<T>(&self, value: T)
    where
        T: NappGUIEventResult,
    {
        let result = unsafe { nappgui_sys::event_result_imp(self.inner, T::TYPE.as_ptr()) } as *mut T::CType;
        let value = value.to();
        *result = value;
    }
}

/// The event parameters.
pub trait NappGUIEventParams {
    /// The event parameters in c.
    type CType;
    /// The type of the event parameters.
    const TYPE: &'static CStr;

    /// Get the event parameters from the pointer.
    fn from(event: &Self::CType) -> Self
    where
        Self: Sized;
}

/// The event parameters.
pub trait NappGUIEventResult {
    /// The event result in c.
    type CType;
    /// The type of the event result.
    const TYPE: &'static CStr;

    /// Convert to the cross type.
    fn to(&self) -> Self::CType;
}
