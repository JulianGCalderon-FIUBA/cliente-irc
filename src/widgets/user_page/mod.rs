//! This module defines all [`UserPage`] related structures

mod constant;
mod imp;

pub use constant::UserPageProperty;
use glib::Object;
use gtk::glib;

use crate::client::UserData;

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
