mod constant;
mod imp;

use glib::Object;
use gtk::{glib, prelude::ObjectExt};

pub use constant::FieldProperty;

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

    pub fn set_input(&self, value: &str) {
        self.set_property(&FieldProperty::Input, value);
    }

    pub fn lock(&self) {
        let input: String = self.property(&FieldProperty::Input);
        if input.is_empty() {
            let default: String = self.property(&FieldProperty::Default);
            self.set_input(&default);
        }

        self.set_property(&FieldProperty::Locked, true);
    }

    pub fn set_error(&self, message: &str) {
        self.set_property(&FieldProperty::Error, message);
    }

    pub fn unset_error(&self) {
        self.set_property(&FieldProperty::Error, "");
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
