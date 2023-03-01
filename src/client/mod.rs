mod inner;

use std::fmt::Display;
use std::io::ErrorKind;

use async_std::channel::{Receiver, Sender};
use async_std::io::{self, prelude::*, WriteExt};
use async_std::net::{TcpStream, ToSocketAddrs};

use crate::message::{IrcCommand, IrcMessage};

use inner::{spawn_reader, spawn_writer};

const MESSAGE_SEPARATOR: &[u8] = b"\r\n";

#[derive(Debug, Clone)]
pub struct IrcClient {
    sender: Sender<IrcCommand>,
    receiver: Option<Receiver<IrcMessage>>,
}

impl IrcClient {
    pub async fn connect<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let stream = TcpStream::connect(address).await?;

        let sender = spawn_writer(stream.clone());
        let receiver = spawn_reader(stream);

        Ok(Self {
            sender,
            receiver: Some(receiver),
        })
    }

    pub async fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        let result = self.sender.send(command).await;
        result.map_err(|_| unexpected_eof())
    }

    pub async fn receive(&mut self) -> io::Result<IrcMessage> {
        let result = self.receiver.as_mut().unwrap().recv().await;
        result.map_err(|_| unexpected_eof())
    }
}

fn unexpected_eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "")
}
