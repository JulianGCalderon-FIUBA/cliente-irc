/// Parsing errors that can be encountered while parsing [IrcMessage]
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
