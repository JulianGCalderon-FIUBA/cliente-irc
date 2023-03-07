//! This module contains all [´Session`] related structures

pub mod constant;
mod handle;
mod imp;

use glib::Object;
use gtk::glib::{self, clone};
use gtk::prelude::{Cast, ObjectExt};
use gtk::subclass::prelude::*;

use crate::client::{IrcClient, UserData};
use crate::message::IrcCommand;

use self::constant::SessionProperty;
use crate::widgets::chat_page::ChatPage;

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
    pub fn new(client: IrcClient, data: UserData) -> Self {
        let session: Self = Object::builder()
            .property(&SessionProperty::Data, data)
            .build();

        session.setup_client(client);

        session
    }

    /// Assigns the client to the structure and starts
    /// an asynchronous read on server messages until the connections is closed
    fn setup_client(&self, client: IrcClient) {
        self.imp().client.set(client).unwrap();

        self.start_client_handler();
    }

    fn client(&self) -> IrcClient {
        self.imp().client.get().unwrap().clone()
    }

    /// Adds a new ´Chat´ with given name to stack.
    ///
    /// Chats are stored in the chat section of the sidebar for easy openning.
    fn add_chat(&self, name: String) -> ChatPage {
        let chat = ChatPage::new(name.clone());

        chat.connect_close(|_| println!("close"));
        chat.connect_send(clone!(@weak self as session => move |chat, message| {
            session.send_message(chat, message);
        }));

        self.imp().chats.add_titled(&chat, Some(&name), &name);

        chat
    }

    /// Sends ´message´ to the target of the given chat.
    ///
    /// May fail on a connection error.
    fn send_message(&self, chat: &ChatPage, message: String) {
        let target = chat.property("name");
        let privmsg_command = IrcCommand::Privmsg { target, message };
        if self.client().send(privmsg_command).is_err() {
            println!("todo! connection error");
        };
    }

    /// If the chat does not already exists, it creates it.
    ///
    /// Returns the specified [´Chat´]
    fn get_or_insert_chat(&self, chat_name: String) -> ChatPage {
        self.imp()
            .chats
            .child_by_name(&chat_name)
            .map(|widget| widget.downcast().unwrap())
            .unwrap_or_else(|| self.add_chat(chat_name))
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
        self.property::<UserData>(&SessionProperty::Data).nickname()
    }
}
