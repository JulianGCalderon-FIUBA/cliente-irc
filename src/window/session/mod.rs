mod chat;
mod handle;
mod imp;
mod message;

use glib::Object;
use gtk::glib::{self, clone};
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::*;

use crate::client::IrcClient;
use crate::message::IrcCommand;

use self::chat::Chat;

glib::wrapper! {
    pub struct Session(ObjectSubclass<imp::Session>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Session {
    pub fn new(client: IrcClient) -> Self {
        let session: Self = Object::builder().build();

        session.setup_client(client);

        session
    }

    fn setup_client(&self, client: IrcClient) {
        self.imp().client.set(client).unwrap();

        self.start_client_handler();
    }

    fn client(&self) -> IrcClient {
        self.imp().client.get().unwrap().clone()
    }

    fn add_chat(&self, name: String) -> Chat {
        let chat = Chat::new(name.clone());

        chat.connect_close(|_| println!("close"));
        chat.connect_send(clone!(@weak self as session => move |chat, message| {
            session.send_message(chat, message);
        }));

        self.imp().chats.add_titled(&chat, Some(&name), &name);

        chat
    }

    fn send_message(&self, chat: &Chat, message: String) {
        let target = chat.property("name");
        let privmsg_command = IrcCommand::Privmsg { target, message };
        if self.client().send(privmsg_command).is_err() {
            println!("todo! connection error");
        };
    }
}
