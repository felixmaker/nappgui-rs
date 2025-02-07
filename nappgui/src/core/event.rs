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
    pub fn params<T>(&self) -> T
    where
        T: NappGUIEvent,
    {
        let tp = T::type_();
        let tp = std::ffi::CString::new(tp).unwrap();
        let params = unsafe { nappgui_sys::event_params_imp(self.inner, tp.as_ptr()) };

        T::from_ptr(params)
    }
}

pub trait NappGUIEvent {
    fn type_() -> &'static str;
    fn from_ptr(ptr: *mut std::ffi::c_void) -> Self;
}
