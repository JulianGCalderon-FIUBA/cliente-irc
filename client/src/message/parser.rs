//! This module contains the parser for IRC messages.
//!
//! The parser is based on the RFC 1459 specification.

const PREFIX_CHARACTER: u8 = b':';
const MAX_LENGTH: usize = 510;

use std::iter::Peekable;
use std::str::SplitWhitespace;

use super::ParsingError;

pub type Prefix = Option<String>;
pub type Command = String;
pub type Parameters = Vec<String>;
pub type Trailing = Option<String>;
type IrcMessageParse = (Prefix, Command, Parameters, Trailing);

/// Parses an IRC message
///
/// This function parses an IRC message and returns a tuple containing the prefix, the command, the
/// parameters and the trailing.
///
/// # Errors
///
/// This function returns an error if the message is not a valid IRC message.
pub fn parse(content: &str) -> Result<IrcMessageParse, ParsingError> {
    if content.is_empty() {
        return Err(ParsingError::EmptyMessage);
    }
    if content.len() > MAX_LENGTH {
        return Err(ParsingError::TooManyParameters);
    }

    let mut words = content.split_whitespace().peekable();

    let prefix = get_prefix(&mut words)?;
    let command = get_command(&mut words)?;
    let parameters = get_parameters(&mut words)?;
    let trailing = get_trailing(&mut words)?;

    Ok((prefix, command, parameters, trailing))
}

/// Parses the prefix of an IRC message
///
/// If the next item in the iterator is a prefix, it is returned. Otherwise, None is returned.
///
/// # Errors
///
/// This function returns an error if the prefix is empty
fn get_prefix(split: &mut Peekable<SplitWhitespace>) -> Result<Prefix, ParsingError> {
    let possible_prefix = match split.peek() {
        None => return Err(ParsingError::EmptyMessage),
        Some(possible_prefix) => possible_prefix,
    };

    let first_character = *possible_prefix
        .as_bytes()
        .first()
        .expect("SplitWhitespace does not generate empty elements");

    if first_character == PREFIX_CHARACTER {
        let prefix = split.next().expect("Existance was verified on peek");

        if prefix.len() == 1 {
            return Err(ParsingError::EmptyPrefix);
        }

        let prefix = &prefix[1..];

        return Ok(Some(prefix.to_string()));
    }

    Ok(None)
}

/// Parses the command of an IRC message
///
/// If the next item in the iterator is a command, it is returned. Otherwise, an error is returned.
fn get_command(split: &mut Peekable<SplitWhitespace>) -> Result<Command, ParsingError> {
    let possible_command = match split.next() {
        None => return Err(ParsingError::NoCommand),
        Some(possible_command) => possible_command,
    };

    Ok(possible_command.to_string())
}

/// Parses the parameters of an IRC message
///
/// Returns a vector containing all the remaining parameters, until the trailing is found.
///
/// # Errors
///
/// This function returns an error if there are more than 15 parameters.
fn get_parameters(split: &mut Peekable<SplitWhitespace>) -> Result<Parameters, ParsingError> {
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
        return Err(ParsingError::TooManyParameters);
    }

    Ok(parameters)
}

/// Parses the trailing of an IRC message
///
/// Returns the trailing, if it exists. Otherwise, None is returned.
///
/// # Errors
///
/// This function will never return an error
fn get_trailing(split: &mut Peekable<SplitWhitespace>) -> Result<Trailing, ParsingError> {
    if split.peek().is_none() {
        return Ok(None);
    }

    let string_list: Vec<String> = split.map(|word| word.to_string()).collect();
    let mut joined_string = string_list.join(" ");

    joined_string.remove(0);

    Ok(Some(joined_string))
}
