macro_rules! listener {
    ($handler:expr, ($($params: ty)?) $(-> $return:ty)?) => {{
        use std::ffi::c_void;

        unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut Box<dyn FnMut($(& $params)?) $(-> $return)?>;
            let f = &mut *data;
            #[allow(unused)]
            let event = crate::core::event::Event::new(event);
            $(
                let params = event.params::<$params>().unwrap();
            )?
            #[allow(unused)]
            if let Ok(r) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f($(&(params as $params))?))) {
                $( event.result(r as $return); )?
            }
        }

        let cb: Box<dyn FnMut($(& $params)?) $(-> $return)?> = Box::new($handler);
        let data: *mut Box<dyn FnMut($(& $params)?) $(-> $return)?> = Box::into_raw(Box::new(cb));

        let listener = unsafe { nappgui_sys::listener_imp(data as *mut c_void, Some(shim)) };
        listener
    }};
}

// macro_rules! callback {
//     (
//         $(#[$meta:meta])*
//         $vis:vis $func:ident($obj:ty) => $c_func:ident
//     ) => {
//         $(#[$meta])*
//         $vis fn $func<F>(&self, handler: F)
//         where
//             F: FnMut(&mut $obj, &crate::core::event::Event) + 'static,
//         {
//             let listener = crate::util::macros::listener!(self.as_ptr(), handler, $obj);
//             unsafe {
//                 $c_func(self.as_ptr(), listener);
//             }
//         }
//     };
//     (
//         $(
//             $(#[$meta:meta])*
//             $vis:vis $func:ident($obj:ty) => $c_func:ident
//         );*$(;)?
//     ) => {
//         $(
//             callback!(
//                 $(#[$meta])*
//                 $vis $func($obj) => $c_func
//             );
//         )*
//     }
// }

macro_rules! callback {
    (
        $(#[$meta:meta])*
        $vis:vis $func:ident($($params: ty)?) $(-> $return:ty)? => $c_func:ident
    ) => {
        $(#[$meta])*
        $vis fn $func<F>(&self, handler: F)
        where
            F: FnMut($(& $params)?) $(-> $return)? + 'static,
        {
            use std::ffi::c_void;

            unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
                let data = data as *mut Box<dyn FnMut($(& $params)?) $(-> $return)?>;
                let f = &mut *data;
                #[allow(unused)]
                let event = crate::core::event::Event::new(event);
                $(
                    let params = event.params::<$params>().unwrap();
                )?
                #[allow(unused)]
                if let Ok(r) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f($(&(params as $params))?))) {
                    $( event.result(r as $return); )?
                }
            }

            let cb: Box<dyn FnMut($(& $params)?) $(-> $return)?> = Box::new(handler);
            let data: *mut Box<dyn FnMut($(& $params)?) $(-> $return)?> = Box::into_raw(Box::new(cb));

            let listener = unsafe { nappgui_sys::listener_imp(data as *mut c_void, Some(shim)) };

            unsafe {
                $c_func(self.as_ptr(), listener);
            }
        }
    };
    (
        $(
            $(#[$meta:meta])*
            $vis:vis $func:ident($($params: ty)?) $(-> $return:ty)? => $c_func:ident
        );*$(;)?
    ) => {
        $(
            callback!(
                $(#[$meta])*
                $vis $func($($params)?) $(-> $return)? => $c_func
            );
        )*
    }
}

// macro_rules! pub_crate_ptr_ops {
//     ($pointer: ty) => {
//         #[allow(unused)]
//         pub(crate) fn from_raw(ptr: $pointer) -> Self {
//             if ptr.is_null() {
//                 panic!("pointer `{}` is null", std::any::type_name::<$pointer>());
//             }
//             Self {
//                 inner: std::rc::Rc::<$pointer>::new(ptr),
//             }
//         }

//         #[allow(unused)]
//         pub(crate) unsafe fn from_raw_no_drop(ptr: $pointer) -> Self {
//             if ptr.is_null() {
//                 panic!("pointer `{}` is null", std::any::type_name::<$pointer>());
//             }

//             let inner = std::rc::Rc::<$pointer>::new(ptr);
//             let inner = std::rc::Rc::<$pointer>::into_raw(inner);
//             std::rc::Rc::increment_strong_count(inner);
//             let inner = std::rc::Rc::from_raw(inner);
//             Self { inner }
//         }

//         #[allow(unused)]
//         pub(crate) fn from_raw_option(ptr: $pointer) -> Option<Self> {
//             if ptr.is_null() {
//                 None
//             } else {
//                 Some(Self::from_raw(ptr))
//             }
//         }

//         #[allow(unused)]
//         pub(crate) unsafe fn from_raw_no_drop_option(ptr: $pointer) -> Option<Self> {
//             if ptr.is_null() {
//                 None
//             } else {
//                 Some(Self::from_raw_no_drop(ptr))
//             }
//         }

//         #[allow(unused)]
//         pub(crate) fn as_ptr(&self) -> $pointer {
//             *self.inner
//         }
//     };
// }

pub(crate) use callback;
pub(crate) use listener;
// pub(crate) use pub_crate_ptr_ops;
