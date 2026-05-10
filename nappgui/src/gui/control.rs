// use std::{
//     any::Any,
//     cell::{Cell, RefCell},
//     collections::HashMap,
//     rc::Rc,
// };

// use crate::gui::*;

// pub(crate) struct ControlInner<T, P> {
//     pub(crate) ptr: Cell<*mut T>,
//     pub(crate) props: P,
// }

// /// Macro to implement the `Control` trait for widget types.
// macro_rules! define_object {
//     ($type:ident, $inner_type:ident, $nappgui_type:ident, $props:ident) => {
//         pub(crate) type $inner_type = crate::gui::ControlInner<nappgui_sys::$nappgui_type, $props>;

//         #[doc = concat!("The ", stringify!($type), " control.")]
//         #[repr(transparent)]
//         #[derive(Clone)]
//         pub struct $type(crate::gui::GUID);

//         impl $type {
//             pub(crate) fn from_raw(ptr: *mut nappgui_sys::$nappgui_type) -> Self {
//                 let inner = std::rc::Rc::new(crate::gui::ControlInner {
//                     ptr: std::cell::Cell::new(ptr),
//                     props: Default::default(),
//                 });
//                 let id = crate::gui::global_control_insert(inner);
//                 Self(id)
//             }

//             pub(crate) fn inner<F, R>(&self, f: F) -> Option<R>
//             where
//                 F: FnOnce(&$inner_type) -> R,
//             {
//                 crate::gui::global_control(self.0, f)
//             }

//             /// Returns a pointer to the control. Can be null.
//             pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::$nappgui_type {
//                 self.inner(|inner| inner.as_ptr()).unwrap_or(std::ptr::null_mut())
//             }
//         }
//     };
// }

// pub(crate) use define_object;

// pub type Callback<T, R = ()> = RefCell<Option<Rc<dyn Fn(&T) -> R + 'static>>>;

// // /// Control.
// // pub(crate) enum Control {
// //     Button(ButtonInner),
// //     Combo(ComboInner),
// //     Edit(EditInner),
// //     ImageView(ImageViewInner),
// //     Label(LabelInner),
// //     Panel(PanelInner),
// //     ListBox(ListBoxInner),
// //     PopUp(PopUpInner),
// //     Progress(ProgressInner),
// //     Slider(SliderInner),
// //     SplitView(SplitViewInner),
// //     TableView(TableViewInner),
// //     TextView(TextViewInner),
// //     UpDown(UpDownInner),
// //     View(ViewInner),
// //     WebView(WebViewInner),
// //     Line(LineInner),
// // }

// thread_local! {
//     static GLOBAL_UID: Cell<GUID> = Cell::new(0);
//     static GLOBAL_CONTROLS: RefCell<HashMap<GUID, Rc<dyn Any + 'static>>> = Default::default();
// }

// fn global_id() -> GUID {
//     GLOBAL_UID.with(|uid| {
//         let id = uid.get() + 1;
//         uid.set(id);
//         id
//     })
// }

// pub(crate) unsafe fn global_control_set_id(control: *mut nappgui_sys::GuiControl, uid: GUID) {
//     nappgui_sys::guicontrol_tag(control, uid)
// }

// pub(crate) unsafe fn global_control_id(control: *mut nappgui_sys::GuiControl) -> GUID {
//     nappgui_sys::guicontrol_get_tag(control)
// }

// pub(crate) fn global_control_insert<T>(control: T) -> GUID
// where
//     T: Any + 'static,
// {
//     let control = Rc::new(control);
//     let control_id = global_id();
//     GLOBAL_CONTROLS.with_borrow_mut(|controls| controls.insert(control_id, control));
//     control_id
// }

// pub(crate) fn global_control<T, F, R>(uid: GUID, f: F) -> Option<R>
// where
//     T: Any + 'static,
//     F: FnOnce(&T) -> R,
// {
//     let control = GLOBAL_CONTROLS.with_borrow(|controls| controls.get(&uid).map(|x| x.clone()))?;
//     if let Ok(control) = control.downcast::<T>() {
//         Some(f(control.as_ref()))
//     } else {
//         None
//     }
// }

// // macro_rules! define_control {
// //     ($type:ident, $inner_type:ident, $nappgui_type:ident, $props:ident) => {
// //         pub(crate) type $inner_type_inner = ControlInner<nappgui_sys::$nappgui_type, $props>;

// //         /// The button control.
// //         #[repr(transparent)]
// //         #[derive(Clone)]
// //         pub struct $type(GUID);

// //         #[allow(dead_code)]
// //         impl $type {
// //             pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::$type) -> Self {
// //                 let inner = $props::from_raw(ptr);
// //                 let uid = crate::gui::global_control_new(inner);
// //                 unsafe { crate::gui::global_control_set_id(ptr as _, uid) };
// //                 Self(uid)
// //             }

// //             pub(crate) fn inner<F, R>(&self, f: F) -> Option<R>
// //             where
// //                 F: FnOnce(&$props) -> R,
// //             {
// //                 let control = crate::gui::global_control(self.0)?;
// //                 match control.as_ref() {
// //                     crate::gui::Control::$type(inner) => Some(f(inner)),
// //                     _ => None,
// //                 }
// //             }

// //             pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::$type {
// //                 self.inner(|inner| inner.as_ptr()).unwrap()
// //             }
// //         }

// //         impl $props {
// //             pub(crate) fn from_raw(ptr: *mut nappgui_sys::$type) -> Self {
// //                 assert!(!ptr.is_null(), "Null pointer passed to $props::from_raw");
// //                 let mut obj = Default::default();
// //                 obj.ptr = RefCell::new(ptr);
// //                 obj
// //             }

// //             pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::$type {
// //                 *self.ptr.borrow()
// //             }
// //         }
// //     };
// // }
