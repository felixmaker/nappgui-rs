use std::ffi::{CStr, CString};

use nappgui_sys::{comwin_color, comwin_open_file, comwin_save_file, comwin_select_dir};

use crate::{draw_2d::Color, gui::Window, types::Align, util::macros::listener};

/// Launches the directory selection dialog.
///
/// # Remarks
/// It will be launched in modal. parent will remain locked until the dialog is accepted.
pub fn select_dir(window: &Window, caption: &str, start_dir: &str) -> Option<String> {
    let caption = CString::new(caption).unwrap();
    let start_dir = CString::new(start_dir).unwrap();
    let dir = unsafe { comwin_select_dir(window.as_ptr(), caption.as_ptr(), start_dir.as_ptr()) };
    if dir.is_null() {
        return None;
    }
    let dir = unsafe { CStr::from_ptr(dir) };
    Some(dir.to_string_lossy().into_owned())
}

/// Launch the open file dialog.
pub fn open_file<I, S>(
    window: &Window,
    caption: &str,
    ftypes: I,
    size: u32,
    start_dir: &str,
    filename: &str,
) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut types: Vec<*const i8> = Vec::new();
    for ftype in ftypes.into_iter() {
        let cstr = CString::new(ftype.as_ref()).unwrap();
        types.push(cstr.as_ptr());
    }
    let caption = CString::new(caption).unwrap();
    let start_dir = CString::new(start_dir).unwrap();
    let filename = CString::new(filename).unwrap();
    let file = unsafe {
        comwin_open_file(
            window.as_ptr(),
            caption.as_ptr(),
            types.as_mut_ptr(),
            size,
            start_dir.as_ptr(),
            filename.as_ptr(),
        )
    };
    if file.is_null() {
        return None;
    }
    let file = unsafe { std::ffi::CStr::from_ptr(file) };
    Some(file.to_string_lossy().into_owned())
}

/// Launch the save file dialog.
pub fn save_file<I, S>(
    window: &Window,
    caption: &str,
    ftypes: I,
    size: u32,
    start_dir: &str,
    filename: &str,
) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut types: Vec<*const i8> = Vec::new();
    for ftype in ftypes.into_iter() {
        let cstr = CString::new(ftype.as_ref()).unwrap();
        types.push(cstr.as_ptr());
    }
    let caption = CString::new(caption).unwrap();
    let start_dir = CString::new(start_dir).unwrap();
    let filename = CString::new(filename).unwrap();

    let file = unsafe {
        comwin_save_file(
            window.as_ptr(),
            caption.as_ptr(),
            types.as_mut_ptr(),
            size,
            start_dir.as_ptr(),
            filename.as_ptr(),
        )
    };
    if file.is_null() {
        return None;
    }
    let file = unsafe { std::ffi::CStr::from_ptr(file) };
    Some(file.to_string_lossy().into_owned())
}

/// Launch the color selection dialog.
pub fn color<T, F>(
    window: &Window,
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
    F: FnMut() + 'static,
{
    let listener = todo!();

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
