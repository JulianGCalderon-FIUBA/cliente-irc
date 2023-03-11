//! Implementation of the Field component.

use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecBoolean, ParamSpecString};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Entry, Label};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/components/field.ui")]
pub struct Field {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub error_label: TemplateChild<Label>,
    name: RefCell<String>,
    input: RefCell<String>,
    default: RefCell<String>,
    error: RefCell<String>,
    locked: RefCell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for Field {
    const NAME: &'static str = "Field";
    type Type = super::Field;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("field")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Field {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("name").build(),
                ParamSpecString::builder("input").build(),
                ParamSpecString::builder("default").build(),
                ParamSpecString::builder("error").build(),
                ParamSpecBoolean::builder("locked").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "name" => {
                let value = value.get().unwrap();
                self.name.replace(value);
            }
            "input" => {
                let value = value.get().unwrap();
                self.input.replace(value);
            }
            "default" => {
                let value = value.get().unwrap();
                self.default.replace(value);
            }

            "locked" => {
                let value = value.get().unwrap();
                self.locked.replace(value);
            }
            "error" => {
                let value = value.get().unwrap();
                self.error.replace(value);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "input" => self.input.borrow().to_value(),
            "default" => self.default.borrow().to_value(),
            "locked" => self.locked.borrow().to_value(),
            "error" => self.error.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        self.obj().setup_bindings();
    }
}
impl WidgetImpl for Field {}
impl BoxImpl for Field {}
