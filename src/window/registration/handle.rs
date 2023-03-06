use std::ops::ControlFlow;

use glib::{clone, MainContext};
use gtk::{
    glib,
    prelude::{ObjectExt, ToValue},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::{
    client::ClientData,
    message::{IrcMessage, IrcResponse},
};

use super::{Registration, RegistrationSignal};

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
            if let IrcResponse::Welcome {
                nickname,
                realname,
                username,
                hostname,
                servername,
            } = response
            {
                self.handle_welcome(nickname, realname, username, hostname, servername)?
            } else if let IrcResponse::NickCollision { .. } = response {
                self.handle_nick_collision()?
            }
        }

        ControlFlow::Continue(())
    }

    fn handle_welcome(
        &self,
        nickname: String,
        realname: String,
        username: String,
        hostname: String,
        servername: String,
    ) -> ControlFlow<()> {
        let data = ClientData {
            nickname,
            realname,
            username,
            hostname,
            servername,
        };

        self.emit_by_name::<()>(
            &RegistrationSignal::Registered,
            &[self, &self.client().to_value(), &data.to_value()],
        );

        ControlFlow::Break(())
    }

    fn handle_nick_collision(&self) -> ControlFlow<()> {
        self.imp().nickname.set_error("Nickname already in use");

        ControlFlow::Continue(())
    }
}
