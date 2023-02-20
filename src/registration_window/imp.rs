use std::net::TcpStream;

use gtk::glib::once_cell::sync::OnceCell;
use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/registration.ui")]
pub struct RegistrationWindow {
    #[template_child]
    pub address_entry: TemplateChild<Entry>,
    #[template_child]
    pub password_entry: TemplateChild<Entry>,
    #[template_child]
    pub nickname_entry: TemplateChild<Entry>,
    #[template_child]
    pub username_entry: TemplateChild<Entry>,
    #[template_child]
    pub realname_entry: TemplateChild<Entry>,
    pub client: OnceCell<TcpStream>,
    pub password_sent: bool,
}

#[glib::object_subclass]
impl ObjectSubclass for RegistrationWindow {
    const NAME: &'static str = "RegistrationWindow";
    type Type = super::RegistrationWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl RegistrationWindow {
    #[template_callback]
    fn handle_button_clicked(&self, _button: &Button) {
        // let address = self.address_entry.buffer().to_string();
        // let password = self.address_entry.buffer().to_string();
        // let nickname = self.address_entry.buffer().to_string();
        // let username = self.address_entry.buffer().to_string();
        // let realname = self.address_entry.buffer().to_string();
    }
}

impl WindowImpl for RegistrationWindow {}
impl WidgetImpl for RegistrationWindow {}
impl ObjectImpl for RegistrationWindow {}
impl ApplicationWindowImpl for RegistrationWindow {}
