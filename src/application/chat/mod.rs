use glib::Object;
use gtk::glib;

use crate::application::Application;

mod imp;

glib::wrapper! {
    pub struct Chat(ObjectSubclass<imp::Chat>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

}

impl Chat {
    pub fn new(application: &Application) -> Self {
        Object::builder()
            .property("application", application)
            .build()
    }
}
