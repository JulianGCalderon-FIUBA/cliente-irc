/// Defines useful constanta for dealing with `Field`
use std::ops::Deref;

/// All custom defined propeties for `Field`
///
/// Can be easily converted between `&str` to use with properties
pub enum FieldProperty {
    /// Name of the field
    ///
    /// Type: `String`
    Name,
    /// User provided input
    ///
    /// Type: `String`
    Input,
    /// Default value to be used if input was not provided
    ///
    /// Type: `String`
    Default,
    /// Wether the input is locked or not
    ///
    /// Type: `bool`
    Locked,
    /// Error message displayed to the user.
    /// Can be deactivated with an empty String
    ///
    /// Type: `String`
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
