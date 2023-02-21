mod registration_window;
mod server;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use registration_window::RegistrationWindow;

const APPLICATION_ID: &str = "com.jgcalderon.irc-client";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources");

    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    application.connect_activate(build_ui);

    application.run()
}

fn build_ui(application: &Application) {
    let registration_window = RegistrationWindow::new(application);
    registration_window.present();
}
