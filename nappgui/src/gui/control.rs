// use std::{
//     any::{Any, TypeId},
//     cell::{Cell, RefCell},
//     collections::{HashMap, HashSet},
//     rc::{Rc, Weak},
//     sync::atomic::{AtomicU32, Ordering},
// };

// use nappgui_sys::{guicontrol_get_tag, guicontrol_tag};

// use super::*;

// #[allow(missing_docs)]
// #[derive(Clone)]
// pub enum Control {
//     Button(Button),
//     Combo(Combo),
//     Edit(Edit),
//     ImageView(ImageView),
//     Label(Label),
//     Panel(Panel),
//     ListBox(ListBox),
//     PopUp(PopUp),
//     Progress(Progress),
//     Slider(Slider),
//     SplitView(SplitView),
//     TableView(TableView),
//     TextView(TextView),
//     UpDown(UpDown),
//     View(View),
//     WebView(WebView),
// }

// impl Control {
//     pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::GuiControl {
//         match self {
//             Control::Button(v) => v.as_ptr() as *mut _,
//             Control::Combo(v) => v.as_ptr() as *mut _,
//             Control::Edit(v) => v.as_ptr() as *mut _,
//             Control::ImageView(v) => v.as_ptr() as *mut _,
//             Control::Label(v) => v.as_ptr() as *mut _,
//             Control::Panel(v) => v.as_ptr() as *mut _,
//             Control::ListBox(v) => v.as_ptr() as *mut _,
//             Control::PopUp(v) => v.as_ptr() as *mut _,
//             Control::Progress(v) => v.as_ptr() as *mut _,
//             Control::Slider(v) => v.as_ptr() as *mut _,
//             Control::SplitView(v) => v.as_ptr() as *mut _,
//             Control::TableView(v) => v.as_ptr() as *mut _,
//             Control::TextView(v) => v.as_ptr() as *mut _,
//             Control::UpDown(v) => v.as_ptr() as *mut _,
//             Control::View(v) => v.as_ptr() as *mut _,
//             Control::WebView(v) => v.as_ptr() as *mut _,
//         }
//     }

//     /// Sets a tag for the control.
//     pub(crate) fn set_tag(&self, id: u32) {
//         unsafe { guicontrol_tag(self.as_ptr(), id) };
//     }

//     /// Gets a tag for the control.
//     pub(crate) fn tag(&self) -> u32 {
//         unsafe { guicontrol_get_tag(self.as_ptr()) }
//     }
// }

// #[allow(missing_docs)]
// pub trait AsControl {
//     /// Since widgets in this library are managed via reference counting (`Rc`),
//     /// this method creates a cheap clone of the internal handle without
//     /// duplicating the underlying C resource.
//     fn as_control(&self) -> Control;
// }

// macro_rules! impl_as_control {
//     ($type: ident) => {
//         impl AsControl for $type {
//             fn as_control(&self) -> Control {
//                 Control::$type(self.clone())
//             }
//         }
//     };
// }

// impl_as_control!(Button);
// impl_as_control!(Combo);
// impl_as_control!(Edit);
// impl_as_control!(ImageView);
// impl_as_control!(Label);
// impl_as_control!(Panel);
// impl_as_control!(ListBox);
// impl_as_control!(PopUp);
// impl_as_control!(Progress);
// impl_as_control!(Slider);
// impl_as_control!(SplitView);
// impl_as_control!(TableView);
// impl_as_control!(TextView);
// impl_as_control!(UpDown);
// impl_as_control!(View);
// impl_as_control!(WebView);

// #[allow(missing_docs)]
// #[derive(Clone)]
// pub enum Object {
//     Window(Window),
//     Control(Control),
//     Layout(Layout),
// }

// #[allow(missing_docs)]
// pub trait AsObject {
//     /// Since objects in this library are managed via reference counting (`Rc`),
//     /// this method creates a cheap clone of the internal handle without
//     /// duplicating the underlying C resource.
//     fn as_object(&self) -> Object;

//     fn as_id(&self) -> ObjectID;

//     fn is_visual_owner(&self) -> bool;
// }

// // impl<T> AsObject for T
// // where
// //     T: AsControl,
// // {
// //     fn as_object(&self) -> Object {
// //         Object::Control(self.as_control())
// //     }
// // }

// pub(crate) struct ObjectContext {
//     pub(crate) id: ObjectID,
//     pub(crate) object: Object,
//     pub(crate) is_visual_owener: bool,
//     pub(crate) is_dead: Cell<bool>,
//     pub(crate) owner: RefCell<HashSet<ObjectID>>,
//     pub(crate) childs: RefCell<HashSet<ObjectID>>,
// }

// pub(crate) type ObjectID = u32;
// pub(crate) static UID: AtomicU32 = AtomicU32::new(0);

// thread_local! {
//     pub(crate) static CONTEXT: RefCell<HashMap<u32, ObjectContext>> = Default::default();
// }

// pub(crate) fn global_object(id: ObjectID) -> Option<Object> {
//     CONTEXT.with_borrow(|ctx| ctx.get(&id).map(|x| x.object.clone()))
// }

// // pub(crate) fn global_control<T>(id: ObjectID) -> Option<T>
// // where
// //     T: AsControl,
// // {
// //     let control = global_object(id).and_then(|object| match object {
// //         Object::Control(v) => Some(v),
// //         _ => None,
// //     });
// //     control.map_or(None, |x| T::from_control(&x))
// // }

// // pub(crate) fn global_window(id: ObjectID) -> Option<Window> {
// //     global_object(id).and_then(|object| match object {
// //         Object::Window(v) => Some(v),
// //         _ => None,
// //     })
// // }

// fn global_free_id() -> u32 {
//     UID.fetch_add(1, Ordering::AcqRel)
// }

// pub(crate) fn global_insert_object<T>(id: ObjectID, object: T)
// where
//     T: AsObject,
// {
//     let object = ObjectContext {
//         id,
//         object: object.as_object(),
//         is_visual_owener: object.is_visual_owner(),
//         is_dead: Default::default(),
//         owner: Default::default(),
//         childs: Default::default(),
//     };
//     CONTEXT.with_borrow_mut(|ctx| ctx.insert(id, object));
// }

// pub(crate) fn global_is_alive(id: ObjectID) -> bool {
//     CONTEXT
//         .with_borrow(|ctx| ctx.get(&id).map(|entry| !entry.is_dead.get()))
//         .unwrap_or(false)
// }

// pub(crate) fn global_set_owner(id: ObjectID, owner_id: ObjectID) {
//     CONTEXT.with_borrow_mut(|ctx| {
//         if let Some(child) = ctx.get(&id) {
//             child.owner.borrow_mut().insert(owner_id);
//         }
//         if let Some(owner) = ctx.get(&owner_id) {
//             owner.childs.borrow_mut().insert(id);
//         }
//     });
// }

// pub(crate) fn global_kill(id: ObjectID) {
//     // 1. A queue to hold the IDs we need to process
//     let mut queue = vec![id];

//     // 2. Process the queue until it's empty
//     while let Some(current_id) = queue.pop() {
//         // 3. Open the map, mark the item dead, and grab its children
//         let children = CONTEXT.with_borrow(|ctx| {
//             if let Some(entry) = ctx.get(&current_id) {
//                 // Mark this specific object as dead
//                 entry.is_dead.set(true);

//                 // Clone the children IDs so we can process them outside this borrow
//                 // Using .borrow() because childs is a RefCell
//                 return Some(entry.childs.clone());
//             }
//             None
//         });

//         // 4. Add the children to the queue to be processed in the next iterations
//         if let Some(child_ids) = children {
//             for child_id in child_ids.borrow().iter() {
//                 let Some(is_visual_owner) = CONTEXT.with_borrow(|ctx| ctx.get(child_id).map(|x| x.is_visual_owener))
//                 else {
//                     continue;
//                 };
//                 if is_visual_owner {
//                     continue;
//                 }
//                 queue.push(*child_id);
//             }
//         }
//     }
// }

// pub(crate) fn global_with<R, F>(id: ObjectID, f: F) -> Option<R>
// where
//     F: FnOnce() -> R,
// {
//     if global_is_alive(id) {
//         return Some(f());
//     }
//     debug_assert!(
//         false,
//         "NAppGUI Warning: Attempted to use controls after windows destroyed.",
//     );
//     return None;
// }

// pub(crate) fn global_with2<R, F>(id: &[ObjectID], f: F) -> Option<R>
// where
//     F: FnOnce() -> R,
// {
//     if id.iter().all(|x| global_is_alive(*x)) {
//         return Some(f());
//     }
//     debug_assert!(
//         false,
//         "NAppGUI Warning: Attempted to use controls after windows destroyed.",
//     );
//     return None;
// }

// pub(crate) fn global_expect<R, F>(id: ObjectID, f: F) -> R
// where
//     F: FnOnce() -> R,
// {
//     global_with(id, f).expect("NAppGUI panic: Attempted to get propety of object that is no longer alive.")
// }

// pub(crate) fn global_expect2<R, F>(id: &[ObjectID], f: F) -> R
// where
//     F: FnOnce() -> R,
// {
//     global_with2(id, f).expect("NAppGUI panic: Attempted to get propety of object that is no longer alive.")
// }

// // pub(crate) fn global_with_id<T, R, F>(id: ObjectID, f: F) -> Option<R>
// // where
// //     T: AsControl,
// //     F: FnOnce(&T) -> R,
// // {
// //     if global_is_alive(id) {
// //         if let Some(control) = global_control::<T>(id) {
// //             return Some(f(&control));
// //         }
// //     }
// //     debug_assert!(
// //         false,
// //         "NAppGUI Warning: Attempted to use a {} that is no longer alive.",
// //         std::any::type_name::<T>()
// //     );
// //     return None;
// // }

// // pub(crate) fn global_with_id2<T1, T2, R, F>(id1: ObjectID, id2: ObjectID, f: F) -> Option<R>
// // where
// //     T1: AsControl,
// //     T2: AsControl,
// //     F: FnOnce(&T1, &T2) -> R,
// // {
// //     if global_is_alive(id1) && global_is_alive(id2) {
// //         match (global_control::<T1>(id1), global_control::<T2>(id2)) {
// //             (Some(t1), Some(t2)) => return Some(f(&t1, &t2)),
// //             _ => return None,
// //         }
// //     }
// //     debug_assert!(
// //         false,
// //         "NAppGUI Warning: Attempted to use a {} or {} that is no longer alive.",
// //         std::any::type_name::<T1>(),
// //         std::any::type_name::<T2>()
// //     );
// //     return None;
// // }

// // pub(crate) fn global_try_with<T, R, F>(control: &T, f: F) -> Option<R>
// // where
// //     T: AsControl,
// //     F: FnOnce(&T) -> R,
// // {
// //     let id = control.as_control().tag();
// //     global_with_id(id, f)
// // }

// // pub(crate) fn global_with<T, R, F>(control: &T, f: F) -> R
// // where
// //     T: AsControl,
// //     F: FnOnce(&T) -> R,
// // {
// //     global_try_with(control, f).expect(&format!(
// //         "NAppGUI Panic: Attempted to use a {} that is no longer alive.",
// //         std::any::type_name::<R>()
// //     ))
// // }

// // pub(crate) fn global_try_with2<T1, T2, R, F>(control1: &T1, control2: &T2, f: F) -> Option<R>
// // where
// //     T1: AsControl,
// //     T2: AsControl,
// //     F: FnOnce(&T1, &T2) -> R,
// // {
// //     let id1 = control1.as_control().tag();
// //     let id2 = control2.as_control().tag();
// //     global_with_id2(id1, id2, f)
// // }

// // pub(crate) fn global_with2<T1, T2, R, F>(control1: &T1, control2: &T2, f: F) -> R
// // where
// //     T1: AsControl,
// //     T2: AsControl,
// //     F: FnOnce(&T1, &T2) -> R,
// // {
// //     global_try_with2(control1, control2, f).expect(&format!(
// //         "NAppGUI Panic: Attempted to use a {} that is no longer alive.",
// //         std::any::type_name::<R>()
// //     ))
// // }

// pub(crate) fn global_create<F, R>(f: F) -> R
// where
//     F: FnOnce(ObjectID) -> R,
//     R: AsObject + Clone,
// {
//     let id = global_free_id();
//     let object = f(id);
//     global_insert_object(id, object.clone());
//     object
// }

// // thread_local! {
// //     static OBJ: RefCell<HashMap<ObjectID, Weak<dyn AsObjectID>>> = Default::default();
// // }

// struct ObjectInner<T> {
//     pointer: *mut T,
//     id: ObjectID,
//     object_type: ObjectType,
//     owner: RefCell<Option<Weak<dyn AsObjectID>>>, // An Object may has one owner that control its release.
//     related: RefCell<Vec<Weak<dyn AsObjectID>>>,  // They are controled by one owner.
// }

// trait AsObjectID {
//     fn as_id(&self) -> ObjectID;
//     fn owner_as_weak(&self) -> Option<Weak<dyn AsObjectID>>;
//     fn related_as_weak(&self) -> Vec<Weak<dyn AsObjectID>>;
//     fn set_owner(&self, owner: Weak<dyn AsObjectID>);
//     fn set_owner_for_related(&self, owner: Weak<dyn AsObjectID>);
// }

// impl<T> AsObjectID for Object2<T> {
//     fn as_id(&self) -> ObjectID {
//         self.0.id
//     }

//     fn owner_as_weak(&self) -> Option<Weak<dyn AsObjectID>> {
//         self.0.owner.borrow().as_ref().cloned()
//     }

//     fn related_as_weak(&self) -> Vec<Weak<dyn AsObjectID>> {
//         self.0.related.borrow().iter().cloned().collect()
//     }

//     fn set_owner(&self, owner: Weak<dyn AsObjectID>) {
//         *self.0.owner.borrow_mut() = Some(owner);
//     }

//     fn set_owner_for_related(&self, owner: Weak<dyn AsObjectID>) {
//         // We use the ID to prevent infinite loops if there are cycles
//         let mut queue: Vec<Weak<dyn AsObjectID>> = vec![];

//         // Initial update for self
//         self.set_owner(owner.clone());
//         for related in self.related_as_weak() {
//             queue.push(related);
//         }

//         while let Some(related) = queue.pop() {
//             if let Some(related) = related.upgrade() {
//                 // Compare IDs or pointer addresses instead of just checking "is_some"
//                 let owner_of_related = related.owner_as_weak();

//                 // Only update if the window is actually different
//                 // This handles cycles AND allows for re-parenting/updating
//                 let needs_update = match (&owner_of_related, &owner) {
//                     (Some(cur), new) => !std::ptr::addr_eq(cur.as_ptr(), new.as_ptr()),
//                     (None, _) => true,
//                 };

//                 if needs_update {
//                     related.set_owner(owner.clone());
//                     for r in related.related_as_weak() {
//                         queue.push(r);
//                     }
//                 }
//             }
//         }
//     }
// }

// struct Object2<T>(Rc<ObjectInner<T>>);

// impl<T> Object2<T> {
//     fn from_raw(pointer: *mut T, object_type: ObjectType) -> Self {
//         assert!(!pointer.is_null());
//         let id = global_free_id();
//         let object = ObjectInner {
//             pointer,
//             id,
//             object_type,
//             owner: Default::default(),
//             related: Default::default(),
//         };
//         Self(object.into())
//     }
// }
// //     pub fn add_child<U>(&self, child: &Object2<U>)
// //     where
// //         ObjectInner<U>: AsObjectID + 'static,
// //     {
// //         // 1. Get the Rc from the child
// //         let child_rc: Rc<ObjectInner<U>> = child.0.clone();

// //         // 2. Coerce Rc<ObjectInner<U>> to Rc<dyn AsObjectID>
// //         let trait_rc: Rc<dyn AsObjectID> = child_rc;

// //         // 3. Downgrade to Weak<dyn AsObjectID> and insert
// //         self.0.related.borrow_mut().push(Rc::downgrade(&trait_rc));
// //     }
// // }

// // use std::hash::{Hash, Hasher};

// // #[derive(Clone)]
// // struct WeakObject(Weak<dyn AsObjectID>);

// // impl PartialEq for WeakObject {
// //     fn eq(&self, other: &Self) -> bool {
// //         // Compare the raw data pointers
// //         self.0.as_ptr() == other.0.as_ptr()
// //     }
// // }

// // impl Eq for WeakObject {}

// // impl Hash for WeakObject {
// //     fn hash<H: Hasher>(&self, state: &mut H) {
// //         // Hash the memory address of the data
// //         self.0.as_ptr().hash(state);
// //     }
// // }

// #[repr(u32)]
// #[derive(PartialEq, Eq, Clone, Copy, Debug)]
// pub enum ObjectType {
//     Button = 0,
//     Combo,
//     Edit,
//     ImageView,
//     Label,
//     Panel,
//     ListBox,
//     PopUp,
//     Progress,
//     Slider,
//     SplitView,
//     TableView,
//     TextView,
//     UpDown,
//     View,
//     WebView,
//     Window = 64,
//     Layout,
//     Image,
//     Other = 1024,
// }

// impl ObjectType {
//     pub fn is_control(&self) -> bool {
//         (*self as u32) < ObjectType::Window as _
//     }

//     pub fn from<T>(object: *mut T) -> Self
//     where
//         T: Any + 'static,
//     {
//         let id = object.type_id();

//         if id == TypeId::of::<*mut nappgui_sys::Button>() {
//             Self::Button
//         } else if id == TypeId::of::<*mut nappgui_sys::Label>() {
//             Self::Label
//         } else if id == TypeId::of::<*mut nappgui_sys::Edit>() {
//             Self::Edit
//         } else if id == TypeId::of::<*mut nappgui_sys::Panel>() {
//             Self::Panel
//         } else if id == TypeId::of::<*mut nappgui_sys::View>() {
//             Self::View
//         } else if id == TypeId::of::<*mut nappgui_sys::Window>() {
//             Self::Window
//         } else if id == TypeId::of::<*mut nappgui_sys::Layout>() {
//             Self::Layout
//         } else if id == TypeId::of::<*mut nappgui_sys::TextView>() {
//             Self::TextView
//         } else if id == TypeId::of::<*mut nappgui_sys::ImageView>() {
//             Self::ImageView
//         } else if id == TypeId::of::<*mut nappgui_sys::Slider>() {
//             Self::Slider
//         } else if id == TypeId::of::<*mut nappgui_sys::UpDown>() {
//             Self::UpDown
//         } else if id == TypeId::of::<*mut nappgui_sys::Progress>() {
//             Self::Progress
//         } else if id == TypeId::of::<*mut nappgui_sys::PopUp>() {
//             Self::PopUp
//         } else if id == TypeId::of::<*mut nappgui_sys::ListBox>() {
//             Self::ListBox
//         } else if id == TypeId::of::<*mut nappgui_sys::TableView>() {
//             Self::TableView
//         } else if id == TypeId::of::<*mut nappgui_sys::SplitView>() {
//             Self::SplitView
//         } else if id == TypeId::of::<*mut nappgui_sys::WebView>() {
//             Self::WebView
//         } else if id == TypeId::of::<*mut nappgui_sys::Image>() {
//             Self::Image
//         } else {
//             Self::Other
//         }
//     }
// }
