mod imp;
mod field;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Registration(ObjectSubclass<imp::Registration>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Registration {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for Registration {
    fn default() -> Self {
        Self::new()
    }
}
