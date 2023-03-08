//! This module contains usefull constants for using `Registration`

use std::ops::Deref;

use gtk::glib::subclass::Signal;
use gtk::prelude::StaticType;

use crate::client::{IrcClient, UserData};

use super::Registration;

/// All Registration custom signals
///
/// Can be easily converted between `&str` to use with properties
pub enum RegistrationSignal {
    /// Emitted after registration is completed
    ///
    /// Arguments:
    /// - Registration
    /// - IrcClient
    /// - UserData
    Registered,
}

impl RegistrationSignal {
    pub fn vec() -> Vec<Signal> {
        vec![Signal::builder(&Self::Registered)
            .param_types([
                Registration::static_type(),
                IrcClient::static_type(),
                UserData::static_type(),
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
