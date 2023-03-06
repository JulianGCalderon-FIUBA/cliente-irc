//! This modules defines [`Registration`]
mod constant;
mod field;
mod handle;
mod imp;
mod password_field;
mod registerer;

use std::io;

use crate::client::IrcClient;
use glib::Object;
use gtk::{glib, subclass::prelude::*};

pub use constant::RegistrationSignal;

glib::wrapper! {
    /// This windows manages the registration process
    ///
    /// Asks for user information and establishes connection with an IrcServer
    ///
    /// Derives GtkBox
    pub struct Registration(ObjectSubclass<imp::Registration>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Registration {
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Attempts to connect with the server.
    ///
    /// if successfull, starts an asynchronous read of server messages until registration is complete
    fn setup_client(&self) -> io::Result<()> {
        self.connect_client()?;

        self.start_client_handler();

        Ok(())
    }

    /// Attempts to connect with the server
    fn connect_client(&self) -> io::Result<()> {
        let address = self.imp().address.input();

        let client = IrcClient::connect(address)?;

        self.imp().client.set(client).unwrap();

        self.imp().address.lock();

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
