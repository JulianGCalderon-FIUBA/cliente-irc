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
