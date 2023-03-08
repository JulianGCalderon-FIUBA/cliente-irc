//! This module contains all IRC message variations.
//!
//! Messages are parsed acording to RFC 1459

mod command;
mod error;
mod parser;
mod response;

pub use command::IrcCommand;
pub use error::ParsingError;
pub use response::IrcResponse;

type Trail = Option<String>;
type Args = Vec<String>;

/// Parses messages coming from the server
pub enum IrcMessage {
    /// If a command comes from a server, it must always have a sender
    IrcCommand(String, IrcCommand),
    IrcResponse(IrcResponse),
}

impl IrcMessage {
    /// Creates an [IrcMessage] by parsing a [&str]
    ///
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
