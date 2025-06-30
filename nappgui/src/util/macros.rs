macro_rules! listener {
    ($target:expr, $handler:ident, $obj:ty) => {{
        use crate::core::event::Event;
        use std::ffi::c_void;

        unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (Box<dyn FnMut(&mut $obj, &Event)>, *mut c_void);
            let f = &mut *(*data).0;
            let mut obj = <$obj>::new_increment((*data).1 as _);
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
    ($pointer: ty, $wrapper: ty) => {
        pub(crate) fn new(ptr: $pointer) -> Self {
            if ptr.is_null() {
                panic!("pointer `{}` is null", std::any::type_name::<$pointer>());
            }
            Self {
                inner: <$wrapper>::new(ptr),
            }
        }

        pub(crate) fn as_ptr(&self) -> $pointer {
            *self.inner
        }

        #[allow(unused)]
        pub(crate) unsafe fn new_increment(ptr: $pointer) -> Self {
            if ptr.is_null() {
                panic!("pointer `{}` is null", std::any::type_name::<$pointer>());
            }

            let inner = <$wrapper>::new(ptr);
            let inner = <$wrapper>::into_raw(inner);
            Rc::increment_strong_count(inner);
            let inner = Rc::from_raw(inner);
            Self { inner }
        }
    };
}

macro_rules! impl_clone {
    ($type: ty) => {
        impl Clone for $type {
            fn clone(&self) -> Self {
                Self {
                    inner: self.inner.clone(),
                }
            }
        }
    };
}

pub(crate) use callback;
pub(crate) use listener;
pub(crate) use pub_crate_ptr_ops;
