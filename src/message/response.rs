use super::Error;

pub enum IrcResponse {}

impl IrcResponse {
    pub fn new(
        command: String,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> Result<Self, Error> {
        todo!()
    }

    pub fn is_valid_response(command: &str) -> bool {
        todo!()
    }
}
