use async_std::channel::{Receiver, Sender};
use async_std::io::{self, ReadExt, WriteExt};
use async_std::net::TcpStream;
use gtk::glib::MainContext;

use crate::message::{IrcCommand, IrcMessage};

const MESSAGE_SEPARATOR: &[u8] = b"\r\n";

pub fn spawn_reader(mut stream: TcpStream) -> Receiver<IrcMessage> {
    let (sender, receiver) = async_std::channel::unbounded();

    MainContext::default().spawn_local(async move {
        while let Ok(raw_message) = read_irc_server_message(&mut stream).await {
            match IrcMessage::parse(&raw_message) {
                Ok(message) => {
                    if sender.send(message).await.is_err() {
                        return;
                    }
                }
                Err(error) => {
                    eprintln!("Error while parsing server message: {error:?}");
                }
            }
        }
    });

    receiver
}

pub fn spawn_writer(mut stream: TcpStream) -> Sender<IrcCommand> {
    let (sender, receiver) = async_std::channel::unbounded();
    MainContext::default().spawn_local(async move {
        while let Ok(command) = receiver.recv().await {
            if write!(stream, "{command}\r\n").await.is_err() {
                return;
            }
        }
    });

    sender
}

pub async fn read_irc_server_message(reader: &mut TcpStream) -> io::Result<String> {
    let mut content = String::new();

    while !content.as_bytes().ends_with(MESSAGE_SEPARATOR) {
        let mut buf = vec![0; 1];
        reader.read_exact(&mut buf).await?;
        content.push(buf[0] as char);
    }

    content.pop();
    content.pop();

    Ok(content)
}
