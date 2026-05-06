use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::gui::*;

#[derive(Default)]
pub(crate) struct GlobalObject {
    /// Always points to the object itself if its wrapped with reference count.
    weak_object: Option<Weak<dyn Any + 'static>>,
    /// The object itself.
    _object: Option<Rc<dyn Any + 'static>>,
    /// Objects that this object owns.
    _object_owns: Vec<Rc<dyn Any + 'static>>,
}

pub(crate) enum GObject {
    Button(ButtonInner),
    Combo(ComboInner),
    Edit(EditInner),
    ImageView(ImageViewInner),
    Label(LabelInner),
    Panel(PanelInner),
    ListBox(ListBoxInner),
    PopUp(PopUpInner),
    Progress(ProgressInner),
    Slider(SliderInner),
    SplitView(SplitViewInner),
    TableView(TableViewInner),
    TextView(TextViewInner),
    UpDown(UpDownInner),
    View(ViewInner),
    WebView(WebViewInner),
    Line(LineInner),
    Window(WindowInner),
    Menu(MenuInner),
    MenuItem(MenuItemInner),
}

pub(crate) type GUID = u32;

thread_local! {
    static GLOBAL_UID: Cell<GUID> = Cell::new(0);
    pub(crate) static GLOBAL_OBJECTS: RefCell<HashMap<GUID, Rc<GObject>>> = Default::default();
}

pub(crate) fn global_guid() -> GUID {
    GLOBAL_UID.with(|uid| {
        let id = uid.get() + 1;
        uid.set(id);
        id
    })
}

pub(crate) fn global_get(uid: GUID) -> Option<Rc<GObject>> {
    GLOBAL_OBJECTS.with_borrow(|objects| objects.get(&uid).map(|x| x.clone()))
}

/// Record the object to the global object.
pub(crate) fn global_record(uid: GUID, object: GObject) -> Rc<GObject> {
    let object = Rc::new(object);
    GLOBAL_OBJECTS.with_borrow_mut(|objects| objects.insert(uid, object.clone()));
    object
}

// /// Move the ownership of the object from one pointer to another.
// ///
// /// # Remarks
// /// The object that is moved to the `to` pointer will be created if it does not exist.
// pub(crate) fn global_move_ownership(from: *mut (), to: *mut ()) {
//     GLOBAL_OBJECTS.with_borrow_mut(|objects| -> Option<()> {
//         let from_objects: Vec<Rc<dyn Any + 'static>> = {
//             let mut result = Vec::new();
//             let from_object = objects.get_mut(&from)?;
//             if let Some(obj) = from_object.object.take() {
//                 result.push(obj);
//             } else if let Some(obj) = from_object.weak_object.as_ref() {
//                 if let Some(obj) = obj.upgrade() {
//                     result.push(obj);
//                 }
//             }
//             while let Some(obj) = from_object.object_owns.pop() {
//                 result.push(obj);
//             }
//             result
//         };
//         if !objects.contains_key(&to) {
//             objects.insert(to, GlobalObject::default());
//         }
//         let to_object = objects.get_mut(&to)?;
//         to_object.object_owns.extend(from_objects);
//         Some(())
//     });
// }

macro_rules! impl_object {
    ($type:ident, $type_ex:ident) => {
        impl $type {
            pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::$type) -> Self {
                let uid = crate::gui::global_guid();
                crate::gui::global_record(uid, crate::gui::GObject::$type($type_ex::from_raw(ptr)));
                Self(uid)
            }

            pub(crate) fn from_ptr(ptr: *mut nappgui_sys::$type) -> Option<Self> {
                let uid = crate::gui::control_uid(ptr as _)?;
                Some(Self(uid))
            }

            pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::$type {
                match crate::gui::global_get(self.0).unwrap().as_ref() {
                    crate::gui::GObject::$type(obj) => obj.as_ptr(),
                    _ => panic!("$type object not found."),
                }
            }

            pub(crate) fn inner<F, R>(&self, f: F) -> Option<R>
            where
                F: FnOnce(&$type_ex) -> R,
            {
                crate::gui::global_get(self.0).and_then(|x| match x.as_ref() {
                    crate::gui::GObject::$type(object) => Some(f(object)),
                    _ => None,
                })
            }
        }

        impl $type_ex {
            pub(crate) fn from_raw(ptr: *mut nappgui_sys::$type) -> Self {
                assert!(!ptr.is_null(), "Null pointer passed to $type_ex::from_raw");
                let object: Self = Default::default();
                *object.ptr.borrow_mut() = ptr;
                object
            }

            pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::$type {
                *self.ptr.borrow()
            }
        }
    };
}

pub(crate) use impl_object;
