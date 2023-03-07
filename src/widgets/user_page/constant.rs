//! This module contains useful constants for dealing with `UserPage`

use std::ops::Deref;

use gtk::glib::{ParamSpec, ParamSpecObject};

use crate::client::UserData;

/// All `UserPage` custom properties
///
/// Can be easly converted to &str to use as property names.
pub enum UserPageProperty {
    /// User registration data
    /// Type: UserData
    Data,
}

impl UserPageProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![ParamSpecObject::builder::<UserData>(&Self::Data).build()]
    }
}

impl Deref for UserPageProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Data => "data",
        }
    }
}

impl From<&str> for UserPageProperty {
    fn from(value: &str) -> Self {
        match value {
            "data" => Self::Data,
            _ => unimplemented!(),
        }
    }
}
