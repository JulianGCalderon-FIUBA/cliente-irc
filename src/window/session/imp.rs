use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::client::IrcClient;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/session.ui")]
pub struct Session {
    pub client: OnceCell<IrcClient>,
}

#[glib::object_subclass]
impl ObjectSubclass for Session {
    const NAME: &'static str = "Session";
    type Type = super::Session;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Session {
    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for Session {}
impl BoxImpl for Session {}

// #[template_callbacks]
// impl Session {
//     #[template_callback]
//     pub fn handler(&self) {

//     }
// }
