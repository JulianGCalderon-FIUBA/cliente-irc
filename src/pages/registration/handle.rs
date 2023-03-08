//! This modules encapsulates the receiving and handling of server messages

use std::ops::ControlFlow;

use glib::{clone, MainContext};
use gtk::glib;
use gtk::prelude::{ObjectExt, ToValue};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use super::{Registration, RegistrationSignal};
use crate::client::UserData;
use crate::message::{IrcMessage, IrcResponse};

impl Registration {
    /// Starts an asynchronous read of server messages until registration is complete
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

    /// Handles the message with the corresponding action
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

    /// After receiving a [´IrcResponse::Welcome´], the asynchronous read is finished and 'registered' signal is emited
    fn handle_welcome(
        &self,
        nickname: String,
        realname: String,
        username: String,
        hostname: String,
        servername: String,
    ) -> ControlFlow<()> {
        let data = UserData::new(nickname, realname, username, hostname, servername);

        self.emit_by_name::<()>(
            &RegistrationSignal::Registered,
            &[self, &self.client().to_value(), &data.to_value()],
        );

        ControlFlow::Break(())
    }

    /// Notifies the user that nickname is already in use
    fn handle_nick_collision(&self) -> ControlFlow<()> {
        self.imp().nickname.set_error("Nickname already in use");

        ControlFlow::Continue(())
    }
}
