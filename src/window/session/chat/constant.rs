//! This module contains useful constants for `Chat`

use std::ops::Deref;

use gtk::{
    glib::{subclass::Signal, ParamSpec, ParamSpecString},
    prelude::StaticType,
};

/// All `Chat` custom properties
///
/// Can be converted between &str to use as property names
pub enum ChatProperty {
    Name,
}

impl ChatProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![ParamSpecString::builder(&ChatProperty::Name).build()]
    }
}

impl Deref for ChatProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Name => "name",
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

/// All `chat`custom signals
///
/// Can be converted between &str to use as signal names
pub enum ChatSignal {
    /// Emited when the user atempts to send a message through the chat
    ///
    /// Argument: Message (String)
    Send,
    /// Emited when the user atempts to close the chat
    Close,
}

impl ChatSignal {
    pub fn vec() -> Vec<Signal> {
        vec![
            Signal::builder(&Self::Send)
                .param_types([String::static_type()])
                .build(),
            Signal::builder(&Self::Close).build(),
        ]
    }
}

impl Deref for ChatSignal {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Send => "send",
            Self::Close => "close",
        }
    }
}

impl From<&str> for ChatSignal {
    fn from(value: &str) -> Self {
        match value {
            "send" => Self::Send,
            "close" => Self::Close,
            _ => unimplemented!(),
        }
    }
}
