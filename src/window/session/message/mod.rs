mod constant;
mod imp;

use glib::Object;
use gtk::{glib, prelude::ObjectExt};

pub use constant::MessageProperty;

glib::wrapper! {
    pub struct Message(ObjectSubclass<imp::Message>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Message {
    pub fn new(message: String) -> Self {
        Object::builder()
            .property(&MessageProperty::Message, message)
            .build()
    }

    pub fn set_sender(&self, sender: String) {
        self.set_property(&MessageProperty::Sender, sender);
    }
}
