use glib::subclass::InitializingObject;
use gtk::prelude::StaticTypeExt;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Stack};

use crate::client::IrcClient;

use super::registration::Registration;
use super::session::Session;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/window.ui")]
pub struct Window {
    #[template_child]
    stack: TemplateChild<Stack>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();

        Registration::ensure_type()
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
        let session = Session::new(client);
        self.stack.add_named(&session, Some("session"));
        self.stack.set_visible_child_name("session");
    }
}
