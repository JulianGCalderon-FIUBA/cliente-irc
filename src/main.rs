//! IRC Client
//!
//! This is a simple IRC client written in Rust using GTK and GTK-rs.

#![warn(missing_docs)]

mod components;
mod gtk_client;
mod pages;
mod utils;
mod window;

use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::traits::GtkWindowExt;
use gtk::{gio, glib, Application, CssProvider, IconTheme, StyleContext};
use window::Window;

/// The application ID.
///
/// This is used to identify the application in the desktop environment.
const APPLICATION_ID: &str = "com.jgcalderon.irc-client";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources");

    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .flags(gio::ApplicationFlags::NON_UNIQUE)
        .build();

    application.connect_startup(|_| load());
    application.connect_activate(build);

    application.run()
}

/// Build the main window.
///
/// This function is called when the application is activated.
fn build(application: &Application) {
    let window = Window::new(application);
    window.present();
}

/// Load the CSS and custom icons.
///
/// This function is called when the application starts.
fn load() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../resources/style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    IconTheme::default().add_resource_path("/com/jgcalderon/irc-client");
}
