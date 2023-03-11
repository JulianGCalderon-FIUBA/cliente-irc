//! This module contains the the handling of server messages

use std::ops::ControlFlow;

use client::data::RegistrationData;
use client::message::{IrcMessage, IrcResponse};
use glib::{clone, MainContext};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::gtk_client::RegistrationDataObject;

use super::Login;

impl Login {
    /// Starts an asynchronous read from the server until [´ControlFlow::Break(())´] is returned
    ///
    /// Calls [´Login::handle_message´] for each message received
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

    /// Handles a message received from the server
    ///
    /// If the handler returns [´ControlFlow::Break(())´] then this function will return
    /// [´ControlFlow::Break(())´] as well
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

    /// builds the registration data and emits the 'registered' signal
    ///
    /// Returns [´ControlFlow::Break(())´]
    fn handle_welcome(
        &self,
        nickname: String,
        realname: String,
        username: String,
        hostname: String,
        servername: String,
    ) -> ControlFlow<()> {
        let data = RegistrationData {
            nickname,
            realname,
            username,
            hostname,
            servername,
        };
        let data = RegistrationDataObject::new(data);

        self.emit_registered_signal(data);

        ControlFlow::Break(())
    }

    /// Notifies the user that the nickname is already in use
    ///
    /// Returns [´ControlFlow::Continue(())´]
    fn handle_nick_collision(&self) -> ControlFlow<()> {
        self.imp().nickname.set_error("Nickname already in use");

        ControlFlow::Continue(())
    }

    /// Emits the 'registered' signal
    fn emit_registered_signal(&self, data: RegistrationDataObject) {
        self.emit_by_name(
            "registered",
            &[self, &self.client().to_value(), &data.to_value()],
        )
    }
}
