use std::cell::RefCell;

use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::{InitializingObject, Signal};
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};
use gtk::{prelude::*, Label};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/chat-header.ui")]
pub struct ChatHeader {
    #[template_child]
    pub chat_label: TemplateChild<Label>,
    pub chat_name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for ChatHeader {
    const NAME: &'static str = "ChatHeader";
    type Type = super::ChatHeader;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl ChatHeader {
    #[template_callback]
    fn close_window(&self, _button: &Button) {
        self.obj().emit_by_name::<()>("close-request", &[]);
    }
}

impl ObjectImpl for ChatHeader {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> =
            Lazy::new(|| vec![Signal::builder("close-request").build()]);
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecString::builder("chat-name").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "chat-name" => {
                let address = value.get().unwrap();
                self.chat_name.replace(address);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "chat-name" => self.chat_name.borrow().to_string().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for ChatHeader {}
impl BoxImpl for ChatHeader {}
