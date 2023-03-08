use std::cell::RefCell;

use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{self, ParamSpec, ParamSpecString};
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;

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
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("nickname").build(),
                ParamSpecString::builder("realname").build(),
                ParamSpecString::builder("username").build(),
                ParamSpecString::builder("hostname").build(),
                ParamSpecString::builder("servername").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "nickname" => {
                let nickname = value.get().unwrap();
                self.nickname.replace(nickname);
            }
            "realname" => {
                let realname = value.get().unwrap();
                self.realname.replace(realname);
            }
            "username" => {
                let username = value.get().unwrap();
                self.username.replace(username);
            }
            "hostname" => {
                let hostname = value.get().unwrap();
                self.hostname.replace(hostname);
            }
            "servername" => {
                let servername = value.get().unwrap();
                self.servername.replace(servername);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "nickname" => self.nickname.borrow().to_value(),
            "realname" => self.realname.borrow().to_value(),
            "username" => self.username.borrow().to_value(),
            "hostname" => self.hostname.borrow().to_value(),
            "servername" => self.servername.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }

    fn dispose(&self) {}
}
impl WidgetImpl for UserData {}
impl BoxImpl for UserData {}
