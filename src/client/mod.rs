//! This module defines [`IrcClient`] and other related structures
//!
//! A client is connected to a server using a socket address.
//! After connection is established, the user must register to the network.
//! Once registered, user may send commands to the server and await for responses asynchronously
//!
//! This is indicated by RFC 1459

mod data;
mod utils;

use std::io::ErrorKind;

use async_std::channel::{Receiver, Sender};
use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::task::block_on;
pub use data::UserData;
use gtk::glib;

use self::utils::{spawn_reader, spawn_writer};
use crate::message::{IrcCommand, IrcMessage};

/// This struct can be used to comunicate with an IRC Server
///
/// - Establishing a connection
/// - Sending IrcCommands
/// - Receiving IrcMessages
///
/// Derives `glib::Boxed`
#[derive(glib::Boxed, Clone, Debug)]
#[boxed_type(name = "IrcClient")]
pub struct IrcClient {
    sender: Sender<IrcCommand>,
    receiver: Receiver<IrcMessage>,
}

impl IrcClient {
    /// Creates a new `IrcClient` connected to `address`
    ///
    /// Fails if connection could not be established
    pub fn connect<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let stream = block_on(TcpStream::connect(address))?;

        let sender = spawn_writer(stream.clone());
        let receiver = spawn_reader(stream);

        Ok(Self { sender, receiver })
    }

    /// Sends an [`IrcCommand`] to the server
    ///
    /// Fails if connection with the server was finalized
    pub fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        let result = self.sender.send_blocking(command);
        result.map_err(|_| unexpected_eof())
    }

    /// Returns the next incoming server [`IrcMessage`]
    ///
    /// Can be called from multiple clones of the same instance, although it is advised otherwise.
    ///
    /// Fails if connection with the server was finalized
    pub async fn receive(&mut self) -> io::Result<IrcMessage> {
        let result = self.receiver.recv().await;
        result.map_err(|_| unexpected_eof())
    }
}

fn unexpected_eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "")
}
