//! Contains utils for comunicating with the server spawning futures
//! for either reading or writing to said server

use async_std::channel::{Receiver, Sender};
use async_std::io::prelude::BufReadExt;
use async_std::io::{self, BufReader, WriteExt};
use async_std::net::TcpStream;
use glib::MainContext;
use gtk::glib;

use crate::message::{IrcCommand, IrcMessage};

use super::unexpected_eof;

const MESSAGE_SEPARATOR: &[u8] = b"\r\n";

/// Spawns a future in [`MainContext`] that reads messages from `stream` and sends it
/// to the returned receiver until the connection with the server or all receivers are dropped
///
/// Reading message from server is blocking, so on dropping al receivers, future wont finalize until
/// encountering an EOF or a new message.
pub fn spawn_reader(stream: TcpStream) -> Receiver<IrcMessage> {
    let (sender, receiver) = async_std::channel::unbounded();

    let mut reader = BufReader::new(stream);
    MainContext::default().spawn_local(async move {
        while let Ok(raw_message) = read_irc_server_message(&mut reader).await {
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

/// Spawns a future in [`MainContext`] that reads messages from `sender` and sends it
/// to the server until connection is closed or after droppping all senders.
///
/// Reading message from channel is blocking, so on connection closed, future wont finalize until
/// encountering on EOF (dropped senders) or a new command.
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

/// Reads a CRLF trailled message from a [`BufReader<TcpStream>`].
/// Fails on an invalid utf8 byte, or after encountering an EOF.
pub async fn read_irc_server_message(reader: &mut BufReader<TcpStream>) -> io::Result<String> {
    let mut content = String::new();

    while !content.as_bytes().ends_with(MESSAGE_SEPARATOR) {
        let read = reader.read_line(&mut content).await?;
        if read == 0 {
            return Err(unexpected_eof());
        }
    }

    content.pop();
    content.pop();

    Ok(content)
}
