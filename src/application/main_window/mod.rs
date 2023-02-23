use glib::Object;
use gtk::{gio, glib, prelude::Cast, subclass::prelude::ObjectSubclassIsExt, Stack};

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

    pub fn get_or_insert_chat(&self, name: &str) -> Chat {
        if let Some(chat) = self.chats_stack().child_by_name(name) {
            return chat.downcast().unwrap();
        }

        let chat: Chat = Object::builder().property("client", name).build();
        self.chats_stack().add_titled(&chat, Some(name), name);

        chat
    }
}
