mod chat;
pub mod constant;
mod handle;
mod imp;
mod message;
mod user_page;

use glib::Object;
use gtk::glib::{self, clone};
use gtk::prelude::{Cast, ObjectExt};
use gtk::subclass::prelude::*;

use crate::client::{IrcClient, UserData};
use crate::message::IrcCommand;

use self::chat::Chat;
use self::constant::SessionProperty;

const CHANNEL_INDICATOR: char = '#';

glib::wrapper! {
    pub struct Session(ObjectSubclass<imp::Session>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Session {
    pub fn new(client: IrcClient, data: UserData) -> Self {
        let session: Self = Object::builder()
            .property(&SessionProperty::Data, data)
            .build();

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

    fn get_or_insert_chat(&self, chat_name: String) -> Chat {
        self.imp()
            .chats
            .child_by_name(&chat_name)
            .map(|widget| widget.downcast().unwrap())
            .unwrap_or_else(|| self.add_chat(chat_name))
    }

    fn is_private_chat(&self, target: &str) -> bool {
        *target == self.nickname()
    }

    fn is_own_message(&self, sender: &str) -> bool {
        *sender == self.nickname()
    }

    fn nickname(&self) -> String {
        self.property::<UserData>(&SessionProperty::Data).nickname()
    }
}
