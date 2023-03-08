//! A Rust IRC-Client built in Gtk4

#![warn(missing_docs)]

mod client;
mod message;
mod pages;
mod utils;
mod widgets;
mod window;

use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::traits::GtkWindowExt;
use gtk::{gio, glib, Application, CssProvider, IconTheme, StyleContext};
use window::Window;

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

fn build(application: &Application) {
    let window = Window::new(application);
    window.present();
}

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
