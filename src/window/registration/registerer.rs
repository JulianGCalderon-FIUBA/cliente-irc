use std::io;

use gtk::{
    glib::{self, clone, MainContext},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::message::IrcCommand;

use super::Registration;

impl Registration {
    pub(super) fn register(&self) {
        MainContext::default().spawn_local(clone!(@weak self as registration => async move {
            if registration.try_register().await.is_err() {
                eprintln!("error while sending message to client");
            }
        }));
    }

    async fn try_register(&self) -> io::Result<()> {
        self.send_pass().await?;

        self.send_nick().await?;

        self.send_user().await
    }

    async fn send_pass(&self) -> io::Result<()> {
        let password = self.imp().password.text();
        let pass_command = IrcCommand::Pass { password };

        self.client().send(pass_command).await
    }

    async fn send_nick(&self) -> io::Result<()> {
        let nickname = self.imp().nickname.text();
        let nick_command = IrcCommand::Nick { nickname };

        self.client().send(nick_command).await
    }

    async fn send_user(&self) -> io::Result<()> {
        let username = self.imp().username.text();
        let realname = self.imp().realname.text();
        let user_command = IrcCommand::User { username, realname };

        self.client().send(user_command).await
    }
}
