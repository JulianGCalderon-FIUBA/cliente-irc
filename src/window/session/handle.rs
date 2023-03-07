//! This modules contains functionality for handling server messages

use gtk::glib::{self, clone, MainContext};

use crate::message::{IrcCommand, IrcMessage, IrcResponse};

use super::Session;

impl Session {
    /// Starts an asynchronous read of server messages, handling each one.
    pub(super) fn start_client_handler(&self) {
        MainContext::default().spawn_local(clone!(@weak self as session => async move {
            let mut client = session.client();
            while let Ok(message) = client.receive().await {
                session.handle_message(message)
            }
        }));
    }

    /// Calls the acording function for each message received.
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

    fn handle_privmsg(&self, sender: String, target: String, message: String) {
        if self.is_private_chat(&target) {
            let chat = self.get_or_insert_chat(sender);
            chat.add_message(message);
        } else if !self.is_own_message(&sender) {
            let chat = self.get_or_insert_chat(target);
            chat.add_message_with_sender(message, sender);
        }
    }

    fn handle_quit(&self, _sender: String, _message: String) {
        println!("todo! quit!");
    }

    fn handle_join(&self, _sender: String, _name: String) {
        println!("todo! joined!");
    }
}
