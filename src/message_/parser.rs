use super::Error;

const PREFIX_CHARACTER: u8 = b':';
const MAX_LENGTH: usize = 510;
const INVALID_CHARACTERS: [char; 3] = ['\r', '\n', '\0'];

use std::iter::Peekable;
use std::str::SplitWhitespace;

pub type Prefix = Option<String>;
pub type Command = String;
pub type Parameters = Vec<String>;
pub type Trailing = Option<String>;
type IrcMessageParse = (Prefix, Command, Parameters, Trailing);

/// Parses string into prefix, command, parameters and trailing
pub fn parse(content: &str) -> Result<IrcMessageParse, Error> {
    if content.is_empty() {
        return Err(Error::EmptyMessage);
    }
    if content.len() > MAX_LENGTH {
        return Err(Error::TooManyParameters);
    }
    if content.contains(INVALID_CHARACTERS) {
        return Err(Error::InvalidCharacter);
    }

    let mut words = content.split_whitespace().peekable();

    let prefix = get_prefix(&mut words)?;
    let command = get_command(&mut words)?;
    let parameters = get_parameters(&mut words)?;
    let trailing = get_trailing(&mut words)?;

    Ok((prefix, command, parameters, trailing))
}

/// If next iter item is a prefix, it consumes it and returns its value
fn get_prefix(split: &mut Peekable<SplitWhitespace>) -> Result<Prefix, Error> {
    let possible_prefix = match split.peek() {
        None => return Err(Error::EmptyMessage),
        Some(possible_prefix) => possible_prefix,
    };

    let first_character = *possible_prefix
        .as_bytes()
        .first()
        .expect("SplitWhitespace does not generate empty elements");

    if first_character == PREFIX_CHARACTER {
        let prefix = split.next().expect("Existance was verified on peek");

        if prefix.len() == 1 {
            return Err(Error::EmptyPrefix);
        }

        let prefix = &prefix[1..];

        return Ok(Some(prefix.to_string()));
    }

    Ok(None)
}

/// If next iter item is a command, it consumes it and returns its value
fn get_command(split: &mut Peekable<SplitWhitespace>) -> Result<Command, Error> {
    let possible_command = match split.next() {
        None => return Err(Error::NoCommand),
        Some(possible_command) => possible_command,
    };

    Ok(possible_command.to_string())
}

/// Consumes parameters from iterator and returns them
fn get_parameters(split: &mut Peekable<SplitWhitespace>) -> Result<Parameters, Error> {
    let mut parameters = Vec::new();

    while let Some(possible_parameter) = split.peek() {
        let first_character = *possible_parameter
            .as_bytes()
            .first()
            .expect("SplitWhitespace does not generate empty elements");

        if first_character == PREFIX_CHARACTER {
            break;
        }

        let parameter = split.next().expect("Existance was verified on peek");
        parameters.push(parameter.to_string());
    }

    if parameters.len() > 15 {
        return Err(Error::TooManyParameters);
    }

    Ok(parameters)
}

/// If next iter item is a trailing parameter, it consumes it and returns its value
fn get_trailing(split: &mut Peekable<SplitWhitespace>) -> Result<Trailing, Error> {
    if split.peek().is_none() {
        return Ok(None);
    }

    let string_list: Vec<String> = split.map(|word| word.to_string()).collect();
    let mut joined_string = string_list.join(" ");

    joined_string.remove(0);

    Ok(Some(joined_string))
}
