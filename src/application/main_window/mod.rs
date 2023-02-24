use glib::Object;
use gtk::{
    gio,
    glib::{self, closure_local},
    prelude::{Cast, ObjectExt, ToValue},
    subclass::prelude::ObjectSubclassIsExt,
    Stack,
};

use crate::application::Application;

use super::chat::Chat;

mod imp;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Window, gtk::Widget,
        @implements
            gio::ActionGroup, gio::ActionMap, gtk::Accessible,
            gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
            gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(application: &Application) -> Self {
        Object::builder()
            .property("application", application)
            .build()
    }

    fn chats_stack(&self) -> Stack {
        self.imp().chats_stack.clone()
    }

    pub fn get_or_add_chat(&self, name: &str) -> Chat {
        match self.chats_stack().child_by_name(name) {
            Some(chat) => chat.downcast().unwrap(),
            None => self.add_chat(name),
        }
    }

    fn add_chat(&self, name: &str) -> Chat {
        let chat: Chat = Object::builder().property("client", name).build();
        self.chats_stack().add_titled(&chat, Some(name), name);

        chat.connect_closure(
            "send-message",
            true,
            closure_local!(@strong self as main_window, @to-owned name =>
                move |_: Chat, message: String| {
                    main_window.emit_by_name::<()>("send-message", &[&message.to_value(), &name.to_value()]);
                }
            ),
        );

        chat
    }
}
