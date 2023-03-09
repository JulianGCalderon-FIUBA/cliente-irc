//! # Window
//!
//! This module contains the `Window` type, which is the main window of the application.

mod imp;

use glib::Object;
use gtk::{gio, glib, Application};

glib::wrapper! {
    /// The main window of the application.
    ///
    /// This is the main window of the application. It contains the main
    /// `gtk::Stack`
    ///
    /// After creation, the login page is shown. When the user logs in, the
    /// session page is shown.
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
