use std::fmt::Display;

use async_std::io::{self, prelude::*, WriteExt};
use async_std::net::{TcpStream, ToSocketAddrs};

const MESSAGE_SEPARATOR: &[u8] = b"\r\n";

#[derive(Debug, Clone)]
pub struct Server {
    stream: TcpStream,
}

impl Server {
    pub async fn connect<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let stream = TcpStream::connect(address).await?;

        Ok(Self { stream })
    }

    pub async fn send<D: Display>(&mut self, message: D) -> io::Result<()> {
        write!(&mut self.stream, "{message}\r\n").await
    }

    pub async fn receive(&mut self) -> io::Result<String> {
        read_irc_server_message(&mut self.stream).await
    }
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
