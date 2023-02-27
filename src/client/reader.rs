use std::{
    io::{self, BufRead, BufReader},
    net::TcpStream,
    sync::mpsc,
    thread,
};

use crate::message::IrcMessage;

const MESSAGE_SEPARATOR: &[u8] = b"\r\n";

pub fn spawn_reader(stream: TcpStream) -> io::Result<mpsc::Receiver<IrcMessage>> {
    let (sender, receiver) = mpsc::channel();

    let name = "message reader".to_string();
    thread::Builder::new()
        .name(name)
        .spawn(move || read_loop(stream, sender))?;

    Ok(receiver)
}

fn read_loop(stream: TcpStream, reader_sender: mpsc::Sender<IrcMessage>) {
    let mut reader = BufReader::new(stream);
    while let Ok(raw_message) = read_raw_message(&mut reader) {
        match IrcMessage::parse(&raw_message) {
            Ok(message) => {
                if reader_sender.send(message).is_err() {
                    return;
                }
            }
            Err(error) => eprintln!("Error while parsing server message, {error:?}"),
        }
    }
}

fn read_raw_message(stream: &mut BufReader<TcpStream>) -> io::Result<String> {
    let mut content = String::new();

    while !content.as_bytes().ends_with(MESSAGE_SEPARATOR) {
        read_line(stream, &mut content)?;
    }

    content.pop();
    content.pop();

    Ok(content)
}

fn read_line(stream: &mut BufReader<TcpStream>, content: &mut String) -> Result<(), io::Error> {
    let read = stream.read_line(content)?;
    if read == 0 {
        return Err(io::ErrorKind::UnexpectedEof)?;
    }

    Ok(())
}
