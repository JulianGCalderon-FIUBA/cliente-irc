use gtk::{
    glib::{self, clone, MainContext},
    prelude::Cast,
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::message::{IrcCommand, IrcMessage, IrcResponse};

use super::Session;

impl Session {
    pub(super) fn start_client_handler(&self) {
        MainContext::default().spawn_local(clone!(@weak self as session => async move {
            let mut client = session.client();
            while let Ok(message) = client.receive().await {
                session.handle_message(message)
            }
        }));
    }

    pub fn handle_message(&self, message: IrcMessage) {
        match message {
            IrcMessage::IrcCommand(sender, command) => match command {
                IrcCommand::Privmsg { target, message } => {
                    self.handle_privmsg(sender, target, message)
                }
                IrcCommand::Quit { message } => {
                    self.handle_quit(sender, message);
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

    fn handle_privmsg(&self, sender: String, _target: String, message: String) {
        let chat = self
            .imp()
            .chats
            .child_by_name(&sender)
            .map(|widget| widget.downcast().unwrap())
            .unwrap_or_else(|| self.add_chat(sender));

        chat.add_message(message);
    }

    fn handle_quit(&self, _sender: String, _message: String) {
        todo!()
    }
}
