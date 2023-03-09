//! Defines the BoxedIrcClient struct

use std::io;

use client::message::{IrcCommand, IrcMessage};
use client::IrcClient;
use gtk::glib;

/// A wrapper for the IrcClient struct from the client crate
///
/// Derives the glib::Boxed trait to allow it to be used as a boxed type
#[derive(Debug, Clone, glib::Boxed)]
#[boxed_type(name = "BoxedIrcClient")]
pub struct BoxedIrcClient(IrcClient);

impl BoxedIrcClient {
    /// Connects to an IRC server at the given address.
    ///
    /// # Errors
    ///
    /// This method returns an error if the connection to the server fails.
    pub fn connect(address: String) -> io::Result<Self> {
        IrcClient::connect(address).map(BoxedIrcClient)
    }

    /// Sends a command to the server.
    ///
    /// # Errors
    ///
    /// This method returns an error if the connection to the server is closed.
    pub fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        self.0.send(command)
    }

    /// Receives a message from the server.
    ///
    /// The receiving of messages is done asynchronously as it integrates better with the
    /// asynchronous nature of the gtk4 library. Should be fixed in the future.
    ///
    /// # Errors
    ///
    /// This method returns an error if the connection to the server is closed.
    pub async fn receive(&mut self) -> io::Result<IrcMessage> {
        self.0.receive().await
    }
}
