use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::ParamSpec;
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::client::UserData;
use crate::widgets::user_page::UserPageProperty;

// use super::UserPageProperty;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/user-page.ui")]
pub struct UserPage {
    data: RefCell<UserData>,
}

#[glib::object_subclass]
impl ObjectSubclass for UserPage {
    const NAME: &'static str = "UserPage";
    type Type = super::UserPage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("user-page")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for UserPage {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(UserPageProperty::vec);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match UserPageProperty::from(pspec.name()) {
            UserPageProperty::Data => {
                let data = value.get().unwrap();
                self.data.replace(data);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match UserPageProperty::from(pspec.name()) {
            UserPageProperty::Data => self.data.borrow().to_value(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }

    fn dispose(&self) {}
}
impl WidgetImpl for UserPage {}
impl BoxImpl for UserPage {}
