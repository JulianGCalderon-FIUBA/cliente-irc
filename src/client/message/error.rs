//! This module defines [`ParsingError`]

/// Parsing errors that can be encountered while parsing a server message
#[derive(Debug)]
pub enum ParsingError {
    EmptyMessage,
    EmptyPrefix,
    MissingPrefix,
    NoCommand,
    TooManyParameters,
    MissingParameter,
    InvalidParameter,
    UnknownCommand(String),
}
