use std::ops::Deref;

use gtk::glib::{ParamSpec, ParamSpecString};

pub enum MessageProperty {
    Message,
}

impl MessageProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![ParamSpecString::builder(&MessageProperty::Message).build()]
    }
}

impl Deref for MessageProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Message => "message",
        }
    }
}

impl From<&str> for MessageProperty {
    fn from(value: &str) -> Self {
        match value {
            "message" => Self::Message,
            _ => unimplemented!(),
        }
    }
}
