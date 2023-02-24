use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::{InitializingObject, Signal};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Stack};
use gtk::{prelude::*, Entry};

use crate::application::chat::Chat;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/main.ui")]
pub struct MainWindow {
    #[template_child]
    pub chats_stack: TemplateChild<Stack>,
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();

        Chat::ensure_type();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl MainWindow {
    #[template_callback]
    fn add_client(&self, entry: &Entry) {
        let client = entry.buffer().text().to_string();
        entry.buffer().set_text("");

        self.obj().add_chat(&client);
    }
}

impl WindowImpl for MainWindow {}
impl WidgetImpl for MainWindow {}
impl ObjectImpl for MainWindow {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("send-message-request")
                .param_types([String::static_type(), String::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
}

impl ApplicationWindowImpl for MainWindow {}
