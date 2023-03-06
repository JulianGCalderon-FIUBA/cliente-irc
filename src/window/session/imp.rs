use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::prelude::{EntryBufferExtManual, StaticTypeExt};
use gtk::subclass::prelude::*;
use gtk::traits::EntryExt;
use gtk::{glib, template_callbacks, CompositeTemplate, Entry};

use crate::client::IrcClient;

use super::chat::Chat;

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
        let buffer = entry.buffer();
        let client = buffer.text().to_string();

        if client.is_empty() {
            return;
        }

        buffer.set_text("");
        println!("todo! add chat: {client}");
    }
}
