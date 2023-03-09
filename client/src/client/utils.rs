//! This module contains utility functions for the client.

use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;

use tokio::sync::mpsc::UnboundedReceiver;

use super::connection_closed_error;
use crate::message::{IrcCommand, IrcMessage};

const MESSAGE_SEPARATOR: &[u8] = b"\r\n";

/// Spawns a task that reads messages from a channel and sends them to the server.
///
/// Returns a sender that can be used to send commands to the server.
pub fn spawn_writer(mut stream: TcpStream) -> Sender<IrcCommand> {
    let (sender, receiver) = std::sync::mpsc::channel::<IrcCommand>();

    thread::spawn(move || {
        while let Ok(command) = receiver.recv() {
            if write!(stream, "{command}\r\n").is_err() {
                return;
            }
        }
    });

    sender
}

/// Spawns a task that reads messages from the server and sends them to a channel.
///
/// Returns a receiver that can be used to receive messages from the server.
pub fn spawn_reader(stream: TcpStream) -> UnboundedReceiver<IrcMessage> {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    let mut reader = BufReader::new(stream);

    thread::spawn(move || {
        while let Ok(raw_message) = read_raw_message(&mut reader) {
            let Ok(message) = IrcMessage::parse(&raw_message) else {continue};

            if sender.send(message).is_err() {
                return;
            };
        }
    });

    receiver
}

/// Reads a raw message from the server.
///
/// Utilizes a BufReader as its more efficient than reading byte by byte.
///
/// A raw message is a string that ends with "\r\n".
fn read_raw_message(reader: &mut BufReader<TcpStream>) -> io::Result<String> {
    let mut content = String::new();

    while !content.as_bytes().ends_with(MESSAGE_SEPARATOR) {
        let read = reader.read_line(&mut content)?;
        if read == 0 {
            return Err(connection_closed_error());
        }
    }

    content.pop();
    content.pop();

    Ok(content)
}
