use crate::message::{IrcCommand, IrcMessage, IrcResponse};

use super::Session;

impl Session {
    pub(super) fn handle_message(&self, message: IrcMessage) {
        match message {
            IrcMessage::IrcCommand(sender, command) => match command {
                IrcCommand::Privmsg { target, message } => {
                    self.handle_privmsg(sender, target, message)
                }
                IrcCommand::Quit { message } => self.handle_quit(sender, message),
                IrcCommand::Pass { .. } | IrcCommand::Nick { .. } | IrcCommand::User { .. } => (),
            },
            IrcMessage::IrcResponse(response) => match response {
                IrcResponse::Welcome { .. } => (),
                IrcResponse::NickCollision { .. } => (),
            },
        }
    }

    fn handle_privmsg(&self, _sender: String, _target: String, _message: String) {
        todo!()
    }

    fn handle_quit(&self, _sender: String, _message: String) {
        todo!()
    }
}
