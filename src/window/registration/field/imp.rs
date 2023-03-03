use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Entry};

pub const NAME_PROPERTY: &str = "name";
pub const INPUT_PROPERTY: &str = "input";
pub const DEFAULT_PROPERTY: &str = "default";

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/registration-field.ui")]
pub struct Field {
    #[template_child(internal = true)]
    pub entry: TemplateChild<Entry>,
    name: RefCell<String>,
    input: RefCell<String>,
    default: RefCell<String>,
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
                ParamSpecString::builder(NAME_PROPERTY).build(),
                ParamSpecString::builder(INPUT_PROPERTY).build(),
                ParamSpecString::builder(DEFAULT_PROPERTY).build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            NAME_PROPERTY => {
                let value = value.get().unwrap();
                self.name.replace(value);
            }
            INPUT_PROPERTY => {
                let value = value.get().unwrap();
                self.input.replace(value);
            }
            DEFAULT_PROPERTY => {
                let value = value.get().unwrap();
                self.default.replace(value);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            NAME_PROPERTY => self.name.borrow().to_value(),
            INPUT_PROPERTY => self.input.borrow().to_value(),
            DEFAULT_PROPERTY => self.default.borrow().to_value(),

            _ => unimplemented!(),
        }
    }
}
impl WidgetImpl for Field {}
impl BoxImpl for Field {}
