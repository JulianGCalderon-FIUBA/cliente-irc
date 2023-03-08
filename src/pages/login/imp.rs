use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::{Lazy, OnceCell};
use gtk::glib::subclass::Signal;
use gtk::prelude::StaticType;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, Button, CompositeTemplate};

use crate::client::{IrcClient, UserData};
use crate::components::field::Field;
use crate::components::password_field::PasswordField;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/registration.ui")]
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
    pub client: OnceCell<IrcClient>,
}

#[glib::object_subclass]
impl ObjectSubclass for Login {
    const NAME: &'static str = "Registration";
    type Type = super::Login;
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

impl ObjectImpl for Login {
    fn constructed(&self) {
        self.parent_constructed();

        // AUTOMATIC LOGIN: ONLY FOR TESTING PURPOSES
        // let generator = rnglib::RNG::try_from(&rnglib::Language::Roman).unwrap();
        // let nickname = generator.generate_short();
        // self.nickname.set_input(&nickname);
        // self.connect_clicked();
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("registered")
                .param_types([
                    super::Login::static_type(),
                    IrcClient::static_type(),
                    UserData::static_type(),
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
