use super::ParsingError;

const WELCOME: &str = "001";

pub enum IrcResponse {
    Welcome {
        realname: String,
        servername: String,
        nickname: String,
        username: String,
        hostname: String,
    },

}

impl IrcResponse {
    pub fn new(
        response: String,
        arguments: Vec<String>,
        trailing: Option<String>,
    ) -> Result<Self, ParsingError> {
        match &response[..] {
            WELCOME => Self::new_welcome(arguments, trailing),
            _ => Err(ParsingError::UnknownCommand(response)),
        }
    }

    pub fn new_welcome(mut args: Vec<String>, trail: Option<String>) -> Result<Self, ParsingError> {
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
}
