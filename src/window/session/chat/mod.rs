mod constant;
mod imp;

use glib::Object;
use gtk::{
    glib, prelude::ObjectExt, subclass::prelude::ObjectSubclassIsExt, traits::WidgetExt, Align,
};

pub use constant::{ChatProperty, ChatSignal};

use super::message::Message;

glib::wrapper! {
    pub struct Chat(ObjectSubclass<imp::Chat>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Chat {
    pub fn new(name: String) -> Self {
        Object::builder()
            .property(&ChatProperty::Name, name)
            .build()
    }

    pub fn connect_close<F>(&self, f: F)
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_local(&ChatSignal::Close, true, move |args| {
            let chat: Chat = args[0].get().unwrap();
            f(&chat);
            None
        });
    }

    pub fn connect_send<F>(&self, f: F)
    where
        F: Fn(&Self, String) + 'static,
    {
        self.connect_local(&ChatSignal::Send, true, move |args| {
            let chat: Chat = args[0].get().unwrap();
            let message: String = args[1].get().unwrap();
            f(&chat, message);
            None
        });
    }

    pub fn add_message(&self, message: String) {
        let message = create_external_message(message);

        self.imp().messages.append(&message);
    }

    pub fn add_message_with_sender(&self, message: String, sender: String) {
        let message = create_external_message(message);
        message.set_sender(sender);

        self.imp().messages.append(&message);
    }
}

fn create_external_message(message: String) -> Message {
    let message = Message::new(message);
    message.set_halign(Align::Start);
    message.add_css_class("external");
    message
}
