mod command;
mod error;
mod parser;
mod response;

pub use command::IrcCommand;
pub use error::ParsingError;
pub use response::IrcResponse;

pub enum IrcMessage {
    IrcCommand(String, IrcCommand),
    IrcResponse(response::IrcResponse),
}

impl IrcMessage {
    pub fn parse(message: &str) -> Result<Self, ParsingError> {
        let (prefix, command, arguments, trailing) = parser::parse(message)?;

        if command::IrcCommand::is_command(&command) {
            let prefix = prefix.ok_or(ParsingError::MissingPrefix)?;
            let command = command::IrcCommand::parse(command, arguments, trailing)?;
            return Ok(IrcMessage::IrcCommand(prefix, command));
        }

        let response = response::IrcResponse::parse(command, arguments, trailing)?;

        Ok(Self::IrcResponse(response))
    }
}
