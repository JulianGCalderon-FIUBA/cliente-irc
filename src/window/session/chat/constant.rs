use std::ops::Deref;

use gtk::{
    glib::{subclass::Signal, ParamSpec, ParamSpecString},
    prelude::StaticType,
};

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

pub enum ChatSignal {
    Send,
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
