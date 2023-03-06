mod constant;
mod imp;

use glib::Object;
use gtk::{glib, prelude::ObjectExt, subclass::prelude::ObjectSubclassIsExt, Label};

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

    fn bind_sender_to_label_visibility(&self) {
        let sender_label = &self.imp().sender_label;
        self.bind_property::<Label>(&MessageProperty::Sender, sender_label, "visible")
            .transform_to(|_, sender: String| Some(!sender.is_empty()))
            .sync_create()
            .build();
    }
}
