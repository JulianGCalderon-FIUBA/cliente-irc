mod chat;
mod handle;
mod imp;
mod message;

use glib::Object;
use gtk::glib::{self, clone};
use gtk::prelude::{Cast, ObjectExt};
use gtk::subclass::prelude::*;

use crate::client::{ClientData, IrcClient};
use crate::message::IrcCommand;

use self::chat::Chat;

const CHANNEL_INDICATOR: char = '#';

glib::wrapper! {
    pub struct Session(ObjectSubclass<imp::Session>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Session {
    pub fn new(client: IrcClient, data: ClientData) -> Self {
        let session: Self = Object::builder().build();

        session.setup_client(client);
        session.setup_data(data);

        session
    }

    fn setup_client(&self, client: IrcClient) {
        self.imp().client.set(client).unwrap();

        self.start_client_handler();
    }

    fn setup_data(&self, data: ClientData) {
        self.imp().info.set_label(&data.nickname);

        self.imp().client_data.replace(data);
    }

    fn client(&self) -> IrcClient {
        self.imp().client.get().unwrap().clone()
    }

    fn client_data(&self) -> ClientData {
        self.imp().client_data.borrow().clone()
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

    fn get_or_insert_chat(&self, chat_name: String) -> Chat {
        self.imp()
            .chats
            .child_by_name(&chat_name)
            .map(|widget| widget.downcast().unwrap())
            .unwrap_or_else(|| self.add_chat(chat_name))
    }

    fn is_private_chat(&self, target: &str) -> bool {
        *target == self.client_data().nickname
    }

    fn is_own_message(&self, sender: &str) -> bool {
        *sender == self.client_data().nickname
    }
}
