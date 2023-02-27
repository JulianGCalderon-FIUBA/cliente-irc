mod command;
mod error;
mod parser;
mod response;

use error::Error;

pub use command::IrcCommand;
use response::IrcResponse;

type Prefix = Option<String>;

pub enum IrcMessage {
    Command(Prefix, IrcCommand),
    Response(IrcResponse),
    Unknown(String),
}

impl IrcMessage {
    pub fn parse(message: &str) -> Result<Self, Error> {
        let (prefix, command, parameters, trailing) = parser::parse(message)?;

        if IrcCommand::is_valid_command(&command) {
            let command = IrcCommand::new(command, parameters, trailing)?;
            return Ok(Self::Command(prefix, command));
        }

        if IrcResponse::is_valid_response(&command) {
            let response = IrcResponse::new(command, parameters, trailing)?;
            return Ok(Self::Response(response));
        }

        Ok(Self::Unknown(message.to_string()))
    }
}
