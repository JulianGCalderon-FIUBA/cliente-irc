//! Implementation of the chat adder page.

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::prelude::{ObjectExt, StaticType, ToValue};
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Entry};

use crate::utils::get_and_clear_entry;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/add-chat-page.ui")]
pub struct ChatAdder {}

#[glib::object_subclass]
impl ObjectSubclass for ChatAdder {
    const NAME: &'static str = "AddChatPage";
    type Type = super::ChatAdder;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("chat-adder");
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ChatAdder {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("add")
                .param_types([String::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for ChatAdder {}
impl BoxImpl for ChatAdder {}

#[template_callbacks]
impl ChatAdder {
    /// Called when the user attempts to add a new chat
    #[template_callback]
    fn add_chat(&self, entry: Entry) {
        if let Some(name) = get_and_clear_entry(entry) {
            self.obj().emit_by_name("add", &[&name.to_value()])
        }
    }
}
