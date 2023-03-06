use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::ParamSpec;
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use super::MessageProperty;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/message.ui")]
pub struct Message {
    message: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for Message {
    const NAME: &'static str = "Message";
    type Type = super::Message;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("message")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Message {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(MessageProperty::vec);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match MessageProperty::from(pspec.name()) {
            MessageProperty::Message => {
                let message: String = value.get().unwrap();
                self.message.replace(message);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match MessageProperty::from(pspec.name()) {
            MessageProperty::Message => self.message.borrow().to_value(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }

    fn dispose(&self) {}
}
impl WidgetImpl for Message {}
impl BoxImpl for Message {}
