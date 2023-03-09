//! IRC response messages.
//!
//! This module contains the type and functions to parse and serialize IRC response messages.
//!
//! The responses are defined according to the protocol RFC 1459

use super::{Args, ParsingError, Trail};

const WELCOME: &str = "001";
const NICK_COLLISION: &str = "436";
const NO_NICKNAME: &str = "200";

/// IRC response
///
/// This enum represents an IRC response. It contains all the possible responses.
pub enum IrcResponse {
    /// Welcome response
    ///
    /// This variant represents the welcome response, containing the important information about the
    /// user.
    Welcome {
        /// The nickname of the user
        nickname: String,
        /// The realname of the user
        realname: String,
        /// The username of the user
        username: String,
        /// The hostname of the user
        hostname: String,
        /// The servername of the user
        servername: String,
    },
    /// Nickname collision response
    ///
    /// This variant represents the nickname collision response
    NickCollision {
        /// The nickname that caused the collision
        nickname: String,
    },
    /// No nickname response
    ///
    /// This variant represents the no nickname response
    ///
    /// Although this response is not defined in the RFC, it is used by some IRC servers
    /// to indicate that a USER command was sent before a NICK command.
    NoNickname,
}

impl IrcResponse {
    /// Creates a new response
    ///
    /// This function creates a new response from the given parameters.
    ///
    /// # Errors
    ///
    /// This function returns an error if the response or the parameters are not valid.
    pub fn new(response: String, arguments: Args, trailing: Trail) -> Result<Self, ParsingError> {
        match &response[..] {
            WELCOME => Self::new_welcome(arguments, trailing),
            NICK_COLLISION => Self::new_nick_collision(arguments, trailing),
            NO_NICKNAME => Self::new_no_nickname(arguments, trailing),
            _ => Err(ParsingError::UnknownCommand(response)),
        }
    }

    /// Checks if the given string is a valid response
    pub fn is_response(response: &str) -> bool {
        response == WELCOME || response == NICK_COLLISION || response == NO_NICKNAME
    }

    /// Returns a constructed Welcome response from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    fn new_welcome(mut args: Args, trail: Trail) -> Result<Self, ParsingError> {
        let realname = args.pop().ok_or(ParsingError::MissingParameter)?;

        let trailing = trail.ok_or(ParsingError::MissingParameter)?;
        let mut split_trailing = trailing.split_whitespace().map(str::to_owned);

        let servername = split_trailing
            .nth(2)
            .ok_or(ParsingError::MissingParameter)?;
        let nickname = split_trailing
            .nth(1)
            .ok_or(ParsingError::MissingParameter)?;
        let mut username = split_trailing
            .next()
            .ok_or(ParsingError::MissingParameter)?;
        let mut hostname = split_trailing
            .next()
            .ok_or(ParsingError::MissingParameter)?;

        if username.is_empty() || hostname.is_empty() {
            return Err(ParsingError::InvalidParameter);
        }

        username.remove(0);
        hostname.remove(0);

        Ok(Self::Welcome {
            realname,
            servername,
            nickname,
            username,
            hostname,
        })
    }

    /// Returns a constructed NickCollision response from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    fn new_nick_collision(mut args: Args, _trail: Trail) -> Result<Self, ParsingError> {
        let nickname = args.pop().ok_or(ParsingError::MissingParameter)?;

        Ok(Self::NickCollision { nickname })
    }

    /// Returns a constructed NoNickname response from the given parameters
    ///
    /// # Errors
    ///
    /// This function returns an error if the parameters are not valid.
    fn new_no_nickname(_args: Args, _trail: Trail) -> Result<IrcResponse, ParsingError> {
        Ok(Self::NoNickname)
    }
}
