//! This module defines all [`AddChatPage`] related structures

mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    /// This widget allows the user to add a new chat
    ///
    /// Subclassifies [´gtk::Box´]
    pub struct ChatAdder(ObjectSubclass<imp::ChatAdder>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl ChatAdder {
    /// Creates a new [`AddChatPage`]
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for ChatAdder {
    fn default() -> Self {
        Self::new()
    }
}
