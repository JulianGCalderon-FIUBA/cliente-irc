mod chat;
mod handle;
mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

use crate::client::IrcClient;

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

    fn add_chat(&self, name: String) {
        let chat = Chat::new(name.clone());

        chat.connect_close(|_| println!("close"));
        chat.connect_send(|_, _| println!("send"));

        self.imp().chats.add_titled(&chat, Some(&name), &name);
    }
}
