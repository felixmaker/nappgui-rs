macro_rules! listener {
    ($target:expr, $handler:ident, $obj:ty) => {{
        use crate::core::event::Event;
        use std::ffi::c_void;

        unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (Box<dyn FnMut(&mut $obj, &Event)>, *mut c_void);
            let f = &mut *(*data).0;
            let mut obj = <$obj>::new_no_drop((*data).1 as _);
            let event = Event::new(event);
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut obj, &event)));
        }

        let cb: Box<dyn FnMut(&mut $obj, &Event)> = Box::new($handler);

        let data = Box::into_raw(Box::new((cb, $target)));

        unsafe { nappgui_sys::listener_imp(data as *mut c_void, Some(shim)) }
    }};
}

macro_rules! callback {
    (
        $(#[$meta:meta])*
        $vis:vis $func:ident($obj:ty) => $c_func:ident
    ) => {
        $(#[$meta])*
        $vis fn $func<F>(&self, handler: F)
        where
            F: FnMut(&mut $obj, &crate::core::event::Event) + 'static,
        {
            let listener = crate::util::macros::listener!(self.as_ptr(), handler, $obj);
            unsafe {
                $c_func(self.as_ptr(), listener);
            }
        }
    };
    (
        $(
            $(#[$meta:meta])*
            $vis:vis $func:ident($obj:ty) => $c_func:ident
        );*$(;)?
    ) => {
        $(
            callback!(
                $(#[$meta])*
                $vis $func($obj) => $c_func
            );
        )*
    }
}

macro_rules! pub_crate_ptr_ops {
    ($pointer: ty) => {
        #[allow(unused)]
        pub(crate) fn new(ptr: $pointer) -> Self {
            if ptr.is_null() {
                panic!("pointer `{}` is null", std::any::type_name::<$pointer>());
            }
            Self {
                inner: std::rc::Rc::<$pointer>::new(ptr),
            }
        }

        #[allow(unused)]
        pub(crate) fn new_option(ptr: $pointer) -> Option<Self> {
            if ptr.is_null() {
                None
            } else {
                Some(Self::new(ptr))
            }
        }

        #[allow(unused)]
        pub(crate) unsafe fn new_option_no_drop(ptr: $pointer) -> Option<Self> {
            if ptr.is_null() {
                None
            } else {
                Some(Self::new_no_drop(ptr))
            }
        }

        #[allow(unused)]
        pub(crate) fn as_ptr(&self) -> $pointer {
            *self.inner
        }

        #[allow(unused)]
        pub(crate) unsafe fn new_no_drop(ptr: $pointer) -> Self {
            if ptr.is_null() {
                panic!("pointer `{}` is null", std::any::type_name::<$pointer>());
            }

            let inner = std::rc::Rc::<$pointer>::new(ptr);
            let inner = std::rc::Rc::<$pointer>::into_raw(inner);
            std::rc::Rc::increment_strong_count(inner);
            let inner = std::rc::Rc::from_raw(inner);
            Self { inner }
        }
    };
}

macro_rules! impl_i32_to_enum {
    ($type: ty, $range: expr) => {
        impl TryFrom<i32> for $type {
            type Error = crate::error::NappguiError;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if !($range).contains(&value) {
                    return Err(crate::error::NappguiError::Internal(
                        crate::error::NappguiErrorKind::UndefinedEnumTransmute,
                    ));
                } else {
                    Ok(unsafe { std::mem::transmute(value) })
                }
            }
        }
    };
}

macro_rules! impl_gui_control {
    ($type: ty, $func: ident) => {
        impl crate::gui::GuiControl for $type {
            fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl {
                *self.inner as _
            }

            fn from_control_ptr(ptr: *mut nappgui_sys::GuiControl) -> Option<Self> {
                unsafe {
                    let combo = nappgui_sys::$func(ptr);
                    Self::new_option_no_drop(combo)
                }
            }
        }
    };
}

pub(crate) use callback;
pub(crate) use listener;
pub(crate) use pub_crate_ptr_ops;

pub(crate) use impl_gui_control;
pub(crate) use impl_i32_to_enum;
