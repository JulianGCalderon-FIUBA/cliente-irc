use std::ops::Deref;

pub enum FieldProperty {
    Name,
    Input,
    Default,
    Password,
    Locked,
    Error,
}

impl Deref for FieldProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            FieldProperty::Name => "name",
            FieldProperty::Input => "input",
            FieldProperty::Default => "default",
            FieldProperty::Password => "password",
            FieldProperty::Locked => "locked",
            FieldProperty::Error => "error",
        }
    }
}

impl From<&str> for FieldProperty {
    fn from(value: &str) -> Self {
        match value {
            "name" => FieldProperty::Name,
            "input" => FieldProperty::Input,
            "default" => FieldProperty::Default,
            "password" => FieldProperty::Password,
            "locked" => FieldProperty::Locked,
            "error" => FieldProperty::Error,
            _ => unimplemented!(),
        }
    }
}
