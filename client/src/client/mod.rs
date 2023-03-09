//! The `IrcClient` type.
//!
//! This module contains the `IrcClient` type, which is the main entry point for
//! the library. It provides methods to connect to an IRC server, send commands
//! to it, and receive messages from it.

mod utils;

use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::Sender;
use std::sync::Arc;

use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Mutex;

use self::utils::{spawn_reader, spawn_writer};
use crate::message::{IrcCommand, IrcMessage};

/// An IRC client.
///
/// It can be used to connect to an IRC server,
/// send commands to it, and receive messages from it.
#[derive(Debug, Clone)]
pub struct IrcClient {
    sender: Sender<IrcCommand>,
    /// The receiver is asynchronous because it integrates better with the
    /// asynchronous nature of the gtk4 library. Should be fixed in the future
    /// by investigating tokio to spawn asynchronous tasks successfully.
    ///
    /// It's wrapped in an `Arc` and a `Mutex` because it needs to be shared between tasks safely.
    receiver: Arc<Mutex<UnboundedReceiver<IrcMessage>>>,
}

impl IrcClient {
    /// Connects to an IRC server at the given address.
    ///
    /// # Errors
    ///
    /// This method returns an error if the connection to the server fails.
    pub fn connect<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        let receiver = spawn_reader(stream.try_clone()?);
        let sender = spawn_writer(stream);

        let receiver = Arc::new(Mutex::new(receiver));

        Ok(Self { sender, receiver })
    }

    /// Sends a command to the server.
    ///
    /// # Errors
    ///
    /// This method returns an error if the connection to the server is closed.
    pub fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        self.sender
            .send(command)
            .map_err(|_| connection_closed_error())
    }

    /// Receives a message from the server.
    ///
    /// # Errors
    ///
    /// This method returns an error if the connection to the server is closed.
    pub async fn receive(&mut self) -> io::Result<IrcMessage> {
        self.receiver
            .lock()
            .await
            .recv()
            .await
            .ok_or(connection_closed_error())
    }
}

/// Returns an error indicating that the connection to the server was closed.
fn connection_closed_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::ConnectionAborted,
        "connection with the remote server ended",
    )
}
