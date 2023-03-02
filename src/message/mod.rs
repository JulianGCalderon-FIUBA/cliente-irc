//! This module contains all IRC message variations.

mod command;
mod error;
mod parser;
mod response;

pub use command::IrcCommand;
pub use error::ParsingError;
pub use response::IrcResponse;

/// [IrcMessage] is sent by the server, can be either a command or a response.
/// First argument of a command is the sender of the command
pub enum IrcMessage {
    IrcCommand(String, IrcCommand),
    IrcResponse(IrcResponse),
}

impl IrcMessage {
    /// Parses an [IrcMessage] from a [&str]
    /// Fails on an invalid or unknown command
    pub fn parse(message: &str) -> Result<Self, ParsingError> {
        let (prefix, command, arguments, trailing) = parser::parse(message)?;

        if IrcCommand::is_command(&command) {
            let prefix = prefix.ok_or(ParsingError::MissingPrefix)?;
            let command = IrcCommand::new(command, arguments, trailing)?;
            return Ok(IrcMessage::IrcCommand(prefix, command));
        }

        if IrcResponse::is_response(&command) {
            let response = IrcResponse::new(command, arguments, trailing)?;
            return Ok(IrcMessage::IrcResponse(response));
        }

        Err(ParsingError::UnknownCommand(message.to_string()))
    }
}
