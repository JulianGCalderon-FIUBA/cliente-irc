use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::glib::ParamSpec;
use gtk::prelude::{ObjectExt, ToValue};
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Entry, ListBox};

use crate::utils::get_and_clear_entry;
use crate::widgets::chat_page::constant::ChatSignal;

use super::{create_own_message, ChatPageProperty};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/chat.ui")]
pub struct ChatPage {
    #[template_child]
    pub messages: TemplateChild<ListBox>,
    name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for ChatPage {
    const NAME: &'static str = "Chat";
    type Type = super::ChatPage;
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

impl ObjectImpl for ChatPage {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(ChatPageProperty::vec);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match ChatPageProperty::from(pspec.name()) {
            ChatPageProperty::Name => {
                let name: String = value.get().unwrap();
                self.name.replace(name);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match ChatPageProperty::from(pspec.name()) {
            ChatPageProperty::Name => self.name.borrow().to_value(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(ChatSignal::vec);
        SIGNALS.as_ref()
    }

    fn dispose(&self) {}
}
impl WidgetImpl for ChatPage {}
impl BoxImpl for ChatPage {}

#[template_callbacks]
impl ChatPage {
    #[template_callback]
    /// Called when the user sends a message through the chat
    pub fn send_message(&self, entry: Entry) {
        if let Some(message) = get_and_clear_entry(entry) {
            self.obj()
                .emit_by_name::<()>(&ChatSignal::Send, &[&message.to_value()]);

            let message = create_own_message(message);
            self.messages.append(&message);
        }
    }

    /// Called when the user atempts to close de chat
    #[template_callback]
    pub fn close_chat(&self) {
        self.obj().emit_by_name(&ChatSignal::Close, &[])
    }
}
