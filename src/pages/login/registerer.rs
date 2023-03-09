//! This module encapsulates the sending of messages to the server
use std::io;

use client::message::IrcCommand;
use gtk::subclass::prelude::*;

use super::Login;

impl Login {
    /// Sends registration commands to the client
    pub(super) fn register_client(&self) -> io::Result<()> {
        self.send_pass()?;

        self.send_nick()?;

        self.send_user()
    }

    /// Sends pass command to the server, if a password is provided
    fn send_pass(&self) -> io::Result<()> {
        let password = self.imp().password.input();
        if password.is_empty() {
            return Ok(());
        }

        let pass_command = IrcCommand::Pass { password };

        self.client().send(pass_command)
    }

    /// Sends nick command to the server.
    ///
    /// An error message is shown if nickname field is empty
    fn send_nick(&self) -> io::Result<()> {
        let nickname = self.imp().nickname.input();

        if nickname.is_empty() {
            self.imp().nickname.set_error("This field is mandatory");
            return Ok(());
        }

        self.imp().nickname.unset_error();

        let nick_command = IrcCommand::Nick { nickname };

        self.client().send(nick_command)
    }

    /// Send user command to the server
    fn send_user(&self) -> io::Result<()> {
        let username = self.imp().username.input();
        let realname = self.imp().realname.input();
        let user_command = IrcCommand::User { username, realname };

        self.client().send(user_command)
    }
}
