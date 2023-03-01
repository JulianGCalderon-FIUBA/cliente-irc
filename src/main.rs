mod message;
mod client;
mod window;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::traits::GtkWindowExt;
use gtk::Application;

use window::Window;

// use application::Application;

const APPLICATION_ID: &str = "com.jgcalderon.irc-client";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources");

    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .flags(gio::ApplicationFlags::NON_UNIQUE)
        .build();

    application.connect_activate(build);

    application.run()
}

fn build(application: &Application) {
    let window = Window::new(application);
    window.present();
}
