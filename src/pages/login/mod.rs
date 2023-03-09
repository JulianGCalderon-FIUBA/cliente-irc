//! This module contains the login page

mod handle;
mod imp;
mod registerer;

use std::io;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

use crate::gtk_client::BoxedIrcClient;

glib::wrapper! {
    /// The login page
    ///
    /// This page is used to connect to the server and register the user
    ///
    /// Subclassifies `gtk::Box`
    ///
    /// # Features
    ///
    /// * User input is validated before being sent to the server
    /// * Once the user has connected to the server, then the address input is locked
    /// * After registration is complete, the `registered` signal is emitted
    ///
    /// If the program is executed with the `automatic-login` feature, then the client
    /// will automatically register with a random roman nickname.
    ///
    /// # Signals
    ///
    /// * `registered` - Emitted when the user is registered
    ///
    ///     Arguments:
    ///    - `client` - The client
    ///         * Type: `BoxedIrcClient`
    ///    - `data` - The user data
    ///        * Type: `RegistrationDataObject`
    ///
    /// # CSS nodes
    ///
    /// `Login` has a single CSS node with name `login`.
    pub struct Login(ObjectSubclass<imp::Login>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Login {
    /// Creates a new login page
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Sets up the client
    ///
    /// This function will attempt to connect to the server
    ///
    /// If the connection is successful, then the client handler is started
    fn setup_client(&self) -> io::Result<()> {
        self.connect_client()?;

        self.start_client_handler();

        Ok(())
    }

    /// Attempts to connect to the server
    fn connect_client(&self) -> io::Result<()> {
        let address = self.imp().address.input();

        let client = BoxedIrcClient::connect(address)?;

        self.imp().client.set(client).unwrap();

        self.imp().address.lock();

        Ok(())
    }

    /// Whether the client is connected
    fn connected(&self) -> bool {
        self.imp().client.get().is_some()
    }

    /// Gets the client
    fn client(&self) -> BoxedIrcClient {
        self.imp().client.get().unwrap().clone()
    }

    /// Only for testing purposes
    ///
    /// Registers the client with a random roman nickname
    #[cfg(feature = "automatic-login")]
    fn automatic_login(&self) {
        let generator = rnglib::RNG::try_from(&rnglib::Language::Roman).unwrap();
        let nickname = generator.generate_short();
        self.imp().nickname.set_input(&nickname);
        self.imp().connect_clicked();
    }
}

impl Default for Login {
    fn default() -> Self {
        Self::new()
    }
}
