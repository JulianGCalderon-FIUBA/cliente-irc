//! Implementation of the Login page.

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::{Lazy, OnceCell};
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, Button, CompositeTemplate};

use crate::components::field::Field;
use crate::components::password_field::PasswordField;
use crate::gtk_client::{BoxedIrcClient, RegistrationDataObject};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/pages/login.ui")]
pub struct Login {
    #[template_child]
    pub address: TemplateChild<Field>,
    #[template_child]
    pub nickname: TemplateChild<Field>,
    #[template_child]
    pub password: TemplateChild<PasswordField>,
    #[template_child]
    pub username: TemplateChild<Field>,
    #[template_child]
    pub realname: TemplateChild<Field>,
    #[template_child]
    pub connect: TemplateChild<Button>,
    pub client: OnceCell<BoxedIrcClient>,
}

#[glib::object_subclass]
impl ObjectSubclass for Login {
    const NAME: &'static str = "Registration";
    type Type = super::Login;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
        klass.set_css_name("login")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Login {
    fn constructed(&self) {
        self.parent_constructed();

        #[cfg(feature = "automatic-login")]
        self.obj().automatic_login();
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("registered")
                .param_types([
                    super::Login::static_type(),
                    BoxedIrcClient::static_type(),
                    RegistrationDataObject::static_type(),
                ])
                .build()]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for Login {}
impl BoxImpl for Login {}

#[template_callbacks]
impl Login {
    #[template_callback]
    /// Called when the connect button is clicked
    ///
    /// This function will attempt to connect to the server and register the user
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
