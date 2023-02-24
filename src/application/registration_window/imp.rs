use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::{InitializingObject, Signal};
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry};
use std::cell::RefCell;

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
    pub address: RefCell<String>,
    pub password: RefCell<String>,
    pub nickname: RefCell<String>,
    pub username: RefCell<String>,
    pub realname: RefCell<String>,
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
        self.obj().emit_by_name("connect-button-clicked", &[])
    }
}

impl WindowImpl for RegistrationWindow {}
impl WidgetImpl for RegistrationWindow {}
impl ObjectImpl for RegistrationWindow {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("address").build(),
                ParamSpecString::builder("password").build(),
                ParamSpecString::builder("nickname").build(),
                ParamSpecString::builder("username").build(),
                ParamSpecString::builder("realname").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> =
            Lazy::new(|| vec![Signal::builder("connect-button-clicked").build()]);
        SIGNALS.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "address" => {
                let address = value.get().unwrap();
                self.address.replace(address);
            }
            "password" => {
                let password = value.get().unwrap();
                self.password.replace(password);
            }
            "nickname" => {
                let nickname = value.get().unwrap();
                self.nickname.replace(nickname);
            }
            "username" => {
                let username = value.get().unwrap();
                self.username.replace(username);
            }
            "realname" => {
                let realname = value.get().unwrap();
                self.realname.replace(realname);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "address" => self.address.borrow().to_string().to_value(),
            "password" => self.password.borrow().to_string().to_value(),
            "nickname" => self.nickname.borrow().to_string().to_value(),
            "username" => self.username.borrow().to_string().to_value(),
            "realname" => self.realname.borrow().to_string().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl ApplicationWindowImpl for RegistrationWindow {}
