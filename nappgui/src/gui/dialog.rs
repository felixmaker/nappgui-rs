use std::ffi::CString;

use nappgui_sys::{align_t, comwin_color, comwin_open_file, comwin_save_file, listener_imp};

use crate::{core::event::Event, draw_2d::Color};

use super::window::Window;

/// Launch the open file dialog.
pub fn open_file(window: &Window, ftypes: &[&str], size: u32, start_dir: &str) -> String {
    let mut types: Vec<*const i8> = Vec::new();
    for ftype in ftypes {
        let cstr = CString::new(*ftype).unwrap();
        types.push(cstr.as_ptr());
    }
    let start_dir = CString::new(start_dir).unwrap();
    let file =
        unsafe { comwin_open_file(window.inner, types.as_mut_ptr(), size, start_dir.as_ptr()) };
    let file = unsafe { std::ffi::CStr::from_ptr(file) };
    file.to_string_lossy().into_owned()
}

/// Launch the save file dialog.
pub fn save_file(window: &Window, ftypes: &[&str], size: u32, start_dir: &str) -> String {
    let mut types: Vec<*const i8> = Vec::new();
    for ftype in ftypes {
        let cstr = CString::new(*ftype).unwrap();
        types.push(cstr.as_ptr());
    }
    let start_dir = CString::new(start_dir).unwrap();
    let file =
        unsafe { comwin_save_file(window.inner, types.as_mut_ptr(), size, start_dir.as_ptr()) };
    let file = unsafe { std::ffi::CStr::from_ptr(file) };
    file.to_string_lossy().into_owned()
}

/// Launch the color selection dialog.
pub fn color<F>(
    window: &Window,
    title: &str,
    x: f32,
    y: f32,
    halign: align_t,
    valign: align_t,
    current: Color,
    colors: &[Color],
    n: u32,
    on_change: F,
) where
    F: FnMut(&mut Window, &Event) + 'static,
{
    unsafe extern "C" fn shim(data: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
        let data = data as *mut (
            Box<dyn FnMut(&mut Window, &Event)>,
            *mut nappgui_sys::Window,
        );
        let f = &mut *(*data).0;
        let mut window = Window { inner: (*data).1 };
        let ev = Event::new(event);
        let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut window, &ev)));
    }

    let cb: Box<dyn FnMut(&mut Window, &Event)> = Box::new(on_change);

    let data: *mut (
        Box<dyn FnMut(&mut Window, &Event)>,
        *mut nappgui_sys::Window,
    ) = Box::into_raw(Box::new((cb, window.inner)));

    let title = CString::new(title).unwrap();
    let mut colors: Vec<u32> = colors.iter().map(|color| color.inner).collect();

    unsafe {
        comwin_color(
            window.inner,
            title.as_ptr(),
            x,
            y,
            halign,
            valign,
            current.inner,
            colors.as_mut_ptr(),
            n,
            listener_imp(data as *mut std::ffi::c_void, Some(shim)),
        );
    }
}
