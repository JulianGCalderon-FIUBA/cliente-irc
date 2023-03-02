use super::ParsingError;

const WELCOME: &str = "001";
const NICK_COLLISION: &str = "436";

pub enum IrcResponse {
    Welcome {
        realname: String,
        servername: String,
        nickname: String,
        username: String,
        hostname: String,
    },
    NickCollision {
        nickname: String,
    },
}

type Trail = Option<String>;
type Args = Vec<String>;

impl IrcResponse {
    pub fn new(response: String, arguments: Args, trailing: Trail) -> Result<Self, ParsingError> {
        match &response[..] {
            WELCOME => Self::new_welcome(arguments, trailing),
            NICK_COLLISION => Self::new_nick_collision(arguments, trailing),
            _ => Err(ParsingError::UnknownCommand(response)),
        }
    }

    pub fn is_response(response: &str) -> bool {
        response == WELCOME || response == NICK_COLLISION
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
}
