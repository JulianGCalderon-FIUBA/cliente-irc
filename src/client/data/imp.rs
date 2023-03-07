use std::cell::RefCell;

use gtk::glib;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::ParamSpec;
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;

use super::UserDataProperty;

#[derive(Default)]
pub struct UserData {
    nickname: RefCell<String>,
    realname: RefCell<String>,
    username: RefCell<String>,
    hostname: RefCell<String>,
    servername: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for UserData {
    const NAME: &'static str = "UserData";
    type Type = super::UserData;
    type ParentType = glib::Object;
}

impl ObjectImpl for UserData {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(UserDataProperty::vec);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match UserDataProperty::from(pspec.name()) {
            UserDataProperty::Nickname => {
                let nickname = value.get().unwrap();
                self.nickname.replace(nickname);
            }
            UserDataProperty::Realname => {
                let realname = value.get().unwrap();
                self.realname.replace(realname);
            }
            UserDataProperty::Username => {
                let username = value.get().unwrap();
                self.username.replace(username);
            }
            UserDataProperty::Hostname => {
                let hostname = value.get().unwrap();
                self.hostname.replace(hostname);
            }
            UserDataProperty::Servername => {
                let servername = value.get().unwrap();
                self.servername.replace(servername);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match UserDataProperty::from(pspec.name()) {
            UserDataProperty::Nickname => self.nickname.borrow().to_value(),
            UserDataProperty::Realname => self.realname.borrow().to_value(),
            UserDataProperty::Username => self.username.borrow().to_value(),
            UserDataProperty::Hostname => self.hostname.borrow().to_value(),
            UserDataProperty::Servername => self.servername.borrow().to_value(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }

    fn dispose(&self) {}
}
impl WidgetImpl for UserData {}
impl BoxImpl for UserData {}
