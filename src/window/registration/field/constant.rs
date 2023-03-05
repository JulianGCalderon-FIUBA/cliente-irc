use std::ops::Deref;

pub enum FieldProperty {
    Name,
    Input,
    Default,
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
            FieldProperty::Error => "error",
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
            "locked" => FieldProperty::Locked,
            "error" => FieldProperty::Error,
            _ => unimplemented!(),
        }
    }
}
