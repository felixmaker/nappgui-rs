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
    pub fn type_(&self) -> u32 {
        unsafe { nappgui_sys::event_type(self.inner as _) }
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

    /// Gets an object to write the results of the event. Some events require the return of data by the receiver.
    /// The type of result object will depend on the type of event.
    pub fn result<T>(&self) -> Option<T>
    where
        T: NappGUIEventResult,
    {
        let tp = T::type_();
        let tp = std::ffi::CString::new(tp).unwrap();
        let result = unsafe { nappgui_sys::event_result_imp(self.inner, tp.as_ptr()) };

        T::from_ptr(result)
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

/// The event result.
pub trait NappGUIEventResult {
    /// The event type.
    fn type_() -> &'static str;
    /// Get the event result from the pointer.
    fn from_ptr(ptr: *mut std::ffi::c_void) -> Option<Self>
    where
        Self: Sized; 
}
