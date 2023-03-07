//! Defines useful constants for dealing with `PageRow`

use std::ops::Deref;

use glib::ParamSpec;
use gtk::glib::{self, ParamSpecString};

/// All `message` custom properties
///
/// Can be converted between &str to use as property name
pub enum PageRowProperty {
    /// Stack associated to the sidebar
    /// Type: [´gtk::Stack´]
    Name,
    Icon,
}

impl PageRowProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![
            ParamSpecString::builder(&PageRowProperty::Name).build(),
            ParamSpecString::builder(&PageRowProperty::Icon).build(),
        ]
    }
}

impl Deref for PageRowProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Name => "name",
            Self::Icon => "icon",
        }
    }
}

impl From<&str> for PageRowProperty {
    fn from(value: &str) -> Self {
        match value {
            "name" => Self::Name,
            "icon" => Self::Icon,
            _ => unimplemented!(),
        }
    }
}
