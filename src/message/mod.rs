mod command;
mod error;
mod parser;
mod response;

pub use command::IrcCommand;
pub use error::ParsingError;
pub use response::IrcResponse;

pub enum IrcMessage {
    IrcCommand(String, IrcCommand),
    IrcResponse(IrcResponse),
}

impl IrcMessage {
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
