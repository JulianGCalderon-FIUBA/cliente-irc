//! This module defines all [`AddChatPage`] related structures

mod constant;
mod imp;

pub use constant::AddChatPageSignal;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    /// This widget allows the user to add a new chat
    ///
    /// Subclassifies [´gtk::Box´]
    pub struct AddChatPage(ObjectSubclass<imp::AddChatPage>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl AddChatPage {
    /// Creates a new [`AddChatPage`]
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for AddChatPage {
    fn default() -> Self {
        Self::new()
    }
}
