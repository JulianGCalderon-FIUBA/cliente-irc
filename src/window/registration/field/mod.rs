mod imp;
mod property;

use glib::Object;
use gtk::{glib, prelude::ObjectExt};

pub use property::FieldProperty;

glib::wrapper! {
    pub struct Field(ObjectSubclass<imp::Field>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Field {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn input(&self) -> String {
        let input: String = self.property(&FieldProperty::Input);

        if input.is_empty() {
            return self.property(&FieldProperty::Default);
        }

        input
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
