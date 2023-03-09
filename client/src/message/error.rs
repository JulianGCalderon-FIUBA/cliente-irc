//! Error types for message parsing.
//!
//! This module contains the error types for message parsing.

/// Error type for message parsing.
///
/// This enum represents the different errors that can occur during message parsing.
#[derive(Debug)]
pub enum ParsingError {
    /// Empty message
    ///
    /// When an empty message is received.
    EmptyMessage,
    /// Empty prefix
    ///
    /// When the prefix is empty.
    EmptyPrefix,
    /// Missing prefix
    ///
    /// When the sender of the command is missing.
    MissingSender,
    /// Missing command
    ///
    /// When the command is missing.
    NoCommand,
    /// Too many parameters
    ///
    /// When the message contains too many parameters.
    TooManyParameters,
    /// Missing parameter
    ///
    /// When a parameter is missing.
    MissingParameter,
    /// Invalid parameter
    ///
    /// When a parameter exists, but is invalid.
    InvalidParameter,
    /// Invalid command
    ///
    /// When the command is not valid.
    ///
    /// Contains the invalid command or the entire message, depending on the implementation.
    UnknownCommand(String),
}
