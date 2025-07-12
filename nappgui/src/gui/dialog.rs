use std::ffi::CString;

use nappgui_sys::{comwin_color, comwin_open_file, comwin_save_file};

use crate::{draw_2d::Color, gui::WindowTrait, types::Align, util::macros::listener};

/// Launch the open file dialog.
pub fn open_file<T, I, S>(window: &T, ftypes: I, size: u32, start_dir: &str) -> String
where
    T: WindowTrait,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut types: Vec<*const i8> = Vec::new();
    for ftype in ftypes.into_iter() {
        let cstr = CString::new(ftype.as_ref()).unwrap();
        types.push(cstr.as_ptr());
    }
    let start_dir = CString::new(start_dir).unwrap();
    let file = unsafe {
        comwin_open_file(
            window.as_ptr(),
            types.as_mut_ptr(),
            size,
            start_dir.as_ptr(),
        )
    };
    let file = unsafe { std::ffi::CStr::from_ptr(file) };
    file.to_string_lossy().into_owned()
}

/// Launch the save file dialog.
pub fn save_file<T, I, S>(window: &T, ftypes: I, size: u32, start_dir: &str) -> String
where
    T: WindowTrait,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut types: Vec<*const i8> = Vec::new();
    for ftype in ftypes.into_iter() {
        let cstr = CString::new(ftype.as_ref()).unwrap();
        types.push(cstr.as_ptr());
    }
    let start_dir = CString::new(start_dir).unwrap();
    let file = unsafe {
        comwin_save_file(
            window.as_ptr(),
            types.as_mut_ptr(),
            size,
            start_dir.as_ptr(),
        )
    };
    let file = unsafe { std::ffi::CStr::from_ptr(file) };
    file.to_string_lossy().into_owned()
}

/// Launch the color selection dialog.
pub fn color<T, F>(
    window: &T,
    title: &str,
    x: f32,
    y: f32,
    halign: Align,
    valign: Align,
    current: Color,
    colors: &[Color],
    n: u32,
    on_change: F,
) where
    T: WindowTrait,
    F: FnMut() + 'static,
{
    let listener = listener!(on_change, ());

    let title = CString::new(title).unwrap();
    let mut colors: Vec<u32> = colors.iter().map(|color| color.inner).collect();

    unsafe {
        comwin_color(
            window.as_ptr(),
            title.as_ptr(),
            x,
            y,
            halign as _,
            valign as _,
            current.inner,
            colors.as_mut_ptr(),
            n,
            listener,
        );
    }
}
