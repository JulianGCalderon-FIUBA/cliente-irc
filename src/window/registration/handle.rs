use glib::{clone, MainContext};
use gtk::glib;

use crate::message::{IrcMessage, IrcResponse};

use super::Registration;

impl Registration {
    pub(super) fn start_client_handler(&self) {
        MainContext::default().spawn_local(clone!(@weak self as registration => async move {
            let mut client = registration.client();
            while let Ok(message) = client.receive().await {
                if registration.handle_message(message) {
                    return
                }
            }
        }));
    }

    fn handle_message(&self, message: IrcMessage) -> bool {
        match message {
            IrcMessage::IrcResponse(response) => match response {
                IrcResponse::Welcome { .. } => self.handle_welcome(),
                IrcResponse::NickCollision { .. } => self.handle_nick_collision(),
            },
            _ => false,
        }
    }

    fn handle_welcome(&self) -> bool {
        println!("welcome");

        true
    }

    fn handle_nick_collision(&self) -> bool {
        println!("nick collision");

        false
    }
}
