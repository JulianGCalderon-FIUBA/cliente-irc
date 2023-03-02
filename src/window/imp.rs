use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate};

use crate::client::IrcClient;

use super::registration::Registration;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/window.ui")]
pub struct Window {
    #[template_child]
    registration: TemplateChild<Registration>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}

#[template_callbacks]
impl Window {
    #[template_callback]
    pub fn registered(&self, client: IrcClient) {
        println!("registered");
    }
}
