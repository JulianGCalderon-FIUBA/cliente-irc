mod imp;

use gtk::{
    gio::{self, ApplicationFlags},
    glib::{self, Object},
    prelude::*,
};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
    @extends gio::Application, gtk::Application,
    @implements gio::ActionMap, gio::ActionGroup;
}

const APPLICATION_ID: &str = "com.jgcalderon.irc-client";

impl Application {
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", APPLICATION_ID)
            .property("flags", ApplicationFlags::NON_UNIQUE)
            .build()
    }

    pub fn run(&self) -> glib::ExitCode {
        ApplicationExtManual::run(self)
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
