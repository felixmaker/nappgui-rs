pub(crate) mod button;
pub(crate) mod cell;
pub(crate) mod combo;
pub(crate) mod control;
pub(crate) mod edit;
pub(crate) mod imageview;
pub(crate) mod label;
pub(crate) mod layout;
pub(crate) mod listbox;
pub(crate) mod menu;
pub(crate) mod menuitem;
pub(crate) mod panel;
pub(crate) mod popup;
pub(crate) mod progress;
pub(crate) mod slider;
pub(crate) mod splitview;
pub(crate) mod tableview;
pub(crate) mod textview;
pub(crate) mod updown;
pub(crate) mod view;
pub(crate) mod window;
pub(crate) mod webview;

/// Common dialogs are default windows provided by the operating system to perform daily tasks such 
/// as: Open files, select colors, fonts, etc.
pub mod dialog;
/// Events are messages that are generated by the user interface and that can be handled by the
pub mod event;

use crate::util::macros::impl_clone;

pub use {
    button::Button, cell::Cell, combo::Combo, control::Control, edit::Edit, imageview::ImageView,
    label::Label, layout::Layout, listbox::ListBox, menu::Menu, menuitem::MenuItem, panel::Panel,
    popup::PopUp, progress::Progress, slider::Slider, splitview::SplitView, tableview::TableView,
    textview::TextView, updown::UpDown, view::View, webview::WebView, window::Window,
};

impl_clone!(Button);
impl_clone!(Cell);
impl_clone!(Combo);
impl_clone!(Control);
impl_clone!(Edit);
impl_clone!(ImageView);
impl_clone!(Label);
impl_clone!(Layout);
impl_clone!(ListBox);
impl_clone!(Menu);
impl_clone!(MenuItem);
impl_clone!(Panel);
impl_clone!(PopUp);
impl_clone!(Progress);
impl_clone!(Slider);
impl_clone!(SplitView);
impl_clone!(TableView);
impl_clone!(TextView);
impl_clone!(UpDown);
impl_clone!(View);
impl_clone!(WebView);
impl_clone!(Window);
