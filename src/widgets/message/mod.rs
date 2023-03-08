/// This module defines [`Message`] related structures
mod constant;
mod imp;

use glib::Object;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, Label};

pub use constant::MessageProperty;

glib::wrapper! {
    /// Used to display a message inside a chat
    ///
    /// May have a sender, if needed
    ///
    /// Has a single css node 'message'
    ///
    /// Subclassifies [`gtk::Box`]
    pub struct Message(ObjectSubclass<imp::Message>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Message {
    /// Creates a new [´Message´] with the specified text
    pub fn new(message: String) -> Self {
        Object::builder()
            .property(&MessageProperty::Message, message)
            .build()
    }

    /// Sets the sender of the message to be displayed
    pub fn set_sender(&self, sender: String) {
        self.set_property(&MessageProperty::Sender, sender);
    }

    /// Binds the sender's emptiness to the label's visibiliy
    fn bind_sender_to_label_visibility(&self) {
        let sender_label = &self.imp().sender_label;
        self.bind_property::<Label>(&MessageProperty::Sender, sender_label, "visible")
            .transform_to(|_, sender: String| Some(!sender.is_empty()))
            .sync_create()
            .build();
    }
}
