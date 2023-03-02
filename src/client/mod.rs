mod inner;

use std::io::ErrorKind;

use async_std::channel::{Receiver, Sender};
use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::task::block_on;

use crate::message::{IrcCommand, IrcMessage};

use inner::{spawn_reader, spawn_writer};

#[derive(Debug, Clone)]
pub struct IrcClient {
    sender: Sender<IrcCommand>,
    receiver: Receiver<IrcMessage>,
}

impl IrcClient {
    pub fn connect<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let stream = block_on(TcpStream::connect(address))?;

        let sender = spawn_writer(stream.clone());
        let receiver = spawn_reader(stream);

        Ok(Self { sender, receiver })
    }

    pub async fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        let result = self.sender.send(command).await;
        result.map_err(|_| unexpected_eof())
    }

    pub async fn receive(&mut self) -> io::Result<IrcMessage> {
        let result = self.receiver.recv().await;
        result.map_err(|_| unexpected_eof())
    }
}

fn unexpected_eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "")
}
