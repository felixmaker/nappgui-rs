use std::{
    cell::RefCell, collections::HashMap, ptr::NonNull, rc::{Rc, Weak}
};

#[derive(Clone)]
pub(crate) enum GlobalObject {
    Object(Rc<Object>),
    WeakObject(Weak<Object>),
}

pub(crate) struct Object {
    pub(crate) pointer: NonNull<()>,
    pub(crate) object_type: ObjectType,
    pub(crate) children: RefCell<Vec<Rc<Object>>>,
}

impl GlobalObject {
    pub(crate) fn upgrade(&self) -> Option<Rc<Object>> {
        match self {
            GlobalObject::Object(object) => Some(object.clone()),
            GlobalObject::WeakObject(weak_object) => weak_object.upgrade(),
        }
    }

    pub(crate) fn downgrade(&self) -> Weak<Object> {
        match self {
            GlobalObject::Object(object) => Rc::downgrade(&object),
            GlobalObject::WeakObject(weak_object) => weak_object.clone(),
        }
    }
}

thread_local! {
    pub(crate) static GLOBAL_OBJECTS: RefCell<HashMap<*mut (), GlobalObject>> = Default::default();
}

pub(crate) fn global_set(pointer: *mut (), object: GlobalObject) {
    GLOBAL_OBJECTS.with_borrow_mut(|objs| objs.insert(pointer, object));
}

pub(crate) fn global_new(pointer: *mut (), object_type: ObjectType) -> Weak<Object> {
    if let Some(object) = global_upgrade(pointer) {
        return Rc::downgrade(&object);
    }
    let object = Object {
        pointer: NonNull::new(pointer).unwrap(),
        object_type,
        children: RefCell::new(Vec::new()),
    };
    let object = GlobalObject::Object(Rc::new(object));
    let weak_object = object.downgrade();
    GLOBAL_OBJECTS.with_borrow_mut(|objs| objs.insert(pointer, object));
    weak_object
}

pub(crate) fn global_upgrade(pointer: *mut ()) -> Option<Rc<Object>> {
    if pointer.is_null() {
        return None;
    }
    let object = GLOBAL_OBJECTS.with_borrow(|objects| objects.get(&pointer).cloned())?;
    object.upgrade()
}

pub(crate) fn global_set_parent(this: *mut (), parent: *mut ()) -> Option<()> {
    let this_object = global_upgrade(this)?;
    let parent_object = global_upgrade(parent)?;
    let this_weak_object = Rc::downgrade(&this_object);
    parent_object.children.borrow_mut().push(this_object);
    global_set(this, GlobalObject::WeakObject(this_weak_object));
    Some(())
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[allow(dead_code)]
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
    Line,
    Window = 64,
    Menu,
    MenuItem,
    Layout,
    Cell,
    Image,
    Other = 1024,
}

impl ObjectType {
    pub(crate) fn is_control(&self) -> bool {
        (*self as u32) < ObjectType::Window as _
    }
}
