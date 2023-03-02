use std::thread;

use async_std::channel::{Receiver, Sender};
use async_std::io::{self, ReadExt, WriteExt};
use async_std::net::TcpStream;
use async_std::task::block_on;

use crate::message::{IrcCommand, IrcMessage};

const MESSAGE_SEPARATOR: &[u8] = b"\r\n";

pub fn spawn_reader(mut stream: TcpStream) -> Receiver<IrcMessage> {
    let (sender, receiver) = async_std::channel::unbounded();
    thread::spawn(move || {
        while let Ok(raw_message) = block_on(read_irc_server_message(&mut stream)) {
            let Ok(message) = IrcMessage::parse(&raw_message) else {continue};
            if sender.send_blocking(message).is_err() {
                return;
            }
        }
    });

    receiver
}

pub fn spawn_writer(mut stream: TcpStream) -> Sender<IrcCommand> {
    let (sender, receiver) = async_std::channel::unbounded();
    thread::spawn(move || {
        while let Ok(command) = receiver.recv_blocking() {
            if block_on(write!(stream, "{command}\r\n")).is_err() {
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
