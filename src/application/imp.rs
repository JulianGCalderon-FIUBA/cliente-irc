use gtk::glib::{self, WeakRef};
use gtk::subclass::prelude::*;
use gtk::traits::GtkWindowExt;

use crate::window::Window;

#[derive(Default)]
pub struct Application {
    pub window: WeakRef<Window>,
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = gtk::Application;
}

impl ObjectImpl for Application {}
impl ApplicationImpl for Application {
    fn activate(&self) {
        let application = self.obj();

        let window = Window::new(&application);
        self.window.set(Some(&window));

        window.present();

        self.parent_activate()
    }
}

impl GtkApplicationImpl for Application {}
