use glib::Object;
use gtk::{
    gio,
    glib::{self, closure_local, Value},
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
        let chat: Chat = Object::builder().property("chat-name", name).build();
        let _page = self.chats_stack().add_titled(&chat, Some(name), name);

        chat.connect_closure(
            "send-message-request",
            false,
            closure_local!(@strong self as main_window =>
                move |chat: Chat, message: String| {
                    let name: Value = chat.property("chat-name");
                    main_window.emit_by_name::<()>("send-message-request", &[&message.to_value(), &name]);
                }
            ),
        );

        chat.connect_closure(
            "close-request",
            false,
            closure_local!(@strong self as main_window =>
                move |chat: Chat| {
                    main_window.chats_stack().remove(&chat);
                }
            ),
        );

        chat
    }
}
