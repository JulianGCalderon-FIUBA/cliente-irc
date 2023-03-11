//! IRC message parsing and serialization.
//!
//! This crate provides a set of types and functions to parse and serialize IRC messages.
//!
//! All types are defines according to the protocol RFC 1459

mod command;
mod error;
mod parser;
mod response;

pub use command::IrcCommand;
pub use error::ParsingError;
pub use response::IrcResponse;

type Trail = Option<String>;
type Args = Vec<String>;

/// IRC message
///
/// This enum represents an IRC message. It can be either a command or a response.
///
pub enum IrcMessage {
    /// IRC command
    ///
    /// This variant represents an IRC command. It contains the sender of the command
    IrcCommand(String, IrcCommand),
    /// IRC response
    ///
    /// This variant represents an IRC response. It contains the response itself.
    IrcResponse(IrcResponse),
}

impl IrcMessage {
    /// Parses an IRC message
    ///
    /// This function parses an IRC message and returns an IrcMessage enum.
    ///
    /// # Errors
    ///
    /// This function returns an error if the message is not a valid IRC message.
    pub fn parse(message: &str) -> Result<Self, ParsingError> {
        let (prefix, command, arguments, trailing) = parser::parse(message)?;

        if IrcCommand::is_command(&command) {
            let prefix = prefix.ok_or(ParsingError::MissingSender)?;
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
