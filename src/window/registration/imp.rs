use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::Button;
use gtk::{glib, template_callbacks, CompositeTemplate};

use super::field::Field;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/registration.ui")]
pub struct Registration {
    #[template_child]
    address: TemplateChild<Field>,
    #[template_child]
    nickname: TemplateChild<Field>,
    #[template_child]
    password: TemplateChild<Field>,
    #[template_child]
    username: TemplateChild<Field>,
    #[template_child]
    realname: TemplateChild<Field>,
    #[template_child]
    connect: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for Registration {
    const NAME: &'static str = "Registration";
    type Type = super::Registration;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Registration {
    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for Registration {}
impl BoxImpl for Registration {}

#[template_callbacks]
impl Registration {
    #[template_callback]
    pub fn register(&self) {
        let address = self.address.text();
        let nickname = self.nickname.text();
        let password = self.password.text();
        let username = self.username.text();
        let realname = self.realname.text();

        println!("REGISTERING");

        println!("address: {address}");
        println!("nickname: {nickname}");
        println!("password: {password}");
        println!("username: {username}");
        println!("realname: {realname}");

        println!();
    }
}
