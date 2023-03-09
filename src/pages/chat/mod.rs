//! Defines the [`Chat`] page

mod imp;

use glib::Object;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::traits::WidgetExt;
use gtk::{glib, Align};

use crate::components::Message;

glib::wrapper! {
    /// The chat page is used to display the chat messages
    ///
    /// It also allows the user to send messages
    ///
    /// When the user sends a message, the `send` signal is emitted.
    /// And the message is added to the chat.
    ///
    /// External messages are added to the chat with the `add_message` method.
    /// They may have a sender
    ///
    /// Subclassifies `gtk::Box`
    ///
    /// # Properties
    ///
    /// * `name` - The name of the chat
    ///    - Type: `String`
    ///
    /// # Signals
    ///
    /// * `close` - Emitted when the chat must be closed
    /// * `send` - Emitted when a message must be sent
    ///     - Arguments:
    ///        - `message` - The message to send
    ///           * Type: `String`
    ///
    /// # CSS nodes
    ///
    /// `Chat` has a single CSS node with name `chat`.
    ///
    ///  Message have a CSS node with name `message`.
    ///
    pub struct Chat(ObjectSubclass<imp::Chat>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Chat {
    /// Creates a new [`Chat`] with the given name
    pub fn new(name: String) -> Self {
        Object::builder().property("name", name).build()
    }

    /// Connects to the `close` signal.
    pub fn connect_close<F>(&self, f: F)
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_local("close", true, move |args| {
            let chat: Chat = args[0].get().unwrap();
            f(&chat);
            None
        });
    }

    /// Connects to the `send` signal.
    pub fn connect_send<F>(&self, f: F)
    where
        F: Fn(&Self, String) + 'static,
    {
        self.connect_local("send", true, move |args| {
            let chat: Chat = args[0].get().unwrap();
            let message: String = args[1].get().unwrap();
            f(&chat, message);
            None
        });
    }

    /// Adds an external message.
    pub fn add_message(&self, message: String) {
        let message = create_external_message(message);

        self.imp().messages.append(&message);
    }

    /// Adds an external message with a sender.
    pub fn add_message_with_sender(&self, message: String, sender: String) {
        let message = create_external_message(message);
        message.set_sender(sender);

        self.imp().messages.append(&message);
    }
}

/// Creates an external message
/// The message is aligned to the start
/// And has the `external` CSS class
fn create_external_message(message: String) -> Message {
    let message = Message::new(message);
    message.set_halign(Align::Start);
    message.add_css_class("external");
    message
}

/// Creates an own message
/// The message is aligned to the end
/// And has the `own` CSS class
fn create_own_message(message: String) -> Message {
    let message = Message::new(message);
    message.set_halign(Align::End);
    message.add_css_class("own");
    message
}
