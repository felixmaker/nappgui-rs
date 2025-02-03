use crate::{core::event::Event, draw_2d::image::Image};

use nappgui_sys::{
    align_t, combo_OnChange, combo_OnFilter, combo_add_elem, combo_align, combo_bgcolor,
    combo_bgcolor_focus, combo_color, combo_color_focus, combo_count, combo_create, combo_del_elem,
    combo_duplicates, combo_get_text, combo_ins_elem, combo_phcolor, combo_phstyle, combo_phtext,
    combo_set_elem, combo_text, combo_tooltip, listener_imp,
};

pub struct Combo {
    pub(crate) inner: *mut nappgui_sys::Combo,
}

impl Combo {
    pub(crate) fn new(ptr: *mut nappgui_sys::Combo) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a combo control.
    pub fn create() -> Self {
        let combo = unsafe { combo_create() };
        Self::new(combo)
    }

    /// Set a function to filter the text while editing.
    pub fn on_filter<F>(&self, handler: F)
    where
        F: FnMut(&mut Combo, &Event) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (Box<dyn FnMut(&mut Combo, &Event)>, *mut nappgui_sys::Combo);
            let f = &mut *(*data).0;
            let mut obj = Combo { inner: (*data).1 };
            let ev = Event::new(event as _);
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut obj, &ev)));
        }

        let cb: Box<dyn FnMut(&mut Combo, &Event)> = Box::new(handler);

        let data: *mut (Box<dyn FnMut(&mut Combo, &Event)>, *mut nappgui_sys::Combo) =
            Box::into_raw(Box::new((cb, self.inner)));

        unsafe {
            combo_OnFilter(
                self.inner,
                listener_imp(data as *mut std::ffi::c_void, Some(shim)),
            );
        }
    }

    /// Set a function to be called when the text has changed.
    ///
    /// # Remarks
    /// This event will also be launched when you select an item from the list, a sign that the text has changed
    /// in the edit box. See Validate texts and GUI Events.
    pub fn on_change<F>(&self, handler: F)
    where
        F: FnMut(&mut Combo, &Event) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (Box<dyn FnMut(&mut Combo, &Event)>, *mut nappgui_sys::Combo);
            let f = &mut *(*data).0;
            let mut obj = Combo { inner: (*data).1 };
            let ev = Event::new(event as _);
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut obj, &ev)));
        }

        let cb: Box<dyn FnMut(&mut Combo, &Event)> = Box::new(handler);

        let data: *mut (Box<dyn FnMut(&mut Combo, &Event)>, *mut nappgui_sys::Combo) =
            Box::into_raw(Box::new((cb, self.inner)));

        unsafe {
            combo_OnChange(
                self.inner,
                listener_imp(data as *mut std::ffi::c_void, Some(shim)),
            );
        }
    }

    /// Set the combo edit text.
    pub fn text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_text(self.inner, text.as_ptr());
        }
    }

    /// Set text alignment.
    pub fn align(&self, align: align_t) {
        unsafe {
            combo_align(self.inner, align);
        }
    }

    /// Assign a tooltip to the control combo.
    pub fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_tooltip(self.inner, text.as_ptr());
        }
    }

    /// Set the color of the combo text.
    pub fn color(&self, color: u32) {
        unsafe {
            combo_color(self.inner, color);
        }
    }

    /// Sets the color of the text, when the control has the keyboard focus.
    pub fn color_focus(&self, color: u32) {
        unsafe {
            combo_color_focus(self.inner, color);
        }
    }

    /// Set the background color.
    pub fn bgcolor(&self, color: u32) {
        unsafe {
            combo_bgcolor(self.inner, color);
        }
    }

    /// Sets the background color when the control has keyboard focus.
    pub fn bgcolor_focus(&self, color: u32) {
        unsafe {
            combo_bgcolor_focus(self.inner, color);
        }
    }

    /// Set an explanatory text for when the control is blank.
    pub fn phtext(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_phtext(self.inner, text.as_ptr());
        }
    }

    /// Set the color of the placeholder text.
    pub fn phcolor(&self, color: u32) {
        unsafe {
            combo_phcolor(self.inner, color);
        }
    }

    /// Set the font style for the placeholder.
    pub fn phstyle(&self, style: u32) {
        unsafe {
            combo_phstyle(self.inner, style);
        }
    }

    /// Get control text.
    pub fn get_text(&self, index: u32) -> String {
        let text = unsafe { combo_get_text(self.inner, index) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Gets the number of items in the dropdown list.
    pub fn count(&self) -> u32 {
        unsafe { combo_count(self.inner) }
    }

    /// Add a new item to the drop-down list.
    pub fn add_elem(&self, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_add_elem(self.inner, text.as_ptr(), image.inner);
        }
    }

    /// Edit an item from the drop-down list.
    pub fn set_elem(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_set_elem(self.inner, index, text.as_ptr(), image.inner);
        }
    }

    /// Insert an item in the drop-down list.
    pub fn ins_elem(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_ins_elem(self.inner, index, text.as_ptr(), image.inner);
        }
    }

    /// Remove an item from the drop-down list.
    pub fn del_elem(&self, index: u32) {
        unsafe {
            combo_del_elem(self.inner, index);
        }
    }

    /// Prevents duplicate texts from the drop-down list.
    pub fn duplicates(&self, duplicates: bool) {
        unsafe {
            combo_duplicates(self.inner, duplicates as i8);
        }
    }
}
