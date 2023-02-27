#[derive(Debug)]
pub enum Error {
    EmptyMessage,
    EmptyPrefix,
    NoCommand,
    InvalidCharacter,
    TooManyParameters,
    MissingParameter,
    InvalidParameter,
    UnknownCommand(String),
}
