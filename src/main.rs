mod application;
mod registration_window;
mod server;
mod message;

use application::Application;

use gtk::prelude::*;
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources");

    let application = Application::new();
    application.run()
}
