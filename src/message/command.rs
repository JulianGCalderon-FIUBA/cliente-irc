use std::fmt::Display;

use super::Error;

pub enum IrcCommand {}

impl IrcCommand {
    pub fn is_valid_command(command: &str) -> bool {
        todo!()
    }

    pub fn new(
        command: String,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> Result<Self, Error> {
        todo!()
    }
}

impl Display for IrcCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
