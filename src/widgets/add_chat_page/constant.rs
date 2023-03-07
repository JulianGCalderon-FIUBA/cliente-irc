//! This module contains useful constants for dealing with `AddChatPage`

use gtk::glib::subclass::Signal;
use gtk::prelude::StaticType;
use std::ops::Deref;

pub enum AddChatPageSignal {
    /// Emitted after user requests to add new chat
    ///
    /// Arguments:
    /// - Name
    Add,
}

impl AddChatPageSignal {
    pub fn vec() -> Vec<Signal> {
        vec![Signal::builder(&Self::Add)
            .param_types([String::static_type()])
            .build()]
    }
}

impl Deref for AddChatPageSignal {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Add => "add",
        }
    }
}

impl From<&str> for AddChatPageSignal {
    fn from(value: &str) -> Self {
        match value {
            "add" => Self::Add,
            _ => unimplemented!(),
        }
    }
}
