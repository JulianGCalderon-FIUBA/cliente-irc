//! Implementation of the Chat widget

use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Entry, ListBox};

use super::create_own_message;
use crate::utils::get_and_clear_entry;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/pages/chat.ui")]
pub struct Chat {
    #[template_child]
    pub messages: TemplateChild<ListBox>,
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
            Lazy::new(|| vec![ParamSpecString::builder("name").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "name" => {
                let name: String = value.get().unwrap();
                self.name.replace(name);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("close").build(),
                Signal::builder("send")
                    .param_types([String::static_type()])
                    .build(),
            ]
        });
        SIGNALS.as_ref()
    }

    fn dispose(&self) {}
}
impl WidgetImpl for Chat {}
impl BoxImpl for Chat {}

#[template_callbacks]
impl Chat {
    /// Called when the user atempts to send a message
    ///
    /// Emits the `send` signal and adds the message to the chat
    ///
    /// Entry is cleared after the message is sent
    #[template_callback]
    pub fn send_message(&self, entry: Entry) {
        if let Some(message) = get_and_clear_entry(entry) {
            self.obj()
                .emit_by_name::<()>("send", &[&message.to_value()]);

            let message = create_own_message(message);
            self.messages.append(&message);
        }
    }

    /// Called when the user atempts to close de chat
    ///
    /// Emits the `close` signal
    #[template_callback]
    pub fn close_chat(&self) {
        self.obj().emit_by_name("close", &[])
    }
}
