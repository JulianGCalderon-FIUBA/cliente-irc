//! This modules defines all registration related structures
mod handle;
mod imp;
mod registerer;

use std::io;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

use crate::gtk_client::BoxedIrcClient;

glib::wrapper! {
    /// This windows manages the registration process
    ///
    /// Asks for user information and establishes connection with an IrcServer
    ///
    /// Derives [´gtk::Box´]
    pub struct Login(ObjectSubclass<imp::Login>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Login {
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

        let client = BoxedIrcClient::connect(address)?;

        self.imp().client.set(client).unwrap();

        self.imp().address.lock();

        Ok(())
    }

    fn connected(&self) -> bool {
        self.imp().client.get().is_some()
    }

    fn client(&self) -> BoxedIrcClient {
        self.imp().client.get().unwrap().clone()
    }
}

impl Default for Login {
    fn default() -> Self {
        Self::new()
    }
}
