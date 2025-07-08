use crate::types::EventType;

/// An event is an action that occurs during the program execution, usually asynchronously or unpredictably and
/// on which a given object must be notified.
pub struct Event {
    pub(crate) inner: *mut nappgui_sys::Event,
}

impl Event {
    pub(crate) fn new(ptr: *mut nappgui_sys::Event) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Get the event type.
    pub fn type_(&self) -> EventType {
        let event_type = unsafe { nappgui_sys::event_type(self.inner as _) } as i32;
        EventType::try_from(event_type).unwrap()
    }

    /// Get the event parameters, encapsulated in a structure, which will be different depending on the event type.
    pub fn params<T>(&self) -> Option<T>
    where
        T: NappGUIEventParams,
    {
        let tp = T::type_();
        let tp = std::ffi::CString::new(tp).unwrap();
        let params = unsafe { nappgui_sys::event_params_imp(self.inner, tp.as_ptr()) };

        T::from_ptr(params)
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
        let tp = T::type_();
        let tp = std::ffi::CString::new(tp).unwrap();
        let result = unsafe { nappgui_sys::event_result_imp(self.inner, tp.as_ptr()) }
            as *mut <T as NappGUIEventResult>::CrossType;
        let value = value.to_cross_type();
        *result = value;
    }
}

/// The event parameters.
pub trait NappGUIEventParams {
    /// The event type.
    fn type_() -> &'static str;

    /// Get the event parameters from the pointer.
    fn from_ptr(ptr: *mut std::ffi::c_void) -> Option<Self>
    where
        Self: Sized;
}

/// The event parameters.
pub trait NappGUIEventResult {
    type CrossType;

    /// The event type.
    fn type_() -> &'static str {
        ""
    }

    /// Convert to the cross type.
    fn to_cross_type(&self) -> Self::CrossType;
}
