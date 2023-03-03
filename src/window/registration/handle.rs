use std::ops::ControlFlow;

use glib::{clone, MainContext};
use gtk::{
    glib,
    prelude::{ObjectExt, ToValue},
};

use crate::message::{IrcMessage, IrcResponse};

use super::Registration;

impl Registration {
    pub(super) fn start_client_handler(&self) {
        MainContext::default().spawn_local(clone!(@weak self as registration => async move {
            let mut client = registration.client();
            while let Ok(message) = client.receive().await {
                if let ControlFlow::Break(()) = registration.handle_message(message)  {
                    return
                }
            }
        }));
    }

    fn handle_message(&self, message: IrcMessage) -> ControlFlow<()> {
        if let IrcMessage::IrcResponse(response) = message {
            match response {
                IrcResponse::Welcome { .. } => self.handle_welcome()?,
                IrcResponse::NickCollision { .. } => self.handle_nick_collision()?,
            }
        }

        ControlFlow::Continue(())
    }

    fn handle_welcome(&self) -> ControlFlow<()> {
        self.emit_by_name::<()>("registered", &[self, &self.client().to_value()]);

        ControlFlow::Break(())
    }

    fn handle_nick_collision(&self) -> ControlFlow<()> {
        eprintln!("NICK COLLISION");

        ControlFlow::Continue(())
    }
}
