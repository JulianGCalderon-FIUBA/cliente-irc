mod constant;
mod imp;

use glib::Object;
use gtk::{glib, prelude::ObjectExt};

pub use constant::PasswordFieldProperty;

glib::wrapper! {
    pub struct PasswordField(ObjectSubclass<imp::PasswordField>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl PasswordField {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn input(&self) -> String {
        let input: String = self.property(&PasswordFieldProperty::Input);

        input
    }
}

impl Default for PasswordField {
    fn default() -> Self {
        Self::new()
    }
}
