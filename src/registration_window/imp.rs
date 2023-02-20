use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/registration.ui")]
pub struct RegistrationWindow {}

#[glib::object_subclass]
impl ObjectSubclass for RegistrationWindow {
    const NAME: &'static str = "RegistrationWindow";
    type Type = super::RegistrationWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl WindowImpl for RegistrationWindow {}
impl WidgetImpl for RegistrationWindow {}
impl ObjectImpl for RegistrationWindow {}
impl ApplicationWindowImpl for RegistrationWindow {}
