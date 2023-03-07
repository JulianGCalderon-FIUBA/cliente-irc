//! This module define [`IrcCommand`]

use std::fmt::Display;

use super::{Args, ParsingError, Trail};

const PRIVMSG: &str = "PRIVMSG";
const PASS: &str = "PASS";
const NICK: &str = "NICK";
const USER: &str = "USER";
const QUIT: &str = "QUIT";
const JOIN: &str = "JOIN";

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
    Join { name: String },
}

impl IrcCommand {
    /// Creates the corresponding variation from the given components,
    ///
    /// Fails on invalid `response` or on an invalid argument
    pub fn new(command: String, args: Args, trail: Trail) -> Result<Self, ParsingError> {
        match &command[..] {
            PRIVMSG => Self::new_privmsg(args, trail),
            PASS => Self::new_pass(args, trail),
            NICK => Self::new_nick(args, trail),
            USER => Self::new_user(args, trail),
            QUIT => Self::new_quit(args, trail),
            JOIN => Self::new_join(args, trail),
            _ => Err(ParsingError::UnknownCommand(command)),
        }
    }

    pub fn is_command(command: &str) -> bool {
        command == PRIVMSG
            || command == PASS
            || command == NICK
            || command == USER
            || command == QUIT
            || command == JOIN
    }

    /// Creates a [`IrcCommand::Privmsg`] from the given components.
    ///
    /// Fails on invalid arguments
    pub fn new_privmsg(mut args: Args, trail: Trail) -> Result<Self, ParsingError> {
        args.reverse();

        let message = trail.ok_or(ParsingError::MissingParameter)?;
        let target = args.pop().ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Privmsg { target, message })
    }

    /// Creates a [`IrcCommand::Pass`] from the given components.
    ///
    /// Fails on invalid arguments
    fn new_pass(mut args: Args, _trail: Trail) -> Result<IrcCommand, ParsingError> {
        let password = args.pop().ok_or(ParsingError::MissingParameter)?;
        Ok(Self::Pass { password })
    }

    /// Creates a [`IrcCommand::Nick`] from the given components.
    ///
    /// Fails on invalid arguments
    fn new_nick(mut args: Args, _trail: Trail) -> Result<IrcCommand, ParsingError> {
        let nickname = args.pop().ok_or(ParsingError::MissingParameter)?;
        Ok(Self::Nick { nickname })
    }

    /// Creates a [`IrcCommand::User`] from the given components.
    ///
    /// Fails on invalid arguments
    fn new_user(mut args: Args, trail: Trail) -> Result<IrcCommand, ParsingError> {
        let username = args.pop().ok_or(ParsingError::MissingParameter)?;
        let realname = trail.ok_or(ParsingError::MissingParameter)?;

        Ok(Self::User { username, realname })
    }

    /// Creates a [ IrcCommand::Quit`] from the given components.
    ///
    /// Fails on invalid arguments
    fn new_quit(_args: Args, trail: Trail) -> Result<IrcCommand, ParsingError> {
        let message = trail.ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Quit { message })
    }

    /// Creates a [ IrcCommand::Join`] from the given components.
    ///
    /// Fails on invalid arguments
    fn new_join(mut args: Args, _trail: Trail) -> Result<IrcCommand, ParsingError> {
        let name = args.pop().ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Join { name })
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
            IrcCommand::Join { name } => {
                write!(f, "{JOIN} {name}")
            }
        }
    }
}
