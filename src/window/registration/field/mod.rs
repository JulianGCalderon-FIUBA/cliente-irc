mod imp;

use glib::Object;
use gtk::{glib, prelude::ObjectExt};

pub use imp::{INPUT_PROPERTY, NAME_PROPERTY};

use self::imp::DEFAULT_PROPERTY;

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
        let input: String = self.property(INPUT_PROPERTY);

        if input.is_empty() {
            return self.property(DEFAULT_PROPERTY);
        }

        input
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
