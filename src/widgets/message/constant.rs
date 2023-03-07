//! Defines useful constants for dealing with [`Message`]

use std::ops::Deref;

use gtk::glib::{ParamSpec, ParamSpecString};

/// All `message` custom properties
///
/// Can be converted between &str to use as property name
pub enum MessageProperty {
    /// Text to be displayed as the message
    /// Type: String
    Message,
    /// Sender of the message, only displayed if not empty
    /// Type: String
    Sender,
}

impl MessageProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![
            ParamSpecString::builder(&MessageProperty::Message).build(),
            ParamSpecString::builder(&MessageProperty::Sender).build(),
        ]
    }
}

impl Deref for MessageProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Sender => "sender",
            Self::Message => "message",
        }
    }
}

impl From<&str> for MessageProperty {
    fn from(value: &str) -> Self {
        match value {
            "message" => Self::Message,
            "sender" => Self::Sender,
            _ => unimplemented!(),
        }
    }
}
