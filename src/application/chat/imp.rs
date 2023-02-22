use gtk::glib::subclass::InitializingObject;
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
        let message = entry.buffer().text().to_string();
        println!("send message: {message}")
    }
}

impl ObjectImpl for Chat {}
impl WidgetImpl for Chat {}
impl BoxImpl for Chat {}
