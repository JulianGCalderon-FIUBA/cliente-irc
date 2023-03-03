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

use self::utils::{spawn_reader, spawn_writer};

/// Encapsulates the logic of comunicating with an IRC Server.
///
/// - Connecting to said server
/// - Sending IrcCommands
/// - Receiving IrcMessages
///
/// Derives Boxed, therefore it can comunicate well with Gtk4 rust bindings.
///
/// Uses async functions to ease the spawning of futures, as Gtk4 objects are not thread safe.
#[derive(glib::Boxed, Clone, Debug)]
#[boxed_type(name = "IrcClientBox")]
pub struct IrcClient {
    sender: Sender<IrcCommand>,
    receiver: Receiver<IrcMessage>,
}

impl IrcClient {
    /// Creates a new [IrcClient] connected to `address`
    ///
    /// Fails if connection could not be established
    pub fn connect<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let stream = block_on(TcpStream::connect(address))?;

        let sender = spawn_writer(stream.clone());
        let receiver = spawn_reader(stream);

        Ok(Self { sender, receiver })
    }

    /// Sends `command` to the server
    ///
    /// Fails if connections with the server was drop
    pub async fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        let result = self.sender.send(command).await;
        result.map_err(|_| unexpected_eof())
    }

    /// Returns the next incoming server message
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
