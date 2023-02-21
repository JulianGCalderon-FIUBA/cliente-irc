use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::traits::EntryExt;
use gtk::{glib, Button, CompositeTemplate, Entry};

use crate::server::Server;

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
        let address = self.address_entry.buffer().text().to_string();
        let password = self.password_entry.buffer().text().to_string();
        let nickname = self.nickname_entry.buffer().text().to_string();
        let username = self.username_entry.buffer().text().to_string();
        let realname = self.realname_entry.buffer().text().to_string();

        glib::MainContext::default().spawn_local(async move {
            let mut server = Server::connect(address).await.unwrap();

            let pass_command = format!("PASS {password}");
            let nick_command = format!("NICK {nickname}");
            let user_command = format!("USER {username} :{realname}");

            server.send(pass_command).await.unwrap();
            server.send(nick_command).await.unwrap();
            server.send(user_command).await.unwrap();

            let string = server.receive().await.unwrap();
            println!("{string}");
        });
    }
}

impl WindowImpl for RegistrationWindow {}
impl WidgetImpl for RegistrationWindow {}
impl ObjectImpl for RegistrationWindow {}
impl ApplicationWindowImpl for RegistrationWindow {}
