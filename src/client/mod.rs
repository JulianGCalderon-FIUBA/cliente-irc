mod imp;
mod utils;

use std::io::ErrorKind;

use async_std::channel::{Receiver, Sender};
use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::task::block_on;
use gtk::glib::{self, Object};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::message::{IrcCommand, IrcMessage};

use utils::{spawn_reader, spawn_writer};

glib::wrapper! {
    /// IrcClient encapsulates the logic of comunicating with an IrcServer.
    ///
    /// - Connecting to said server
    /// - Sending IrcCommands
    /// - Receiving IrcMessages
    ///
    /// IrcClient its defined as a subclass of GObject, therefore it can glue well with Gtk4 rust bindings.
    ///
    /// IrcClient uses async functions to ease the spawning of futures, as Gtk4 objects are not thread safe.
    pub struct IrcClient(ObjectSubclass<imp::IrcClient>);
}

impl IrcClient {
    /// Creates an [IrcClient] connected to a given address.
    ///
    /// Fails if connections could not be established
    pub fn new<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let client: Self = Object::builder().build();

        client.connect(address)?;

        Ok(client)
    }

    /// Connects to the server through a [TcpStream]
    ///
    /// Configures server comunication channels
    ///
    /// Fails if connections could not be established
    fn connect<A: ToSocketAddrs>(&self, address: A) -> io::Result<()> {
        let stream = block_on(TcpStream::connect(address))?;

        let sender = spawn_writer(stream.clone());
        let receiver = spawn_reader(stream);

        self.imp().sender.set(sender).unwrap();
        self.imp().receiver.set(receiver).unwrap();

        Ok(())
    }

    /// Returns a sender of server commands.
    ///
    /// Messages sent through the channel will be sent asynchronously to the server.
    pub fn sender(&self) -> Sender<IrcCommand> {
        self.imp().sender.get().unwrap().clone()
    }

    /// Returns a receiver of server messages
    ///
    /// All server messages are read asynchronosuly and sent to this channel.
    ///
    /// Multiple channels can be retreived, although it is advised otherwise.
    pub fn receiver(&self) -> Receiver<IrcMessage> {
        self.imp().receiver.get().unwrap().clone()
    }

    /// Sends `command` to the server
    ///
    /// Fails if connections with the server was drop
    ///
    /// There is no confirmation that the command reached the server,
    /// as connection can be dropped after this function returns
    ///
    /// A sender is cloned with each call to `send`, so it is advised to use `sender` for multiple subsequent calls
    pub async fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        let result = self.sender().send(command).await;
        result.map_err(|_| unexpected_eof())
    }

    /// Returns the next incoming server message
    ///
    /// Can be called from multiple clones of an IrcClient, although it is advised otherwise.
    pub async fn receive(&mut self) -> io::Result<IrcMessage> {
        let result = self.receiver().recv().await;
        result.map_err(|_| unexpected_eof())
    }
}

fn unexpected_eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "")
}
