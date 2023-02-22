mod parsing;
mod parsing_error;

#[cfg(test)]
mod tests;

pub use parsing_error::ParsingError;

pub struct Message {
    prefix: Option<String>,
    command: String,
    parameters: Vec<String>,
    trailing: Option<String>,
}

const PREFIX_CHARACTER: u8 = b':';
const MAX_LENGTH: usize = 510;
const INVALID_CHARACTERS: [char; 3] = ['\r', '\n', '\0'];

impl Message {
    pub fn new(content: &str) -> Result<Self, ParsingError> {
        let (prefix, command, parameters, trailing) = parsing::parse(content)?;

        Ok(Self {
            prefix,
            command,
            parameters,
            trailing,
        })
    }

    pub fn _unpack(self) -> (Option<String>, String, Vec<String>, Option<String>) {
        (self.prefix, self.command, self.parameters, self.trailing)
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, ":{prefix} ")?;
        }

        write!(f, "{}", self.command)?;

        for parameter in self.parameters.iter() {
            write!(f, " {parameter}")?;
        }

        if let Some(trailing) = &self.trailing {
            write!(f, " :{trailing}")
        } else {
            Ok(())
        }
    }
}
