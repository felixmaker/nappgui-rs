use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
    sync::atomic::{AtomicU32, Ordering},
};

pub(crate) type ObjectID = u32;
pub(crate) static UID: AtomicU32 = AtomicU32::new(0);

thread_local! {
    pub(crate) static GLOBAL_OBJECTS: RefCell<HashMap<ObjectID, Object>> = Default::default();
    pub(crate) static GLOBAL_POINTERS: RefCell<HashMap<*mut (), ObjectID>> = Default::default();
}

fn global_free_id() -> u32 {
    UID.fetch_add(1, Ordering::AcqRel)
}

pub(crate) struct ObjectInner {
    pub(crate) pointer: *mut (),
    pub(crate) id: ObjectID,
    pub(crate) object_type: ObjectType,
    pub(crate) owner: RefCell<Option<Weak<dyn AsObject>>>, // An Object may has one owner that control its release.
    pub(crate) related: RefCell<Vec<Weak<dyn AsObject>>>,  // They are controled by one owner.
}

pub(crate) trait AsObject {
    fn as_ptr(&self) -> *mut ();
    fn as_id(&self) -> ObjectID;
    fn owner_as_weak(&self) -> Option<Weak<dyn AsObject>>;
    fn related_as_weak(&self) -> Vec<Weak<dyn AsObject>>;
    fn set_owner(&self, owner: Weak<dyn AsObject>);
    fn set_owner_for_related(&self, owner: Weak<dyn AsObject>);
}

impl AsObject for Object {
    fn as_ptr(&self) -> *mut () {
        self.0.pointer
    }

    fn as_id(&self) -> ObjectID {
        self.0.id
    }

    fn owner_as_weak(&self) -> Option<Weak<dyn AsObject>> {
        self.0.owner.borrow().as_ref().cloned()
    }

    fn related_as_weak(&self) -> Vec<Weak<dyn AsObject>> {
        self.0.related.borrow().iter().cloned().collect()
    }

    fn set_owner(&self, owner: Weak<dyn AsObject>) {
        *self.0.owner.borrow_mut() = Some(owner);
    }

    fn set_owner_for_related(&self, owner: Weak<dyn AsObject>) {
        // We use the ID to prevent infinite loops if there are cycles
        let mut queue: Vec<Weak<dyn AsObject>> = vec![];

        // Initial update for self
        self.set_owner(owner.clone());
        for related in self.related_as_weak() {
            queue.push(related);
        }

        while let Some(related) = queue.pop() {
            if let Some(related) = related.upgrade() {
                // Compare IDs or pointer addresses instead of just checking "is_some"
                let owner_of_related = related.owner_as_weak();

                // Only update if the window is actually different
                // This handles cycles AND allows for re-parenting/updating
                let needs_update = match (&owner_of_related, &owner) {
                    (Some(cur), new) => !std::ptr::addr_eq(cur.as_ptr(), new.as_ptr()),
                    (None, _) => true,
                };

                if needs_update {
                    related.set_owner(owner.clone());
                    for r in related.related_as_weak() {
                        queue.push(r);
                    }
                }
            }
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct WeakObject(Weak<ObjectInner>);

impl WeakObject {
    pub fn as_mut_ptr<T>(&self) -> Option<*mut T>
    where
        T: 'static,
    {
        if let Some(obj) = self.0.upgrade() {
            let pointer = obj.pointer as *mut T;
            if ObjectType::from_ptr(pointer) == obj.object_type {
                return Some(pointer);
            }
        }
        return None;
    }

    pub fn as_mut_ptr_or_panic<T>(&self) -> *mut T
    where
        T: 'static,
    {
        self.as_mut_ptr::<T>()
            .expect("NAppGUI Panic: Object no longer able to access!")
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct Object(pub(crate) Rc<ObjectInner>);

impl Object {
    /// Create a object on global static area and return a weak object to it.
    pub fn new<T>(pointer: *mut T, object_type: ObjectType) -> WeakObject {
        assert!(!pointer.is_null());
        let id = global_free_id();
        let pointer = pointer as *mut ();
        let object = ObjectInner {
            pointer,
            id,
            object_type,
            owner: Default::default(),
            related: Default::default(),
        };
        let object = Object(object.into());
        let weak_object = Rc::downgrade(&object.0);
        GLOBAL_OBJECTS.with_borrow_mut(|objs| objs.insert(id, object));
        GLOBAL_POINTERS.with_borrow_mut(|pointers| pointers.insert(pointer, id));
        WeakObject(weak_object)
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
    Image,
    Other = 1024,
}

impl ObjectType {
    pub fn is_control(&self) -> bool {
        (*self as u32) < ObjectType::Window as _
    }

    pub fn from_ptr<T>(object: *mut T) -> Self
    where
        T: Any + 'static,
    {
        let id = object.type_id();

        if id == TypeId::of::<*mut nappgui_sys::Button>() {
            Self::Button
        } else if id == TypeId::of::<*mut nappgui_sys::Combo>() {
            Self::Combo
        } else if id == TypeId::of::<*mut nappgui_sys::Label>() {
            Self::Label
        } else if id == TypeId::of::<*mut nappgui_sys::Edit>() {
            Self::Edit
        } else if id == TypeId::of::<*mut nappgui_sys::Panel>() {
            Self::Panel
        } else if id == TypeId::of::<*mut nappgui_sys::View>() {
            Self::View
        } else if id == TypeId::of::<*mut nappgui_sys::Window>() {
            Self::Window
        } else if id == TypeId::of::<*mut nappgui_sys::Layout>() {
            Self::Layout
        } else if id == TypeId::of::<*mut nappgui_sys::TextView>() {
            Self::TextView
        } else if id == TypeId::of::<*mut nappgui_sys::ImageView>() {
            Self::ImageView
        } else if id == TypeId::of::<*mut nappgui_sys::Slider>() {
            Self::Slider
        } else if id == TypeId::of::<*mut nappgui_sys::UpDown>() {
            Self::UpDown
        } else if id == TypeId::of::<*mut nappgui_sys::Progress>() {
            Self::Progress
        } else if id == TypeId::of::<*mut nappgui_sys::PopUp>() {
            Self::PopUp
        } else if id == TypeId::of::<*mut nappgui_sys::ListBox>() {
            Self::ListBox
        } else if id == TypeId::of::<*mut nappgui_sys::TableView>() {
            Self::TableView
        } else if id == TypeId::of::<*mut nappgui_sys::SplitView>() {
            Self::SplitView
        } else if id == TypeId::of::<*mut nappgui_sys::WebView>() {
            Self::WebView
        } else if id == TypeId::of::<*mut nappgui_sys::Image>() {
            Self::Image
        } else {
            Self::Other
        }
    }
}
