use std::ops::Deref;

use gtk::glib::{ParamSpec, ParamSpecObject};

use crate::client::UserData;

pub enum UserPageProperty {
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
