mod field;
mod handle;
mod imp;
mod registerer;

use std::io;

use crate::client::IrcClient;
use glib::Object;
use gtk::{glib, subclass::prelude::*};

glib::wrapper! {
    pub struct Registration(ObjectSubclass<imp::Registration>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Registration {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setup_client(&self) -> io::Result<()> {
        self.connect_client()?;

        self.start_client_handler();

        Ok(())
    }

    fn connect_client(&self) -> io::Result<()> {
        let address = self.imp().address.text();

        let client = IrcClient::connect(address)?;
        self.imp().client.set(client).unwrap();

        Ok(())
    }

    fn connected(&self) -> bool {
        self.imp().client.get().is_some()
    }

    fn client(&self) -> IrcClient {
        self.imp().client.get().unwrap().clone()
    }
}

impl Default for Registration {
    fn default() -> Self {
        Self::new()
    }
}
