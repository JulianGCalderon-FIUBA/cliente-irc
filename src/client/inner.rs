use async_std::channel::{Receiver, Sender};
use async_std::net::TcpStream;

use crate::message::{IrcCommand, IrcMessage};

pub fn spawn_reader(stream: TcpStream) -> Receiver<IrcMessage> {
    todo!()
}

pub fn spawn_writer(stream: TcpStream) -> Sender<IrcCommand> {
    todo!()
}

// pub async fn read_irc_server_message(reader: &mut TcpStream) -> io::Result<String> {
//     let mut content = String::new();

//     while !content.as_bytes().ends_with(MESSAGE_SEPARATOR) {
//         let mut buf = vec![0; 1];
//         reader.read_exact(&mut buf).await?;
//         content.push(buf[0] as char);
//     }

//     content.pop();
//     content.pop();

//     Ok(content)
// }
