use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
    sync::atomic::{AtomicU32, Ordering},
};

pub(crate) type ObjectID = u32;
pub(crate) static UID: AtomicU32 = AtomicU32::new(0);

thread_local! {
    pub(crate) static GLOBAL_OBJECTS: RefCell<HashMap<ObjectID, Rc<dyn AsObject >>> = Default::default();
    pub(crate) static GLOBAL_POINTERS: RefCell<HashMap<*mut (), ObjectID>> = Default::default();
}

fn global_free_id() -> u32 {
    UID.fetch_add(1, Ordering::AcqRel)
}

// #[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
// pub(crate) struct Pointer {
//     object_type: ObjectType,
//     pointer: *mut (),
// }

pub(crate) fn global_id(pointer: *mut ()) -> Option<ObjectID> {
    GLOBAL_POINTERS.with_borrow(|pointers| pointers.get(&(pointer as _)).cloned())
}

pub(crate) fn global_object<T>(pointer: *mut T) -> Option<WeakObject<T>>
where
    T: 'static,
{
    if pointer.is_null() {
        return None;
    }
    let id = global_id(pointer as _)?;
    let object = GLOBAL_OBJECTS.with_borrow(|objects| objects.get(&id).cloned())?;
    let object_t = object.as_any().downcast_ref::<Rc<ObjectInner<T>>>()?;
    Some(WeakObject(Rc::downgrade(object_t)))
}

pub(crate) struct ObjectInner<T> {
    pub(crate) pointer: *mut T,
    pub(crate) id: ObjectID,
    pub(crate) object_type: ObjectType,
}

pub(crate) trait AsObject {
    fn as_ptr(&self) -> *mut ();
    fn as_id(&self) -> ObjectID;
    fn as_object_type(&self) -> ObjectType;
    fn as_any(&self) -> &dyn Any;
}

impl<T> AsObject for ObjectInner<T>
where
    T: 'static,
{
    fn as_ptr(&self) -> *mut () {
        self.pointer as _
    }

    fn as_id(&self) -> ObjectID {
        self.id
    }

    fn as_object_type(&self) -> ObjectType {
        self.object_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct WeakObject<T>(pub(crate) Weak<ObjectInner<T>>);

impl<T> WeakObject<T> {
    pub(crate) fn upgrade(&self) -> Option<Object<T>> {
        let object = self.0.upgrade();
        object.and_then(|x| Some(Object(x)))
    }

    pub(crate) fn as_ptr(&self) -> Option<*mut T> {
        self.upgrade().map(|x| x.0.pointer)
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct Object<T>(pub(crate) Rc<ObjectInner<T>>);

impl<T> Object<T>
where
    T: 'static,
{
    /// Create a object on global static area and return a weak object to it.
    pub fn global_new(pointer: *mut T, object_type: ObjectType) -> WeakObject<T> {
        assert!(!pointer.is_null());
        let object = Object::new(pointer, object_type);
        let id = object.0.id;
        let weak_object = Rc::downgrade(&object.0);
        GLOBAL_OBJECTS.with_borrow_mut(|objs| objs.insert(id, object.0));
        GLOBAL_POINTERS.with_borrow_mut(|pointers| pointers.insert(pointer as _, id));
        WeakObject(weak_object)
    }

    /// Create a object.
    pub fn new(pointer: *mut T, object_type: ObjectType) -> Object<T> {
        assert!(!pointer.is_null());
        let id = global_free_id();
        let object = ObjectInner {
            pointer,
            id,
            object_type,
        };
        Object(object.into())
    }
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum ObjectType {
    Button = 0,
    Combo,
    Edit,
    ImageView,
    Label,
    Panel,
    ListBox,
    PopUp,
    Progress,
    Slider,
    SplitView,
    TableView,
    TextView,
    UpDown,
    View,
    WebView,
    Window = 64,
    Layout,
    Cell,
    Image,
    Other = 1024,
}

impl ObjectType {
    pub fn is_control(&self) -> bool {
        (*self as u32) < ObjectType::Window as _
    }
}
