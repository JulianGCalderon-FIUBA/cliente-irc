mod imp;

use glib::Object;
use gtk::gio;
use gtk::glib;

use crate::application::Application;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionMap, gio::ActionGroup, gtk::Accessible,
        gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
        gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(application: &Application) -> Self {
        Object::builder()
            .property("application", &Some(application))
            .build()
    }
}
