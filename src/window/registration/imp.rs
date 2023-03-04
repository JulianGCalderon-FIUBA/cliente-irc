use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::{Lazy, OnceCell};
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Button;
use gtk::{glib, template_callbacks, CompositeTemplate};

use crate::client::IrcClient;

use super::field::Field;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/registration.ui")]
pub struct Registration {
    #[template_child]
    pub address: TemplateChild<Field>,
    #[template_child]
    pub nickname: TemplateChild<Field>,
    #[template_child]
    pub password: TemplateChild<Field>,
    #[template_child]
    pub username: TemplateChild<Field>,
    #[template_child]
    pub realname: TemplateChild<Field>,
    #[template_child]
    pub connect: TemplateChild<Button>,
    pub client: OnceCell<IrcClient>,
}

#[glib::object_subclass]
impl ObjectSubclass for Registration {
    const NAME: &'static str = "Registration";
    type Type = super::Registration;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
        klass.set_css_name("registration")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Registration {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static PROPERTIES: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("registered")
                .param_types([super::Registration::static_type(), IrcClient::static_type()])
                .build()]
        });
        PROPERTIES.as_ref()
    }
}
impl WidgetImpl for Registration {}
impl BoxImpl for Registration {}

#[template_callbacks]
impl Registration {
    #[template_callback]
    pub fn connect_clicked(&self) {
        let registration = self.obj();

        if !registration.connected() {
            if registration.setup_client().is_err() {
                return self.address.set_error("Could not connect to server");
            } else {
                self.address.unset_error()
            }
        }

        if registration.register_client().is_err() {
            todo!("connection closed")
        };
    }
}
