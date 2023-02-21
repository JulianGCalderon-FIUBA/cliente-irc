use std::io::{Error, ErrorKind};
use std::rc::Rc;

use async_std::io::{self, prelude::*, BufReader, WriteExt};
use async_std::net::TcpStream;

const MESSAGE_SEPARATOR: &[u8] = b"\r\n";

#[derive(Debug, Clone)]
pub struct Server {
    stream: TcpStream,
    reader: Rc<BufReader<TcpStream>>,
}

impl Server {
    pub async fn connect(address: String) -> io::Result<Self> {
        let stream = TcpStream::connect(address).await?;
        let reader = Rc::new(BufReader::new(stream.clone()));

        Ok(Self { stream, reader })
    }

    pub async fn send(&mut self, message: String) -> io::Result<()> {
        write!(&mut self.stream, "{message}\r\n").await
    }

    pub async fn receive(&mut self) -> io::Result<String> {
        let reader = Rc::get_mut(&mut self.reader).unwrap();
        read_irc_server_message(reader).await
    }
}

pub async fn read_irc_server_message(reader: &mut BufReader<TcpStream>) -> io::Result<String> {
    let mut content = String::new();

    while !content.as_bytes().ends_with(MESSAGE_SEPARATOR) {
        let read = reader.read_line(&mut content).await?;
        if read == 0 {
            return Err(unexpected_eof_error());
        }
    }

    content.pop();
    content.pop();

    Ok(content)
}

fn unexpected_eof_error() -> Error {
    Error::new(ErrorKind::UnexpectedEof, "")
}
