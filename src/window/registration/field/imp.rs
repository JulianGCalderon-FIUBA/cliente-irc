use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/registration-field.ui")]
pub struct Field {
    label: RefCell<String>,
    text: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for Field {
    const NAME: &'static str = "Field";
    type Type = super::Field;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Field {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("label").build(),
                ParamSpecString::builder("text").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "label" => {
                let value = value.get().unwrap();
                self.label.replace(value);
            }
            "text" => {
                let value = value.get().unwrap();
                self.text.replace(value);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "label" => self.label.borrow().to_value(),
            "text" => self.text.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
impl WidgetImpl for Field {}
impl BoxImpl for Field {}
