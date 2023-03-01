mod application;
mod message;
mod server;

use gtk::{gio, glib};

use application::Application;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources");

    let application = Application::new();
    application.run()
}
