//! This module contains all [´Session`] related structures

mod handle;
mod imp;

use glib::Object;
use gtk::glib::{self, clone};
use gtk::prelude::{Cast, ObjectExt};
use gtk::subclass::prelude::*;

use crate::gtk_client::{BoxedIrcClient, RegistrationDataObject};
use crate::pages::Chat;
use client::message::IrcCommand;

const CHANNEL_INDICATOR: char = '#';

glib::wrapper! {
    /// This structures is used to handle de client functionaly.
    /// It's created with an already regeistered client.
    ///
    /// Handles all client related functionality,
    /// from sending messages to configuring the state of the client
    ///
    /// Subclassifies [´gtk::Box´]
    pub struct Session(ObjectSubclass<imp::Session>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Session {
    /// Creates a new Session for the provided client, client must already be registered.
    pub fn new(client: BoxedIrcClient, data: RegistrationDataObject) -> Self {
        let session: Self = Object::builder().property("user-data", data).build();

        session.setup_client(client);

        session
    }

    /// Assigns the client to the structure and starts
    /// an asynchronous read on server messages until the connections is closed
    fn setup_client(&self, client: BoxedIrcClient) {
        self.imp().client.set(client).unwrap();

        self.start_client_handler();
    }

    fn client(&self) -> BoxedIrcClient {
        self.imp().client.get().unwrap().clone()
    }

    /// Adds a new ´Chat´ with given name to stack.
    ///
    /// Chats are stored in the chat section of the sidebar for easy openning.
    fn add_chat(&self, title: String) -> Chat {
        let chat = Chat::new(title.clone());

        chat.connect_close(clone!(@weak self as session => move |chat| {
            session.imp().pages.remove(chat);
        }));
        chat.connect_send(clone!(@weak self as session => move |chat, message| {
            session.send_message(chat, message);
        }));

        let name = format!("chat-{title}");
        let page = self.imp().pages.add_titled(&chat, Some(&name), &title);

        if title.starts_with('#') {
            page.set_icon_name("system-users-symbolic");
        } else {
            page.set_icon_name("avatar-default-symbolic");
        }

        chat
    }

    /// Sends ´message´ to the target of the given chat.
    ///
    /// May fail on a connection error.
    fn send_message(&self, chat: &Chat, message: String) {
        let target = chat.property("name");
        let privmsg_command = IrcCommand::Privmsg { target, message };
        if self.client().send(privmsg_command).is_err() {
            println!("todo! connection error");
        };
    }

    /// If the chat does not already exists, it creates it.
    ///
    /// Returns the specified [´Chat´]
    fn get_or_insert_chat(&self, chat: String) -> Chat {
        let full_name = format!("chat-{chat}");
        self.imp()
            .pages
            .child_by_name(&full_name)
            .map(|widget| widget.downcast().unwrap())
            .unwrap_or_else(|| self.add_chat(chat))
    }

    /// Returns whether a received privmsg is for a private chat
    /// Received the target of the command
    fn is_private_chat(&self, target: &str) -> bool {
        *target == self.nickname()
    }

    /// Returns whether a channel message was sent by the user client
    /// Receives the sender of the command
    fn is_own_message(&self, sender: &str) -> bool {
        *sender == self.nickname()
    }

    // Shortcut for accessing the client's nickname
    fn nickname(&self) -> String {
        self.property::<RegistrationDataObject>("user-data")
            .property("nickname")
    }
}
