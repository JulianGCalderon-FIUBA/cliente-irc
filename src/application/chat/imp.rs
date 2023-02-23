use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::{InitializingObject, Signal};
use gtk::glib::ParamSpecString;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::traits::EntryExt;
use gtk::{glib, CompositeTemplate, Entry};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/chat.ui")]
pub struct Chat {}

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
        entry.buffer().set_text("");

        let message = entry.buffer().text().to_string();
        self.obj()
            .emit_by_name("send-message", &[&message.to_value()])
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
}
impl WidgetImpl for Chat {}
impl BoxImpl for Chat {}
