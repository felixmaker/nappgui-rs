use std::ffi::{c_void, CString};

use crate::{core::Stream, error::NappguiError};

/// Helper function to convert dbindst_t to Result<(), NappguiError>.
fn convert_dbindst_t_to_nappgui_result(result: i32) -> Result<(), NappguiError> {
    if result == nappgui_sys::_dbindst_t_ekDBIND_OK {
        Ok(())
    } else {
        Err(NappguiError::from_dbindst_t(result))
    }
}

/// Adds a field from a structure to its internal table within DBind.
pub fn dbind_imp(
    type_: &str,
    size: u16,
    mname: &str,
    mtype: &str,
    moffset: u16,
    msize: u16,
) -> Result<(), NappguiError> {
    let type_ = CString::new(type_).unwrap();
    let mname = CString::new(mname).unwrap();
    let mtype = CString::new(mtype).unwrap();
    let result = unsafe {
        nappgui_sys::dbind_imp(
            type_.as_ptr(),
            size,
            mname.as_ptr(),
            mtype.as_ptr(),
            moffset,
            msize,
        )
    };
    convert_dbindst_t_to_nappgui_result(result)
}

/// Registers a value of type enum.
///
/// # Remark
/// dbind_enum(mode_t, ekIMAGE_ANALISYS, "Image Analisys") will use the string "Image Analisys" instead of "ekIMAGE_ANALISYS"
/// for those I/O or interface operations that require displaying enumeration literals. For example, to populate the fields of
/// a PopUp linked to a data field.
pub fn dbind_enum_imp(
    type_: &str,
    name: &str,
    value: i32,
    alias: &str,
) -> Result<(), NappguiError> {
    let type_ = CString::new(type_).unwrap();
    let name = CString::new(name).unwrap();
    let alias = CString::new(alias).unwrap();
    let result = unsafe {
        nappgui_sys::dbind_enum_imp(type_.as_ptr(), name.as_ptr(), value, alias.as_ptr())
    };
    convert_dbindst_t_to_nappgui_result(result)
}

/// Registers an alias for a data type (typedef).
pub fn dbind_alias_imp(
    type_: &str,
    alias: &str,
    type_size: u16,
    alias_size: u16,
) -> Result<(), NappguiError> {
    let type_ = CString::new(type_).unwrap();
    let alias = CString::new(alias).unwrap();
    let result = unsafe {
        nappgui_sys::dbind_alias_imp(type_.as_ptr(), alias.as_ptr(), type_size, alias_size)
    };
    convert_dbindst_t_to_nappgui_result(result)
}

/// Removes a data type from the DBind record.
pub fn dbind_unreg_imp(type_: &str) -> Result<(), NappguiError> {
    let type_ = CString::new(type_).unwrap();
    let result = unsafe { nappgui_sys::dbind_unreg_imp(type_.as_ptr()) };
    convert_dbindst_t_to_nappgui_result(result)
}

/// Creates an object of registered type, initializing its fields with the default values.
pub fn dbind_create_imp(type_: &str) -> *mut u8 {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_create_imp(type_.as_ptr()) }
}

/// Copies an object of registered type.
pub fn dbind_copy_imp(obj: *mut u8, type_: &str) -> *mut u8 {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_copy_imp(obj, type_.as_ptr()) }
}

/// Initializes the fields of a registered type object with the default values.
pub fn dbind_init_imp(obj: *mut u8, type_: &str) {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_init_imp(obj, type_.as_ptr()) }
}

/// Frees the memory reserved by the fields of an object of registered type, but does not destroy the object itself.
pub fn dbind_remove_imp(obj: *mut u8, type_: &str) {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_remove_imp(obj, type_.as_ptr()) }
}

/// Destroys an object of registered type. Memory allocated to fields and sub-objects will also be freed recursively.
pub fn dbind_destroy_imp(obj: *mut *mut u8, type_: &str) {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_destopt_imp(obj, type_.as_ptr()) }
}

/// Compares two objects of registered type.
///
/// Returns -1, 1 or 0 if obj1 is less than, greater than or equal to obj2.
pub fn dbind_cmp_imp(obj1: *mut u8, obj2: *mut u8, type_: &str) -> i32 {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_cmp_imp(obj1, obj2, type_.as_ptr()) }
}

/// Checks if two objects of registered type are the same.
pub fn dbind_equ_imp(obj1: *mut u8, obj2: *mut u8, type_: &str) -> bool {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_equ_imp(obj1, obj2, type_.as_ptr()) != 0 }
}

/// Creates a registered type object from data read from a stream.
pub fn dbind_read(stream: &Stream, type_: &str) -> *mut u8 {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_read_imp(stream.inner, type_.as_ptr()) }
}

/// Writes the contents of a registered type object to a write stream.
pub fn dbind_write_imp(stream: &Stream, obj: *const c_void, type_: &str) {
    let type_ = CString::new(type_).unwrap();
    unsafe { nappgui_sys::dbind_write_imp(stream.inner, obj, type_.as_ptr()) }
}

/// Sets the default value of a field.
pub fn dbind_default_imp(type_: &str, mname: &str, value: *const u8) {
    let type_ = CString::new(type_).unwrap();
    let mname = CString::new(mname).unwrap();
    unsafe { nappgui_sys::dbind_default_imp(type_.as_ptr(), mname.as_ptr(), value) }
}

/// Sets the maximum and minimum value in numeric fields.
pub fn dbind_range_imp(type_: &str, mname: &str, min: *const u8, max: *const u8) {
    let type_ = CString::new(type_).unwrap();
    let mname = CString::new(mname).unwrap();
    unsafe { nappgui_sys::dbind_range_imp(type_.as_ptr(), mname.as_ptr(), min, max) }
}

/// Sets the jump between two consecutive real values.
pub fn dbind_precision_imp(type_: &str, mname: &str, precision: *const u8) {
    let type_ = CString::new(type_).unwrap();
    let mname = CString::new(mname).unwrap();
    unsafe { nappgui_sys::dbind_precision_imp(type_.as_ptr(), mname.as_ptr(), precision) }
}

/// Sets the increment of a numeric value, for example, when clicking an UpDown control.
pub fn dbind_increment_imp(type_: &str, mname: &str, increment: *const u8) {
    let type_ = CString::new(type_).unwrap();
    let mname = CString::new(mname).unwrap();
    unsafe { nappgui_sys::dbind_increment_imp(type_.as_ptr(), mname.as_ptr(), increment) }
}

/// Sets a suffix that will be added to the numeric value when converting to text.
pub fn dbind_suffix_imp(type_: &str, mname: &str, suffix: &str) {
    let type_ = CString::new(type_).unwrap();
    let mname = CString::new(mname).unwrap();
    let suffix = CString::new(suffix).unwrap();
    unsafe { nappgui_sys::dbind_suffix_imp(type_.as_ptr(), mname.as_ptr(), suffix.as_ptr()) }
}

/// Adds a field from a structure to its internal table within DBind.
#[macro_export]
macro_rules! dbind {
    ($struct: ty, $field: ident, $field_type: ty, $bind_type: literal) => {
        nappgui::core::dbind::dbind_imp(
            stringify!($struct),
            size_of::<$struct>() as _,
            stringify!($field),
            $bind_type,
            offset_of!($struct, $field) as _,
            size_of::<$field_type>() as _,
        )
    };
}

/// Registers a value of type enum.
///
/// # Remark
/// dbind_enum(mode_t, ekIMAGE_ANALISYS, "Image Analisys") will use the string "Image Analisys" instead of "ekIMAGE_ANALISYS"
/// for those I/O or interface operations that require displaying enumeration literals. For example, to populate the fields of
/// a PopUp linked to a data field.
#[macro_export]
macro_rules! dbind_enum {
    ($enum: ty, $value: ident, $alias: literal) => {
        nappgui::core::dbind::dbind_enum_imp(
            stringify!($enum),
            stringify!($value),
            <$enum>::$value as _,
            $alias,
        )
    };
}

/// Sets the maximum and minimum value in numeric fields.
#[macro_export]
macro_rules! dbind_range {
    ($struct: ty, $field: ident, $min: literal, $max: literal) => {
        nappgui::core::dbind::dbind_range_imp(
            stringify!($struct),
            stringify!($field),
            $min.to_ne_bytes().as_ptr(),
            $max.to_ne_bytes().as_ptr(),
        );
    };
}

/// Sets the increment of a numeric value, for example, when clicking an UpDown control.
#[macro_export]
macro_rules! dbind_increment {
    ($struct: ty, $field: ident, $increment: literal) => {
        nappgui::core::dbind::dbind_increment_imp(
            stringify!($struct),
            stringify!($field),
            $increment.to_ne_bytes().as_ptr(),
        );
    };
}
