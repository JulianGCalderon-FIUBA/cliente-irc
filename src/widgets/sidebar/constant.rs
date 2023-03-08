//! Defines useful constants for dealing with `Sidebar`

use std::ops::Deref;

use glib::ParamSpec;
use gtk::glib::{self, ParamSpecObject};
use gtk::Stack;

/// All `message` custom properties
///
/// Can be converted between &str to use as property name
pub enum SidebarProperty {
    /// Stack associated to the sidebar
    /// Type: [´gtk::Stack´]
    Stack,
}

impl SidebarProperty {
    pub fn vec() -> Vec<ParamSpec> {
        vec![ParamSpecObject::builder::<Stack>(&SidebarProperty::Stack).build()]
    }
}

impl Deref for SidebarProperty {
    type Target = str;
    fn deref(&self) -> &str {
        match self {
            Self::Stack => "stack",
        }
    }
}

impl From<&str> for SidebarProperty {
    fn from(value: &str) -> Self {
        match value {
            "stack" => Self::Stack,
            _ => unimplemented!(),
        }
    }
}
