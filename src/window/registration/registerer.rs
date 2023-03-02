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
            if registration.register_aux().await.is_err() {
                eprintln!("error while sending message to client");
            }
        }));
    }

    async fn register_aux(&self) -> io::Result<()> {
        if self.imp().password.enabled() {
            self.send_pass().await?;
        }

        self.send_nick().await?;

        self.send_user().await
    }

    async fn send_user(&self) -> Result<(), io::Error> {
        let username = self.imp().username.text();
        let realname = self.imp().realname.text();
        let user_command = IrcCommand::User { username, realname };
        self.client().send(user_command).await
    }

    async fn send_nick(&self) -> Result<(), io::Error> {
        let nickname = self.imp().nickname.text();
        let nick_command = IrcCommand::Nick { nickname };
        self.client().send(nick_command).await?;
        Ok(())
    }

    async fn send_pass(&self) -> Result<(), io::Error> {
        let password = self.imp().password.text();
        let pass_command = IrcCommand::Pass { password };

        self.client().send(pass_command).await?;
        self.imp().password.disable();

        Ok(())
    }
}
