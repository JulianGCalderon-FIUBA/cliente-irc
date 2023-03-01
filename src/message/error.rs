#[derive(Debug)]
pub enum ParsingError {
    EmptyMessage,
    EmptyPrefix,
    MissingPrefix,
    NoCommand,
    InvalidCharacter,
    TooManyParameters,
    MissingParameter,
    InvalidParameter,
    UnknownCommand(String),
}
