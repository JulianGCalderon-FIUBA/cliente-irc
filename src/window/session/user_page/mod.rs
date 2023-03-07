//! This module defines all [`UserPage`] related structures

mod constant;
mod imp;

use glib::Object;
use gtk::glib;

use crate::client::UserData;

pub use constant::UserPageProperty;

glib::wrapper! {
    /// Page to display the user information
    /// 
    /// Subclassifies [´gtk::Box´]
    pub struct UserPage(ObjectSubclass<imp::UserPage>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl UserPage {
    /// Creates a [`UserPage`] with an associated [´UserData`]
    pub fn new(data: UserData) -> Self {
        Object::builder()
            .property(&UserPageProperty::Data, data)
            .build()
    }
}
