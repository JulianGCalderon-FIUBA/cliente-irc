use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct Chat(ObjectSubclass<imp::Chat>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

}

impl Chat {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}
