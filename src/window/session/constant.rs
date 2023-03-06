use std::ops::Deref;

use gtk::glib::{ParamSpec, ParamSpecObject};

use crate::client::UserData;

pub enum SessionProperty {
    Data,
}

impl SessionProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![ParamSpecObject::builder::<UserData>(&SessionProperty::Data).build()]
    }
}

impl Deref for SessionProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Data => "data",
        }
    }
}

impl From<&str> for SessionProperty {
    fn from(value: &str) -> Self {
        match value {
            "data" => Self::Data,
            _ => unimplemented!(),
        }
    }
}
