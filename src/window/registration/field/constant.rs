use std::ops::Deref;

pub enum FieldProperty {
    Name,
    Input,
    Default,
    Password,
    Locked,
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
            _ => unimplemented!(),
        }
    }
}
