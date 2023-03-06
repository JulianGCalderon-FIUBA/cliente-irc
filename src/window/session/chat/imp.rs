use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::{EntryBufferExtManual, ToValue};
use gtk::subclass::prelude::*;
use gtk::traits::EntryExt;
use gtk::{glib, template_callbacks, CompositeTemplate, Entry};

use super::ChatProperty;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/chat.ui")]
pub struct Chat {
    name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for Chat {
    const NAME: &'static str = "Chat";
    type Type = super::Chat;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();

        klass.set_css_name("chat")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Chat {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecString::builder(&ChatProperty::Name).build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match ChatProperty::from(pspec.name()) {
            ChatProperty::Name => {
                let name: String = value.get().unwrap();
                self.name.replace(name);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match ChatProperty::from(pspec.name()) {
            ChatProperty::Name => self.name.borrow().to_value(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for Chat {}
impl BoxImpl for Chat {}

#[template_callbacks]
impl Chat {
    #[template_callback]
    pub fn send_message(&self, entry: Entry) {
        let buffer = entry.buffer();
        let message = buffer.text().to_string();

        if message.is_empty() {
            return;
        }

        buffer.set_text("");
        println!("todo! send message: {message}");
    }

    #[template_callback]
    pub fn close_chat(&self) {
        println!("todo! close chat");
    }
}
