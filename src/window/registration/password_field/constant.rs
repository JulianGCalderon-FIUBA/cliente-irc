use std::ops::Deref;

pub enum PasswordFieldProperty {
    Name,
    Input,
}

impl Deref for PasswordFieldProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            PasswordFieldProperty::Name => "name",
            PasswordFieldProperty::Input => "input",
        }
    }
}

impl From<&str> for PasswordFieldProperty {
    fn from(value: &str) -> Self {
        match value {
            "name" => PasswordFieldProperty::Name,
            "input" => PasswordFieldProperty::Input,

            _ => unimplemented!(),
        }
    }
}
