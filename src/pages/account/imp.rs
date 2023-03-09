use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecObject};
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::gtk_client::RegistrationDataObject;

// use super::UserPageProperty;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/user-page.ui")]
pub struct Account {
    data: RefCell<RegistrationDataObject>,
}

#[glib::object_subclass]
impl ObjectSubclass for Account {
    const NAME: &'static str = "UserPage";
    type Type = super::Account;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("user-page")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Account {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::builder::<RegistrationDataObject>("user-data").build()]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "user-data" => {
                let data = value.get().unwrap();
                self.data.replace(data);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "user-data" => self.data.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }

    fn dispose(&self) {}
}
impl WidgetImpl for Account {}
impl BoxImpl for Account {}
