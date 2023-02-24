use std::cell::RefCell;

use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::{InitializingObject, Signal};
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::traits::EntryExt;
use gtk::{glib, CompositeTemplate, Entry, ListBox};

use super::chat_header::ChatHeader;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/chat.ui")]
pub struct Chat {
    #[template_child]
    pub header: TemplateChild<ChatHeader>,
    #[template_child]
    pub message_list: TemplateChild<ListBox>,
    #[template_child]
    pub message_entry: TemplateChild<Entry>,
    pub chat_name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for Chat {
    const NAME: &'static str = "Chat";
    type Type = super::Chat;
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
impl Chat {
    #[template_callback]
    fn emit_send_message_request(&self) {
        let message = self.message_entry.buffer().text().to_string();
        if message.is_empty() {
            return;
        }

        self.message_entry.buffer().set_text("");

        self.obj()
            .emit_by_name::<()>("send-message-request", &[&message.to_value()]);

        self.obj().add_own_message(message);
    }

    #[template_callback]
    fn emit_close_request(&self) {
        self.obj().emit_by_name::<()>("close-request", &[]);
    }
}

impl ObjectImpl for Chat {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("send-message-request")
                    .param_types([String::static_type()])
                    .build(),
                Signal::builder("close-request").build(),
            ]
        });
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
impl WidgetImpl for Chat {}
impl BoxImpl for Chat {}
