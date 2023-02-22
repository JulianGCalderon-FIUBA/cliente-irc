mod error;
mod parser;

#[cfg(test)]
mod tests;

use std::fmt::Display;

pub use error::Error;

use self::parser::{Parameters, Prefix, Trailing};

pub enum IrcMessage {
    Welcome {
        realname: String,
        servername: String,
        nickname: String,
        username: String,
        hostname: String,
    },
    Quit {
        message: String,
    },
}

const WELCOME_COMMAND: &str = "001";
const QUIT_COMMAND: &str = "QUIT";

impl IrcMessage {
    pub fn new(content: &str) -> Result<Self, Error> {
        let (prefix, command, parameters, trailing) = parser::parse(content)?;

        match &command[..] {
            WELCOME_COMMAND => Self::new_welcome(prefix, parameters, trailing),
            QUIT_COMMAND => Self::new_quit(prefix, parameters, trailing),
            _ => Err(Error::UnknownCommand(content.to_string())),
        }
    }

    fn new_welcome(
        _prefix: Prefix,
        mut parameters: Parameters,
        trailing: Trailing,
    ) -> Result<Self, Error> {
        let realname = parameters.pop().ok_or(Error::MissingParameter)?;

        let trailing = trailing.ok_or(Error::MissingParameter)?;
        let mut split_trailing = trailing.split_whitespace().map(str::to_owned);

        let servername = split_trailing.nth(2).ok_or(Error::MissingParameter)?;
        let nickname = split_trailing.nth(1).ok_or(Error::MissingParameter)?;
        let mut username = split_trailing.next().ok_or(Error::MissingParameter)?;
        let mut hostname = split_trailing.next().ok_or(Error::MissingParameter)?;

        if username.is_empty() || hostname.is_empty() {
            return Err(Error::InvalidParameter);
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

    fn new_quit(
        _prefix: Prefix,
        _parameters: Parameters,
        trailing: Trailing,
    ) -> Result<Self, Error> {
        let message = trailing.ok_or(Error::MissingParameter)?;

        Ok(Self::Quit { message })
    }
}

impl Display for IrcMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Quit { message } => {
                format!("{QUIT_COMMAND} :{message}")
            }

            Self::Welcome {
                realname,
                servername,
                nickname,
                username,
                hostname,
            } => {
                format!(
                    "{WELCOME_COMMAND} {realname} :Welcome to {servername} Network, {nickname} !{username} @{hostname}"
                )
            }
        };
        write!(f, "{string}")
    }
}
