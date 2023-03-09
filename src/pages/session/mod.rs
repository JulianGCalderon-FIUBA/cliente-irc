//! This module contains the Session page

mod handle;
mod imp;

use glib::Object;
use gtk::glib::{self, clone};
use gtk::prelude::{Cast, ObjectExt};
use gtk::subclass::prelude::*;

use crate::gtk_client::{BoxedIrcClient, RegistrationDataObject};
use crate::pages::Chat;
use client::message::IrcCommand;

/// The character as prefix is used to indicate a channel
const CHANNEL_INDICATOR: char = '#';

const GROUP_CHAT_ICON: &str = "system-users-symbolic";
const PRIVATE_CHAT_ICON: &str = "avatar-default-symbolic";

glib::wrapper! {
    /// The session window is used to interact with the server
    ///
    /// Must be created with a valid, already registered client
    ///
    /// # Features
    ///
    /// * Private message between clients
    /// * Joining and leaving channels
    /// * Sending messages to channels
    ///
    /// Subclassifies `gtk::Box`
    ///
    /// # Properties
    ///
    /// * `registration-data` - The user registration data
    ///     - Type: `RegistrationDataObject`
    ///
    /// # CSS nodes
    ///
    /// `Session` has a single CSS node with name `session`.
    pub struct Session(ObjectSubclass<imp::Session>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Session {
    /// Creates a new session page with the given client and user data
    pub fn new(client: BoxedIrcClient, data: RegistrationDataObject) -> Self {
        let session: Self = Object::builder()
            .property("registration-data", data)
            .build();

        session.setup_client(client);

        session
    }

    /// Sets up the client
    ///
    /// This functions starts the client handler
    fn setup_client(&self, client: BoxedIrcClient) {
        self.imp().client.set(client).unwrap();

        self.start_client_handler();
    }

    /// Returns the client
    fn client(&self) -> BoxedIrcClient {
        self.imp().client.get().unwrap().clone()
    }

    /// Adds a new chat to the session with the given title
    fn add_chat(&self, title: String) -> Chat {
        let chat = Chat::new(title.clone());

        self.connect_chat_signals(&chat);

        let name = build_name_for_chat_title(&title);
        let page = self.imp().pages.add_titled(&chat, Some(&name), &title);

        set_chat_page_icon(page);

        chat
    }

    /// Connects the signals of the given chat to the session
    fn connect_chat_signals(&self, chat: &Chat) {
        chat.connect_close(clone!(@weak self as session => move |chat| {
            session.imp().pages.remove(chat);
        }));
        chat.connect_send(clone!(@weak self as session => move |chat, message| {
            session.send_message(chat, message);
        }));
    }

    /// Sends a message to the given chat
    fn send_message(&self, chat: &Chat, message: String) {
        let target = chat.property("name");
        let privmsg_command = IrcCommand::Privmsg { target, message };
        if self.client().send(privmsg_command).is_err() {
            println!("todo! connection error");
        };
    }

    /// Gets the chat with the given title or creates a new one
    fn get_or_insert_chat(&self, title: String) -> Chat {
        let name = build_name_for_chat_title(&title);
        self.imp()
            .pages
            .child_by_name(&name)
            .map(|widget| widget.downcast().unwrap())
            .unwrap_or_else(|| self.add_chat(title))
    }

    /// Whether the target of a PRIVMSG is a channel
    fn is_private_chat(&self, target: &str) -> bool {
        *target == self.nickname()
    }

    /// Whether the sender of a PRIVMSG is the own client
    fn is_own_message(&self, sender: &str) -> bool {
        *sender == self.nickname()
    }

    /// Returns the nickname of the client
    fn nickname(&self) -> String {
        self.property::<RegistrationDataObject>("registration-data")
            .property("nickname")
    }

    /// Join the given channel in the server
    fn join_channel(&self, name: String) {
        let join_command = IrcCommand::Join { name };
        if self.client().send(join_command).is_err() {
            println!("todo! connection error");
        }
    }
}

/// Sets the icon of the given page depending on the title
fn set_chat_page_icon(page: gtk::StackPage) {
    let Some(title) = page.title() else {return};

    let icon_name = if title.starts_with('#') {
        GROUP_CHAT_ICON
    } else {
        PRIVATE_CHAT_ICON
    };

    page.set_icon_name(icon_name);
}

/// Builds the name of the chat page for the given title
fn build_name_for_chat_title(title: &str) -> String {
    format!("chat-{title}")
}
