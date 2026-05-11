use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::gui::*;

pub(crate) struct ObjectInner<T, P> {
    pub(crate) ptr: Cell<*mut T>,
    pub(crate) props: P,
}


/// Macro to implement the `Object` trait for widget types.
macro_rules! define_object {
    ($type:ident, $inner_type:ident, $nappgui_type:ident, $props:ident) => {
        pub(crate) type $inner_type = crate::gui::ObjectInner<nappgui_sys::$nappgui_type, $props>;

        #[doc = concat!("The ", stringify!($type), " object.")]
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $type(crate::gui::GUID);

        impl $type {
            pub(crate) fn from_raw(ptr: *mut nappgui_sys::$nappgui_type) -> Self {
                assert!(
                    !ptr.is_null(),
                    concat!("Try to wrap a null pointer to ", stringify!($type), " object.")
                );
                let inner = crate::gui::ObjectInner {
                    ptr: std::cell::Cell::new(ptr),
                    props: $props::default(),
                };
                let id = ptr as _;
                let id = crate::gui::global_object_insert(id, inner);
                Self(id)
            }

            pub(crate) fn inner<F, R>(&self, f: F) -> Option<R>
            where
                F: FnOnce(&$inner_type) -> R,
            {
                crate::gui::global_object(self.0, f)
            }

            /// Returns a pointer to the object. Can be null.
            pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::$nappgui_type {
                self.inner(|inner| inner.ptr.get())
                    .unwrap_or(std::ptr::null_mut())
            }
        }
    };
}

/// Callback type for event handlers.
pub type Callback<T, R = ()> = RefCell<Option<Rc<dyn Fn(&T) -> R + 'static>>>;
/// Void callback type for event handlers.
pub type VoidCallback<R = ()> = RefCell<Option<Rc<dyn Fn() -> R + 'static>>>;

pub(crate) type GUID = usize;

thread_local! {
    pub(crate)  static GLOBAL_OBJECTS: RefCell<HashMap<GUID, Rc<dyn Any + 'static>>> = Default::default();
}

pub(crate) fn global_object_insert<T>(object_id: GUID, object: T) -> GUID
where
    T: Any + 'static,
{
    let object = Rc::new(object);
    GLOBAL_OBJECTS.with_borrow_mut(|objects| objects.insert(object_id, object));
    object_id
}

pub(crate) fn global_object<T, F, R>(uid: GUID, f: F) -> Option<R>
where
    T: Any + 'static,
    F: FnOnce(&T) -> R,
{
    let object = GLOBAL_OBJECTS.with_borrow(|objects| objects.get(&uid).map(|x| x.clone()))?;
    if let Ok(object) = object.downcast::<T>() {
        Some(f(object.as_ref()))
    } else {
        None
    }
}

// pub(crate) fn global_object_remove<T, F, R>(uid: GUID) {
//     GLOBAL_OBJECTS.with_borrow_mut(|objects| objects.remove(&uid));
// }

macro_rules! listener {
    ($ptr: expr, $type:ident, $member:ident($($params: ty)?) $(-> $return:ty)?) => {{
        #[allow(unused)]
        extern "C" fn shim(obj: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let Some(Some(f)) =
                crate::gui::global_object(obj as _, |x: &$type| x.props.$member.borrow().clone())
            else {
                return;
            };
            let event = crate::core::event::Event::new(event);
            $(
                let params = unsafe { event.params::<$params>() };
            )?
            if let Ok(r) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f($(&(params as $params))?))) {
                $( unsafe { event.result(r as $return); })?
            }
        }

        let listener = unsafe { nappgui_sys::listener_imp($ptr as _, Some(shim)) };
        listener
    }};
}

pub(crate) use define_object;
pub(crate) use listener;
