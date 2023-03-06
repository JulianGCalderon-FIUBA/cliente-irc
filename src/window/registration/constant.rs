use std::ops::Deref;

use gtk::{glib::subclass::Signal, prelude::StaticType};

use crate::client::{ClientData, IrcClient};

use super::Registration;

pub enum RegistrationSignal {
    Registered,
}

impl RegistrationSignal {
    pub fn vec() -> Vec<Signal> {
        vec![Signal::builder(&Self::Registered)
            .param_types([
                Registration::static_type(),
                IrcClient::static_type(),
                ClientData::static_type(),
            ])
            .build()]
    }
}

impl Deref for RegistrationSignal {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Registered => "registered",
        }
    }
}

impl From<&str> for RegistrationSignal {
    fn from(value: &str) -> Self {
        match value {
            "register" => Self::Registered,
            _ => unimplemented!(),
        }
    }
}
