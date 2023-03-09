//! Defines the [`Account`] page

mod imp;

use glib::Object;
use gtk::glib;

use crate::gtk_client::RegistrationDataObject;

glib::wrapper! {
    /// The account page is used to display the user's information
    ///
    /// Subclassifies `gtk::Box`
    ///
    /// # Properties
    ///
    /// * `registration-data` - The user registration data
    ///     - Type: `RegistrationDataObject`
    pub struct Account(ObjectSubclass<imp::Account>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Account {
    /// Creates a new account page with the given registration data
    pub fn new(data: RegistrationDataObject) -> Self {
        Object::builder()
            .property("registration-data", data)
            .build()
    }
}
