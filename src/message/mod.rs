//! This module contains all IRC message variations.
//!
//! All messages are parsed in `prefix`, `command`, `arguments`, `trailing`
//!
//! Example: `:prefix command argument1 argument2 argument3 :trai ling`
//!
//! - prefix: Optional, must be prefixed with `:`
//! - command: Mandatory
//! - arguments: whitespace separated arguments, not prefixed by an `:`
//! - trailing: last argument, prefixed by a `:`. Can contain whitespace
//!
//! Each IRC command has different number of arguments and validations,
//! acording to the protocol RFC 1459

mod command;
mod error;
mod parser;
mod response;

pub use command::IrcCommand;
pub use error::ParsingError;
pub use response::IrcResponse;

/// parses messages coming from the server
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
