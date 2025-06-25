pub(crate) mod event;
pub(crate) mod regex;
pub(crate) mod stream;
pub(crate) mod string;

/// In high-level languages, such as .NET or Javascript, data binding is a technique that allows establishing an automatic 
/// connection between the data of an application and its user interface elements. The NAppGUI DBind module implements and 
/// extends this concept in C language, since it makes it possible to automate certain tasks on the structures and objects 
/// of our application (Figure 1). Thanks to this we will avoid generating redundant code that is problematic to maintain, 
/// providing a general interface for: 
///  - Creation, destruction and copying of objects.
///  - Comparison of objects.
///  - Serialization: Reading and writing in streams.
///  - Import/export in different formats, such as JSON.
///  - Synchronization with user interfaces.
pub mod dbind;

pub use {
    event::{Event, NappGUIEventParams, NappGUIEventResult},
    regex::RegEx,
    stream::Stream,
    string::NappguiString,
};
