use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/message.ui")]
pub struct Message {
    #[template_child]
    pub sender_label: TemplateChild<Label>,
    message: RefCell<String>,
    sender: RefCell<String>,
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
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("message").build(),
                ParamSpecString::builder("sender").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "message" => {
                let message: String = value.get().unwrap();
                self.message.replace(message);
            }
            "sender" => {
                let sender: String = value.get().unwrap();
                self.sender.replace(sender);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "message" => self.message.borrow().to_value(),
            "sender" => self.sender.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let message = self.obj();

        message.bind_sender_to_label_visibility();
    }

    fn dispose(&self) {}
}
impl WidgetImpl for Message {}
impl BoxImpl for Message {}
