use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::prelude::StaticTypeExt;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Entry, Label, Stack};

use crate::client::{ClientData, IrcClient};
use crate::utils::get_and_clear_entry;

use super::chat::Chat;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/session.ui")]
pub struct Session {
    #[template_child]
    pub chats: TemplateChild<Stack>,
    #[template_child]
    pub info: TemplateChild<Label>,
    pub client: OnceCell<IrcClient>,
    pub client_data: RefCell<ClientData>,
}

#[glib::object_subclass]
impl ObjectSubclass for Session {
    const NAME: &'static str = "Session";
    type Type = super::Session;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();

        Chat::ensure_type();
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

#[template_callbacks]
impl Session {
    #[template_callback]
    pub fn add_chat(&self, entry: Entry) {
        if let Some(name) = get_and_clear_entry(entry) {
            self.obj().add_chat(name);
        }
    }
}
