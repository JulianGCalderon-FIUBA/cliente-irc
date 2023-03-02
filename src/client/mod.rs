mod r#async;
mod imp;

use std::io::ErrorKind;

use async_std::channel::{Receiver, Sender};
use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::task::block_on;
use gtk::glib::{self, Object};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::message::{IrcCommand, IrcMessage};

use r#async::{spawn_reader, spawn_writer};

glib::wrapper! {
    pub struct IrcClient(ObjectSubclass<imp::IrcClient>);
}

impl IrcClient {
    pub fn new<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let client: Self = Object::builder().build();

        client.connect(address)?;

        Ok(client)
    }

    fn connect<A: ToSocketAddrs>(&self, address: A) -> io::Result<()> {
        let stream = block_on(TcpStream::connect(address))?;

        let sender = spawn_writer(stream.clone());
        let receiver = spawn_reader(stream);

        self.imp().sender.set(sender).unwrap();
        self.imp().receiver.set(receiver).unwrap();

        Ok(())
    }

    fn sender(&self) -> Sender<IrcCommand> {
        self.imp().sender.get().unwrap().clone()
    }

    fn receiver(&self) -> Receiver<IrcMessage> {
        self.imp().receiver.get().unwrap().clone()
    }

    pub async fn send(&mut self, command: IrcCommand) -> io::Result<()> {
        let result = self.sender().send(command).await;
        result.map_err(|_| unexpected_eof())
    }

    pub async fn receive(&mut self) -> io::Result<IrcMessage> {
        let result = self.receiver().recv().await;
        result.map_err(|_| unexpected_eof())
    }
}

fn unexpected_eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "")
}
