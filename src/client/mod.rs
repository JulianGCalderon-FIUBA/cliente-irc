mod reader;
mod sender;

use std::{
    io::{self, ErrorKind},
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc,
    thread,
};

use reader::spawn_reader;
use sender::spawn_sender;

use crate::message::{IrcCommand, IrcMessage};

struct Client {
    sender: mpsc::Sender<IrcCommand>,
    receiver: Option<mpsc::Receiver<IrcMessage>>,
}

struct ClientConnection {}

impl Client {
    pub fn new<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        let reader_stream = stream.try_clone()?;
        let receiver = spawn_reader(reader_stream)?;

        let sender = spawn_sender(stream)?;

        Ok(Self {
            sender,
            receiver: Some(receiver),
        })
    }

    pub fn sender(&self) -> mpsc::Sender<IrcCommand> {
        self.sender.clone()
    }

    pub fn send(&self, command: IrcCommand) -> io::Result<()> {
        if self.sender.send(command).is_err() {
            return Err(ErrorKind::ConnectionAborted)?;
        }
        Ok(())
    }

    pub fn receiver(&mut self) -> mpsc::Receiver<IrcMessage> {
        let receiver = self.receiver.take();
        receiver.expect("receiver can only be retreived once")
    }

    pub fn for_each_incoming<F>(&mut self, f: F)
    where
        F: Fn(IrcMessage) + Send + 'static,
    {
        let receiver = self.receiver();
        thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                f(message);
            }
        });
    }
}
