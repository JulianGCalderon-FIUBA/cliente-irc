//! This module defines an [`IrcClient`]
//! 
//! This struct can be used to comunicate with an IRC Server
//!
//! - Connecting to said server
//! - Sending IrcCommands
//! - Receiving IrcMessages
mod data;
mod utils;

use async_std::{
    channel::{Receiver, Sender},
    io,
    net::{TcpStream, ToSocketAddrs},
    task::block_on,
};
use gtk::glib;
use std::io::ErrorKind;

use crate::message::{IrcCommand, IrcMessage};
pub use data::UserData;

use self::utils::{spawn_reader, spawn_writer};

/// Struct for comunicating with server
/// Derives Boxed, therefore it can comunicate well with Gtk4 rust bindings.
#[derive(glib::Boxed, Clone, Debug)]
#[boxed_type(name = "IrcClient")]
pub struct IrcClient {
    sender: Sender<IrcCommand>,
    receiver: Receiver<IrcMessage>,
}

impl IrcClient {
    /// Creates a new [`IrcClient`] connected to `address`
    ///
    /// Fails if connection could not be established
    pub fn connect<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let stream = block_on(TcpStream::connect(address))?;

        let sender = spawn_writer(stream.clone());
        let receiver = spawn_reader(stream);

        Ok(Self { sender, receiver })
    }

    /// Sends `IrcCommand` to the server
    ///
    /// Fails if connections with the server was drop
    pub fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        let result = self.sender.send_blocking(command);
        result.map_err(|_| unexpected_eof())
    }

    /// Returns the next incoming server `IrcMessage`
    ///
    /// Can be called from multiple clones of the same instance, although it is advised otherwise.
    pub async fn receive(&mut self) -> io::Result<IrcMessage> {
        let result = self.receiver.recv().await;
        result.map_err(|_| unexpected_eof())
    }
}

fn unexpected_eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "")
}
