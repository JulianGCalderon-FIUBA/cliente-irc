use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct ChatHeader(ObjectSubclass<imp::ChatHeader>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

}

impl ChatHeader {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for ChatHeader {
    fn default() -> Self {
        Self::new()
    }
}
