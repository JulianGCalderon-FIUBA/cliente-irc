use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::application::chat::Chat;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/main.ui")]
pub struct MainWindow {}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        Chat::ensure_type();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl WindowImpl for MainWindow {}
impl WidgetImpl for MainWindow {}
impl ObjectImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}
