use std::{
    io::{self, Write},
    net::TcpStream,
    sync::mpsc,
    thread,
};

use crate::message::IrcCommand;

pub fn spawn_sender(stream: TcpStream) -> io::Result<mpsc::Sender<IrcCommand>> {
    let (sender, receiver) = mpsc::channel();

    let name = "message reader".to_string();
    thread::Builder::new()
        .name(name)
        .spawn(move || send_loop(stream, receiver))?;

    Ok(sender)
}

fn send_loop(mut stream: TcpStream, sender: mpsc::Receiver<IrcCommand>) {
    while let Ok(command) = sender.recv() {
        if write!(stream, "{}\r\n", command).is_err() {
            return;
        }
    }
}
