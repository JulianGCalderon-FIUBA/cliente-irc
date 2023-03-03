mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

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

    pub fn text(&self) -> String {
        self.property("text")
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}