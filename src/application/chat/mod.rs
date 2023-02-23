use glib::Object;
use gtk::{glib, subclass::prelude::ObjectSubclassIsExt, Align, Label};

mod imp;

glib::wrapper! {
    pub struct Chat(ObjectSubclass<imp::Chat>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

}

impl Chat {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn add_external_message(&self, message: String) {
        let label = Label::builder().label(message).halign(Align::Start).build();
        self.imp().message_list.append(&label);
    }

    fn add_own_message(&self, message: String) {
        let label = Label::builder().label(message).halign(Align::End).build();
        self.imp().message_list.append(&label);
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}
