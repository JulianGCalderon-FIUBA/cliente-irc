use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::{Lazy, OnceCell};
use gtk::glib::ParamSpec;
use gtk::prelude::{StaticTypeExt, ToValue};
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Entry, Stack};

use crate::client::{IrcClient, UserData};
use crate::message::IrcCommand;
use crate::utils::get_and_clear_entry;

use super::chat::Chat;
use super::constant::SessionProperty;
use super::user_page::UserPage;
use super::CHANNEL_INDICATOR;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/session.ui")]
pub struct Session {
    #[template_child]
    pub chats: TemplateChild<Stack>,
    pub client: OnceCell<IrcClient>,
    pub data: RefCell<UserData>,
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
        UserPage::ensure_type();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Session {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(SessionProperty::vec);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match SessionProperty::from(pspec.name()) {
            SessionProperty::Data => {
                let data = value.get().unwrap();
                self.data.replace(data);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match SessionProperty::from(pspec.name()) {
            SessionProperty::Data => self.data.borrow().to_value(),
        }
    }
}
impl WidgetImpl for Session {}
impl BoxImpl for Session {}

#[template_callbacks]
impl Session {
    #[template_callback]
    pub fn add_chat(&self, entry: Entry) {
        if let Some(name) = get_and_clear_entry(entry) {
            self.obj().add_chat(name.clone());

            if name.starts_with(CHANNEL_INDICATOR) {
                let join_command = IrcCommand::Join { name };
                if self.obj().client().send(join_command).is_err() {
                    println!("todo! connection error");
                }
            }
        }
    }
}
