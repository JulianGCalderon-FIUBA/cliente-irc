use std::cell::RefCell;

use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::InitializingObject;
use gtk::glib::{BindingFlags, ParamSpec, ParamSpecString};
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};
use gtk::{prelude::*, Label};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/chat-header.ui")]
pub struct ChatHeader {
    #[template_child]
    pub client_label: TemplateChild<Label>,
    pub client: RefCell<String>,
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
        println!("close button");
    }
}

impl ObjectImpl for ChatHeader {
    fn signals() -> &'static [glib::subclass::Signal] {
        // static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
        //     vec![Signal::builder("send-message")
        //         .param_types([String::static_type()])
        //         .build()]
        // });
        // SIGNALS.as_ref()
        &[]
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

        let chat_header = self.obj();

        self.client_label
            .bind_property("label", chat_header.as_ref(), "client")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
    }

    fn dispose(&self) {}
}
impl WidgetImpl for ChatHeader {}
impl BoxImpl for ChatHeader {}
