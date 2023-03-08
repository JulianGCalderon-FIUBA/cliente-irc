//! This module define [`IrcResponse`]

use super::{Args, ParsingError, Trail};

const WELCOME: &str = "001";
const NICK_COLLISION: &str = "436";
const NO_NICKNAME: &str = "200";

/// Messages sent by the server in response to a client's command
pub enum IrcResponse {
    Welcome {
        nickname: String,
        realname: String,
        username: String,
        hostname: String,
        servername: String,
    },
    NickCollision {
        nickname: String,
    },
    NoNickname,
}

impl IrcResponse {
    /// Creates the corresponding variation from the given parameters,
    ///
    /// Fails on invalid `response` or on an invalid argument
    pub fn new(response: String, arguments: Args, trailing: Trail) -> Result<Self, ParsingError> {
        match &response[..] {
            WELCOME => Self::new_welcome(arguments, trailing),
            NICK_COLLISION => Self::new_nick_collision(arguments, trailing),
            NO_NICKNAME => Self::new_no_nickname(arguments, trailing),
            _ => Err(ParsingError::UnknownCommand(response)),
        }
    }

    pub fn is_response(response: &str) -> bool {
        response == WELCOME || response == NICK_COLLISION || response == NO_NICKNAME
    }

    pub fn new_welcome(mut args: Args, trail: Trail) -> Result<Self, ParsingError> {
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

    fn new_nick_collision(mut args: Args, _trail: Trail) -> Result<Self, ParsingError> {
        let nickname = args.pop().ok_or(ParsingError::MissingParameter)?;

        Ok(Self::NickCollision { nickname })
    }

    fn new_no_nickname(_args: Args, _trail: Trail) -> Result<IrcResponse, ParsingError> {
        Ok(Self::NoNickname)
    }
}
