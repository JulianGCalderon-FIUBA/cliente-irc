use std::cell::RefCell;

use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::{InitializingObject, Signal};
use gtk::glib::{BindingFlags, ParamSpec, ParamSpecString};
use gtk::subclass::prelude::*;
use gtk::traits::EntryExt;
use gtk::{glib, CompositeTemplate, Entry, ListBox};
use gtk::{prelude::*, Label};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/chat.ui")]
pub struct Chat {
    #[template_child]
    pub client_label: TemplateChild<Label>,
    #[template_child]
    pub message_list: TemplateChild<ListBox>,
    pub client: RefCell<String>,
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
    fn send_message(&self, entry: &Entry) {
        let message = entry.buffer().text().to_string();

        self.obj()
            .emit_by_name::<()>("send-message", &[&message.to_value()]);

        self.obj().add_own_message(message);

        entry.buffer().set_text("");
    }
}

impl ObjectImpl for Chat {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("send-message")
                .param_types([String::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecString::builder("client").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "client" => {
                let address = value.get().unwrap();
                self.client.replace(address);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "client" => self.client.borrow().to_string().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let chat = self.obj();

        self.client_label
            .bind_property("label", chat.as_ref(), "client")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
    }

    fn dispose(&self) {}
}
impl WidgetImpl for Chat {}
impl BoxImpl for Chat {}
