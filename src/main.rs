mod client;
mod message;

use gtk::prelude::*;
use gtk::{gio, glib};

use irc_client::application::Application;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources");

    let application = Application::new();
    application.run()
}
