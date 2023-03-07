use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::{Lazy, OnceCell};
use gtk::glib::subclass::Signal;
use gtk::subclass::prelude::*;
use gtk::Button;
use gtk::{glib, template_callbacks, CompositeTemplate};
use rnglib::{Language, RNG};

use crate::client::IrcClient;
use crate::pages::registration::RegistrationSignal;

use crate::widgets::field::Field;
use crate::widgets::password_field::PasswordField;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/registration.ui")]
pub struct Registration {
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

        // AUTOMATIC LOGIN: ONLY FOR TESTING PURPOSES
        let generator = RNG::try_from(&Language::Roman).unwrap();
        let nickname = generator.generate_short();
        self.nickname.set_input(&nickname);
        self.connect_clicked();
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(RegistrationSignal::vec);
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for Registration {}
impl BoxImpl for Registration {}

#[template_callbacks]
impl Registration {
    #[template_callback]
    /// Called after ´connect` button is clicked.
    ///
    /// Attempts to connect and register to the server
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
