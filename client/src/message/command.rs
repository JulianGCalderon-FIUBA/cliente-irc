//! IRC commands
//!
//! This module contains the definition of the IRC commands.
//!
//! The commands are defined according to the protocol RFC 1459

use std::fmt::Display;

use super::{Args, ParsingError, Trail};

const PRIVMSG: &str = "PRIVMSG";
const PASS: &str = "PASS";
const NICK: &str = "NICK";
const USER: &str = "USER";
const QUIT: &str = "QUIT";
const JOIN: &str = "JOIN";

/// IRC command
///
/// This enum represents an IRC command. It contains all the possible commands.
#[derive(Debug)]
pub enum IrcCommand {
    /// This variant represents the privmsg command
    Privmsg {
        /// The target of the message
        target: String,
        /// The message itself
        message: String,
    },
    /// This variant represents the pass command
    Pass {
        /// The password provided by the user
        password: String,
    },
    /// This variant represents the nick command
    Nick {
        /// The nickname provided by the user
        nickname: String,
    },
    /// This variant represents the user command
    User {
        /// The username provided by the user
        username: String,
        /// The realname provided by the user
        realname: String,
    },
    /// This variant represents the quit command
    Quit {
        /// The quit message provided by the user
        message: String,
    },
    /// This variant represents the join command
    Join {
        /// The channel to join
        name: String,
    },
}

impl IrcCommand {
    /// Creates a new command
    ///
    /// This function creates a new command from the given parameters.
    ///
    /// # Errors
    ///
    /// This function returns an error if the command or the parameters are not valid.
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

    /// Checks if the given string is a valid command
    pub fn is_command(command: &str) -> bool {
        command == PRIVMSG
            || command == PASS
            || command == NICK
            || command == USER
            || command == QUIT
            || command == JOIN
    }

    /// Returns a constructed Privmsg command from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    pub fn new_privmsg(mut args: Args, trail: Trail) -> Result<Self, ParsingError> {
        args.reverse();

        let message = trail.ok_or(ParsingError::MissingParameter)?;
        let target = args.pop().ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Privmsg { target, message })
    }

    /// Returns a constructed Pass command from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    fn new_pass(mut args: Args, _trail: Trail) -> Result<IrcCommand, ParsingError> {
        let password = args.pop().ok_or(ParsingError::MissingParameter)?;
        Ok(Self::Pass { password })
    }

    /// Returns a constructed Nick command from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    fn new_nick(mut args: Args, _trail: Trail) -> Result<IrcCommand, ParsingError> {
        let nickname = args.pop().ok_or(ParsingError::MissingParameter)?;
        Ok(Self::Nick { nickname })
    }

    /// Returns a constructed User command from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    fn new_user(mut args: Args, trail: Trail) -> Result<IrcCommand, ParsingError> {
        let username = args.pop().ok_or(ParsingError::MissingParameter)?;
        let realname = trail.ok_or(ParsingError::MissingParameter)?;

        Ok(Self::User { username, realname })
    }

    /// Returns a constructed Quit command from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    fn new_quit(_args: Args, trail: Trail) -> Result<IrcCommand, ParsingError> {
        let message = trail.ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Quit { message })
    }

    /// Returns a constructed Join command from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    fn new_join(mut args: Args, _trail: Trail) -> Result<IrcCommand, ParsingError> {
        let name = args.pop().ok_or(ParsingError::MissingParameter)?;

        Ok(Self::Join { name })
    }
}

/// Converts the command into a string as defined by the protocol
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
