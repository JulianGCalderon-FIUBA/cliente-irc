mod constant;
mod imp;

use glib::Object;
use gtk::{glib, prelude::ObjectExt};

pub use constant::{ChatProperty, ChatSignal};

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
}
