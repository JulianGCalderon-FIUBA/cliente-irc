use gtk::gdk::Display;
use gtk::glib;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::glib::WeakRef;
use gtk::CssProvider;
use gtk::StyleContext;
// use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::traits::GtkWindowExt;

use crate::server::Server;

// use super::main_window::MainWindow;
use super::MainWindow;
use super::RegistrationWindow;

#[derive(Default)]
pub struct Application {
    pub registration_window: WeakRef<RegistrationWindow>,
    pub main_window: WeakRef<MainWindow>,
    pub server: OnceCell<Server>,
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = gtk::Application;
}

impl ObjectImpl for Application {}
impl GtkApplicationImpl for Application {}
impl ApplicationImpl for Application {
    fn activate(&self) {
        let application = self.obj();

        self.parent_activate();

        application.setup_registration();
        application.registration_window().present();

        application.setup_main_window();
        application.main_window().present();
    }

    fn shutdown(&self) {
        self.parent_shutdown();

        if self.obj().is_connected() {
            self.obj().send_quit()
        }
    }

    fn startup(&self) {
        self.parent_startup();

        let provider = CssProvider::new();
        provider.load_from_data(include_str!("../../resources/style.css"));

        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
