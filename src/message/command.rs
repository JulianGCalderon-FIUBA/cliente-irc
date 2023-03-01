use super::ParsingError;

const PRIVMSG: &str = "PRIVMSG";

pub enum IrcCommand {
    Privmsg { target: String, message: String },
}

impl IrcCommand {
    pub fn is_command(command: &str) -> bool {
        command == PRIVMSG
    }

    pub fn parse(
        command: String,
        arguments: Vec<String>,
        trailing: Option<String>,
    ) -> Result<Self, ParsingError> {
        match &command[..] {
            PRIVMSG => Self::new_privmsg(arguments, trailing),

            _ => Err(ParsingError::UnknownCommand(command)),
        }
    }

    pub fn new_privmsg(
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> Result<Self, ParsingError> {
        parameters.reverse();

        let message = trailing.ok_or(ParsingError::MissingParameter)?;
        let target = parameters.pop().ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Privmsg { target, message })
    }
}
