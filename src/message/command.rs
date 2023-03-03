use std::fmt::Display;

use super::ParsingError;

const PRIVMSG: &str = "PRIVMSG";
const PASS: &str = "PASS";
const NICK: &str = "NICK";
const USER: &str = "USER";
const QUIT: &str = "QUIT";

/// Commands that can be sent to or received from a server
///
/// They are asynchronous messages that are not generated as a response,
///  but rather notify of a new status or notification.
#[derive(Debug)]
pub enum IrcCommand {
    Privmsg { target: String, message: String },
    Pass { password: String },
    Nick { nickname: String },
    User { username: String, realname: String },
    Quit { message: String },
}

impl IrcCommand {
    /// Creates the corresponding variation from the given parameters,
    ///
    /// Fails on invalid `response` or on an invalid argument
    pub fn new(
        command: String,
        arguments: Vec<String>,
        trailing: Option<String>,
    ) -> Result<Self, ParsingError> {
        match &command[..] {
            PRIVMSG => Self::new_privmsg(arguments, trailing),
            PASS => Self::new_pass(arguments, trailing),
            NICK => Self::new_nick(arguments, trailing),
            USER => Self::new_user(arguments, trailing),
            QUIT => Self::new_quit(arguments, trailing),
            _ => Err(ParsingError::UnknownCommand(command)),
        }
    }

    pub fn is_command(command: &str) -> bool {
        command == PRIVMSG
            || command == PASS
            || command == NICK
            || command == USER
            || command == QUIT
    }

    /// Creates a [IrcCommand::Privmsg] from the message arguments.
    ///
    /// Fails on invalid arguments
    pub fn new_privmsg(
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> Result<Self, ParsingError> {
        parameters.reverse();

        let message = trailing.ok_or(ParsingError::MissingParameter)?;
        let target = parameters.pop().ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Privmsg { target, message })
    }

    /// Creates a [IrcCommand::Pass] from the message arguments.
    ///
    /// Fails on invalid arguments
    fn new_pass(mut args: Vec<String>, _trail: Option<String>) -> Result<IrcCommand, ParsingError> {
        let password = args.pop().ok_or(ParsingError::MissingParameter)?;
        Ok(Self::Pass { password })
    }

    /// Creates a [IrcCommand::Nick] from the message arguments.
    ///
    /// Fails on invalid arguments
    fn new_nick(mut args: Vec<String>, _trail: Option<String>) -> Result<IrcCommand, ParsingError> {
        let nickname = args.pop().ok_or(ParsingError::MissingParameter)?;
        Ok(Self::Nick { nickname })
    }

    /// Creates a [IrcCommand::User] from the message arguments.
    ///
    /// Fails on invalid arguments
    fn new_user(mut args: Vec<String>, trail: Option<String>) -> Result<IrcCommand, ParsingError> {
        let username = args.pop().ok_or(ParsingError::MissingParameter)?;
        let realname = trail.ok_or(ParsingError::MissingParameter)?;

        Ok(Self::User { username, realname })
    }

    /// Creates a [IrcCommand::Quit] from the message arguments.
    ///
    /// Fails on invalid arguments
    fn new_quit(_args: Vec<String>, trail: Option<String>) -> Result<IrcCommand, ParsingError> {
        let message = trail.ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Quit { message })
    }
}

impl Display for IrcCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrcCommand::Privmsg { target, message } => {
                write!(f, "{PRIVMSG} {target} :{message}")
            }
            IrcCommand::Pass { password } => {
                write!(f, "{PASS} {password}")
            }
            IrcCommand::Nick { nickname } => {
                write!(f, "{NICK} {nickname}")
            }
            IrcCommand::User { username, realname } => {
                write!(f, "{USER} {username} :{realname}")
            }
            IrcCommand::Quit { message } => {
                write!(f, "{QUIT} :{message}")
            }
        }
    }
}
