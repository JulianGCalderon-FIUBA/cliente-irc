use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::{Lazy, OnceCell};
use gtk::glib::{ParamSpec, ParamSpecObject};
use gtk::prelude::{StaticTypeExt, ToValue};
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Stack};

use super::CHANNEL_INDICATOR;
use crate::client::message::IrcCommand;
use crate::client::{IrcClient, UserData};
use crate::components::Sidebar;
use crate::pages::{Account, Chat, ChatAdder};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/session.ui")]
pub struct Session {
    #[template_child]
    pub pages: TemplateChild<Stack>,
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
        ChatAdder::ensure_type();
        Account::ensure_type();
        Sidebar::ensure_type();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Session {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecObject::builder::<UserData>("user-data").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "user-data" => {
                let data = value.get().unwrap();
                self.data.replace(data);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "user-data" => self.data.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
impl WidgetImpl for Session {}
impl BoxImpl for Session {}

#[template_callbacks]
impl Session {
    /// Called when a new chat openning is requested.
    /// Adds the chat, if it is a group chat, it also notifies the server of joining it.
    #[template_callback]
    pub fn add_chat(&self, name: String) {
        self.obj().add_chat(name.clone());
        let full_name = format!("chat-{name}");
        self.pages.set_visible_child_name(&full_name);

        if name.starts_with(CHANNEL_INDICATOR) {
            let join_command = IrcCommand::Join { name };
            if self.obj().client().send(join_command).is_err() {
                println!("todo! connection error");
            }
        }
    }
}
