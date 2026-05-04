use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Default)]
pub(crate) struct GlobalObject {
    /// Always points to the object itself if its wrapped with reference count.
    weak_object: Option<Weak<dyn Any + 'static>>,
    /// The object itself.
    object: Option<Rc<dyn Any + 'static>>,
    /// Objects that this object owns.
    object_owns: Vec<Rc<dyn Any + 'static>>,
}

thread_local! {
    pub(crate) static GLOBAL_OBJECTS: RefCell<HashMap<*mut (), GlobalObject>> = Default::default();
}

pub(crate) fn global_exists(pointer: *mut ()) -> bool {
    GLOBAL_OBJECTS.with_borrow(|objects| objects.get(&pointer).is_some())
}

pub(crate) fn global_get<T>(pointer: *mut ()) -> Option<Rc<T>>
where
    T: Any + 'static,
{
    if pointer.is_null() {
        return None;
    }
    let this = GLOBAL_OBJECTS.with_borrow(|objects| objects.get(&pointer).and_then(|x| x.weak_object.clone()))?;
    let object = this.upgrade()?;
    object.downcast::<T>().ok()
}

/// Record the object to the global object.
pub(crate) fn global_record<T>(pointer: *mut (), object: T) -> Rc<T>
where
    T: Any + 'static,
{
    assert!(!pointer.is_null());
    let object = Rc::new(object);
    let weak_object = Rc::downgrade(&object);
    let global_object = GlobalObject {
        object: Some(object.clone()),
        weak_object: Some(weak_object.clone()),
        ..Default::default()
    };
    GLOBAL_OBJECTS.with_borrow_mut(|objects| objects.insert(pointer, global_object));
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
