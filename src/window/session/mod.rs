mod handle;
mod imp;
mod chat;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

use crate::client::IrcClient;

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
}
