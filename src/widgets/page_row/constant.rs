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
    Title,
    Icon,
    Name,
}

impl PageRowProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![
            ParamSpecString::builder(&PageRowProperty::Title).build(),
            ParamSpecString::builder(&PageRowProperty::Icon).build(),
            ParamSpecString::builder(&PageRowProperty::Name).build(),
        ]
    }
}

impl Deref for PageRowProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Title => "title",
            Self::Icon => "icon",
            Self::Name => "name",
        }
    }
}

impl From<&str> for PageRowProperty {
    fn from(value: &str) -> Self {
        match value {
            "title" => Self::Title,
            "icon" => Self::Icon,
            "name" => Self::Name,
            _ => unimplemented!(),
        }
    }
}
