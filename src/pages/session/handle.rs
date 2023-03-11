//! This module contains the functions that handle the messages received from the server.

use gtk::glib::{self, clone, MainContext};

use super::Session;
use client::message::{IrcCommand, IrcMessage, IrcResponse};

impl Session {
    /// Starts the client handler
    ///
    /// This function will spawn a new task that will handle the messages received from the server
    ///
    /// For each message received, the `handle_message` function is called
    pub(super) fn start_client_handler(&self) {
        MainContext::default().spawn_local(clone!(@weak self as session => async move {
            let mut client = session.client();
            while let Ok(message) = client.receive().await {
                session.handle_message(message)
            }
        }));
    }

    /// Handles the message received from the server according to its variant
    pub fn handle_message(&self, message: IrcMessage) {
        match message {
            IrcMessage::IrcCommand(sender, command) => match command {
                IrcCommand::Privmsg { target, message } => {
                    self.handle_privmsg(sender, target, message)
                }
                IrcCommand::Quit { message } => {
                    self.handle_quit(sender, message);
                }
                IrcCommand::Join { name } => {
                    self.handle_join(sender, name);
                }
                IrcCommand::Pass { .. } | IrcCommand::Nick { .. } | IrcCommand::User { .. } => (),
            },
            IrcMessage::IrcResponse(response) => match response {
                IrcResponse::Welcome { .. }
                | IrcResponse::NickCollision { .. }
                | IrcResponse::NoNickname => (),
            },
        }
    }

    /// Handles a private message
    ///
    /// If the message is sent to an existing chat, the message is added to the chat
    /// If the message is sent to a new chat, a new chat is created and the message is added to it
    fn handle_privmsg(&self, sender: String, target: String, message: String) {
        if self.is_private_chat(&target) {
            let chat = self.get_or_insert_chat(sender);
            chat.add_message(message);
        } else if !self.is_own_message(&sender) {
            let chat = self.get_or_insert_chat(target);
            chat.add_message_with_sender(message, sender);
        }
    }

    /// Handles a quit message
    fn handle_quit(&self, _sender: String, _message: String) {
        println!("todo! quit!");
    }

    /// Handles a join message
    fn handle_join(&self, _sender: String, _name: String) {
        println!("todo! joined!");
    }
}
