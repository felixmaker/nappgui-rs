use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, CString},
};

use crate::error::NappguiError;

/// DBind type.
pub enum DBind {
    /// Struct.
    Struct(DBindStruct),
    /// Enum.
    Enum(DBindEnum),
    /// Alias.
    Alias(DBindAlias),
}

impl DBind {
    /// Registers DBind.
    pub(crate) fn register(&self) -> Result<(), NappguiError> {
        match self {
            DBind::Struct(obj) => obj.register(),
            DBind::Enum(obj) => obj.register(),
            DBind::Alias(obj) => obj.register(),
        }
    }

    /// Returns the type of the DBind.
    pub(crate) fn ty(&self) -> CString {
        match self {
            DBind::Struct(obj) => obj.ty.clone(),
            DBind::Enum(obj) => obj.ty.clone(),
            DBind::Alias(obj) => obj.ty.clone(),
        }
    }
}

/// DBind alias.
pub struct DBindAlias {
    /// Alias type.
    pub ty: CString,
    /// Alias.
    pub alias: CString,
    /// Type size in bytes.
    pub size: u16,
    /// Alias size in bytes.
    pub alias_size: u16,
}

impl DBindAlias {
    /// Registers the alias.
    pub(crate) fn register(&self) -> Result<(), NappguiError> {
        let result =
            unsafe { nappgui_sys::dbind_alias_imp(self.ty.as_ptr(), self.alias.as_ptr(), self.size, self.alias_size) };
        convert_dbindst_t_to_nappgui_result(result)?;
        Ok(())
    }
}

/// DBind Enum
pub struct DBindEnum {
    /// Enum name.
    pub ty: CString,
    /// Enum variants.
    pub variants: RefCell<Vec<DBindVariant>>,
}

/// DBind variant.
pub struct DBindVariant {
    /// Variant name.
    pub name: CString,
    /// Variant value.
    pub value: i32,
    /// Variant alias.
    pub alias: CString,
}

impl DBindEnum {
    /// Registers the enum.
    pub(crate) fn register(&self) -> Result<(), NappguiError> {
        for variant in self.variants.borrow().iter() {
            let result = unsafe {
                let DBindVariant { name, value, alias } = variant;
                nappgui_sys::dbind_enum_imp(self.ty.as_ptr(), name.as_ptr(), *value, alias.as_ptr())
            };
            convert_dbindst_t_to_nappgui_result(result)?;
        }
        Ok(())
    }
}

/// DBind struct.
pub struct DBindStruct {
    /// Struct type
    pub ty: CString,
    /// Struct size
    pub size: u16,
    /// Struct fields
    pub fields: RefCell<HashMap<CString, DBindField>>,
}

/// DBind field.
pub struct DBindField {
    /// Field name.
    pub(crate) name: CString,
    /// Field type.
    pub(crate) ty: CString,
    /// Field offset in bytes.
    pub(crate) offset: u16,
    /// Field size in bytes.
    pub(crate) size: u16,
    /// Field range.
    range: Option<(NappguiNumber, NappguiNumber)>,
    /// Field precision.
    precision: Option<NappguiNumber>,
    /// Field increment.  
    increment: Option<NappguiNumber>,
    /// Field suffix.
    suffix: Option<CString>,
    /// Field default value.
    _default: Option<CString>,
}

impl DBindStruct {
    /// Registers the struct.
    pub(crate) fn register(&self) -> Result<(), NappguiError> {
        let struct_type = self.ty.as_ptr();

        for (_name, field) in self.fields.borrow().iter() {
            let result = unsafe {
                nappgui_sys::dbind_imp(
                    struct_type,
                    self.size,
                    field.name.as_ptr(),
                    field.ty.as_ptr(),
                    field.offset,
                    field.size,
                )
            };

            let _ = convert_dbindst_t_to_nappgui_result(result); // Todo!!

            if let Some((min, max)) = &field.range {
                unsafe { nappgui_sys::dbind_range_imp(struct_type, field.name.as_ptr(), min.as_ptr(), max.as_ptr()) }
            }

            if let Some(precision) = &field.precision {
                unsafe { nappgui_sys::dbind_precision_imp(struct_type, field.name.as_ptr(), precision.as_ptr()) }
            }

            if let Some(increment) = &field.increment {
                unsafe { nappgui_sys::dbind_increment_imp(struct_type, field.name.as_ptr(), increment.as_ptr()) }
            }

            if let Some(suffix) = &field.suffix {
                unsafe { nappgui_sys::dbind_suffix_imp(struct_type, field.name.as_ptr(), suffix.as_ptr()) }
            }
        }

        Ok(())
    }

    /// Create a dbind to Struct.
    pub fn new(ty: &CStr, size: u16) -> Self {
        Self {
            ty: ty.to_owned(),
            size,
            fields: RefCell::new(HashMap::new()),
        }
    }

    /// Add a field to struct. If the field exists, then overwrite.
    pub fn add_field(&self, name: &CStr, ty: &CStr, offset: u16, size: u16) {
        self.fields.borrow_mut().insert(
            name.to_owned(),
            DBindField {
                name: name.to_owned(),
                ty: ty.to_owned(),
                offset,
                size,
                range: None,
                precision: None,
                increment: None,
                suffix: None,
                _default: None,
            },
        );
    }
}

/// NAppGUI number type.
#[allow(dead_code)]
pub(crate) enum NappguiNumber {
    /// Integer.
    Integer(NappguiInt),
    /// Float.
    Float(NappguiFloat),
}

impl NappguiNumber {
    /// Converts NappguiNumber to *const u8.
    pub(crate) fn as_ptr(&self) -> *const u8 {
        match self {
            NappguiNumber::Integer(value) => value as *const _ as *const u8,
            NappguiNumber::Float(value) => value as *const _ as *const u8,
        }
    }
}

/// NAppGUI integer type.
pub type NappguiInt = i64;

/// NAppGUI real type.
pub type NappguiFloat = f64;

/// NAppGUI boolean type.
pub type NappguiBoolean = bool;

/// Helper function to convert dbindst_t to Result<(), NappguiError>.
fn convert_dbindst_t_to_nappgui_result(result: i32) -> Result<(), NappguiError> {
    if result == nappgui_sys::_dbindst_t_ekDBIND_OK {
        Ok(())
    } else {
        Err(NappguiError::from_dbindst_t(result))
    }
}

thread_local! {
    pub(crate) static DBIND: RefCell<HashMap<CString, DBind>> = RefCell::new(HashMap::new());
}

impl From<DBindStruct> for DBind {
    fn from(value: DBindStruct) -> Self {
        Self::Struct(value)
    }
}

/// Registers DBind.
pub fn dbind_register<T>(item: T) -> Result<(), NappguiError>
where
    T: Into<DBind>,
{
    let item = item.into();
    item.register()?;
    DBIND.with_borrow_mut(|dbind| dbind.insert(item.ty(), item));
    Ok(())
}

/// Unregisters DBind.
pub fn dbind_unregister(ty: &CStr) -> Result<(), NappguiError> {
    let result = unsafe { nappgui_sys::dbind_unreg_imp(ty.as_ptr()) };
    convert_dbindst_t_to_nappgui_result(result)?;
    DBIND.with_borrow_mut(|dbind| dbind.remove(ty));
    Ok(())
}

pub(crate) fn dbind_struct<F, R>(ty: &CStr, f: F) -> Option<R>
where
    F: FnOnce(&DBindStruct) -> R,
{
    DBIND.with_borrow(|map| {
        map.get(ty).and_then(|obj| match obj {
            DBind::Struct(value) => Some(f(value)),
            _ => None,
        })
    })
}
