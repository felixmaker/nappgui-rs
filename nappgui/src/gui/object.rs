use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ptr::NonNull,
    rc::{Rc, Weak},
};

pub(crate) struct Object {
    pub(crate) pointer: NonNull<()>,
    pub(crate) object_type: ObjectType,
    pub(crate) need_destroy: Cell<bool>,
}

thread_local! {
    pub(crate) static GLOBAL_OBJECTS: RefCell<HashMap<*mut (), Rc<Object>>> = Default::default();
}

pub(crate) fn global_get(pointer: *mut ()) -> Option<Rc<Object>> {
    if pointer.is_null() {
        return None;
    }
    GLOBAL_OBJECTS.with_borrow(|objects| objects.get(&pointer).cloned())
}

pub(crate) fn global_set(pointer: *mut (), object: Rc<Object>) {
    GLOBAL_OBJECTS.with_borrow_mut(|objs| objs.insert(pointer, object));
}

pub(crate) fn global_new(pointer: *mut (), object_type: ObjectType) -> Weak<Object> {
    if let Some(object) = global_get(pointer) {
        return Rc::downgrade(&object);
    }
    let object = Rc::new(Object {
        pointer: NonNull::new(pointer).unwrap(),
        object_type,
        need_destroy: Cell::new(true),
    });
    let weak_object = Rc::downgrade(&object);
    global_set(pointer, object);
    weak_object
}

pub(crate) fn global_set_need_destroy(this: *mut (), need_destroy: bool) -> Option<()> {
    let this_object = global_get(this)?;
    this_object.need_destroy.set(need_destroy);
    Some(())
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum ObjectType {
    Window = 64,
    Menu,
}
