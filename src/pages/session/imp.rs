//! Implementation of the Session page.

use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::{Lazy, OnceCell};
use gtk::glib::{ParamSpec, ParamSpecObject};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Stack};

use super::{build_name_for_chat_title, CHANNEL_INDICATOR};
use crate::components::CategorizedStackSidebar;
use crate::gtk_client::{BoxedIrcClient, RegistrationDataObject};
use crate::pages::{Account, Chat, ChatAdder};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/session.ui")]
pub struct Session {
    #[template_child]
    pub pages: TemplateChild<Stack>,
    #[template_child]
    pub sidebar: TemplateChild<CategorizedStackSidebar>,
    pub client: OnceCell<BoxedIrcClient>,
    pub data: RefCell<RegistrationDataObject>,
}

#[glib::object_subclass]
impl ObjectSubclass for Session {
    const NAME: &'static str = "Session";
    type Type = super::Session;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
        klass.set_css_name("session");

        Chat::ensure_type();
        ChatAdder::ensure_type();
        Account::ensure_type();
        CategorizedStackSidebar::ensure_type();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Session {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::builder::<RegistrationDataObject>("registration-data").build()]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "registration-data" => {
                let data = value.get().unwrap();
                self.data.replace(data);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "registration-data" => self.data.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        self.sidebar.add_category("config");
    }
}
impl WidgetImpl for Session {}
impl BoxImpl for Session {}

#[template_callbacks]
impl Session {
    /// Called when the user requests to add a new chat
    ///
    /// This function adds a new chat to the session and
    /// sends a join command to the server if the chat is a channel
    ///
    /// The new chat is made visible
    #[template_callback]
    pub fn add_chat(&self, title: String) {
        self.obj().add_chat(title.clone());

        let name = build_name_for_chat_title(&title);
        self.pages.set_visible_child_name(&name);

        if title.starts_with(CHANNEL_INDICATOR) {
            self.obj().join_channel(title);
        }
    }
}
