use glib::Object;
use gtk::{
    glib, prelude::ObjectExt, subclass::prelude::ObjectSubclassIsExt, Align, Builder, Label,
};

mod chat_header;
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

    pub fn add_external_message(&self, content: String) {
        let message: Label =
            Builder::from_resource("/com/jgcalderon/irc-client/chat-external-message.ui")
                .object("message")
                .unwrap();

        message.set_property("label", &content);

        self.imp().message_list.append(&message);
    }

    fn add_own_message(&self, content: String) {
        let message: Label =
            Builder::from_resource("/com/jgcalderon/irc-client/chat-own-message.ui")
                .object("message")
                .unwrap();

        message.set_property("label", content);

        self.imp().message_list.append(&message);
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}
