mod error;
mod parser;

#[cfg(test)]
mod tests;

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
    Privmsg {
        sender: String,
        target: String,
        message: String,
    },
}

const WELCOME_COMMAND: &str = "001";
const QUIT_COMMAND: &str = "QUIT";
const PRIVMSG_COMMAND: &str = "PRIVMSG";

impl IrcMessage {
    pub fn new(content: &str) -> Result<Self, Error> {
        let (prefix, command, parameters, trailing) = parser::parse(content)?;

        match &command[..] {
            WELCOME_COMMAND => Self::new_welcome(prefix, parameters, trailing),
            QUIT_COMMAND => Self::new_quit(prefix, parameters, trailing),
            PRIVMSG_COMMAND => Self::new_privmsg(prefix, parameters, trailing),
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

    fn new_privmsg(
        prefix: Prefix,
        mut parameters: Parameters,
        trailing: Trailing,
    ) -> Result<Self, Error> {
        parameters.reverse();

        let sender = prefix.ok_or(Error::MissingParameter)?;
        let message = trailing.ok_or(Error::MissingParameter)?;
        let target = parameters.pop().ok_or(Error::MissingParameter)?;

        Ok(Self::Privmsg {
            sender,
            target,
            message,
        })
    }
}
