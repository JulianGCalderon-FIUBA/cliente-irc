use std::ops::Deref;

pub enum ChatProperty {
    Name,
}

impl Deref for ChatProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            ChatProperty::Name => "name",
        }
    }
}

impl From<&str> for ChatProperty {
    fn from(value: &str) -> Self {
        match value {
            "name" => Self::Name,
            _ => unimplemented!(),
        }
    }
}
