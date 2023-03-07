//! This module defines usefull constants for dealing with `UserData`

use std::ops::Deref;

use gtk::glib::{ParamSpec, ParamSpecString};

/// All `UserData` custom properties
///
/// Can be easily converted between `&str` to access properties
pub enum UserDataProperty {
    /// Type: String
    Nickname,
    /// Type: String
    Realname,
    /// Type: String
    Username,
    /// Type: String
    Hostname,
    /// Type: String
    Servername,
}

impl UserDataProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![
            ParamSpecString::builder(&Self::Nickname).build(),
            ParamSpecString::builder(&Self::Realname).build(),
            ParamSpecString::builder(&Self::Username).build(),
            ParamSpecString::builder(&Self::Hostname).build(),
            ParamSpecString::builder(&Self::Servername).build(),
        ]
    }
}

impl Deref for UserDataProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Nickname => "nickname",
            Self::Realname => "realname",
            Self::Username => "username",
            Self::Hostname => "hostname",
            Self::Servername => "servername",
        }
    }
}

impl From<&str> for UserDataProperty {
    fn from(value: &str) -> Self {
        match value {
            "nickname" => Self::Nickname,
            "realname" => Self::Realname,
            "username" => Self::Username,
            "hostname" => Self::Hostname,
            "servername" => Self::Servername,
            _ => unimplemented!(),
        }
    }
}
