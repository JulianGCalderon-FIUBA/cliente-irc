use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct Application {}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = gtk::Application;
}

impl ObjectImpl for Application {}
impl ApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
