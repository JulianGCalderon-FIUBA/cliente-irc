mod handle;
mod imp;

use glib::Object;
use gtk::glib::{self, clone, MainContext};
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
        self.imp()
            .client
            .set(client)
            .expect("Client should only be set once");

        MainContext::default().spawn_local(clone!(@weak self as session => async move {
            let mut client = session.client();
            while let Ok(message) = client.receive().await {
                session.handle_message(message)
            }
        }));
    }

    fn client(&self) -> IrcClient {
        self.imp()
            .client
            .get()
            .expect("Client should be set up on creation")
            .clone()
    }
}
