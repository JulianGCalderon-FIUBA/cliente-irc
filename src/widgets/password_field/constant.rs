//! This module contains useful constants for dealing with `PasswordField`

use std::ops::Deref;

/// All `PasswordField` custom properties
///
/// Can be easily converted between `&str` to use with properties
pub enum PasswordFieldProperty {
    /// Name of the field
    ///
    /// Type: String
    Name,
    /// User provided input
    ///
    /// Type: String
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
