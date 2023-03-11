//! Implementation of the PasswordField widget.

use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, LevelBar, PasswordEntry};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/components/password-field.ui")]
pub struct PasswordField {
    #[template_child]
    pub entry: TemplateChild<PasswordEntry>,
    #[template_child]
    pub level_bar: TemplateChild<LevelBar>,
    name: RefCell<String>,
    input: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for PasswordField {
    const NAME: &'static str = "PasswordField";
    type Type = super::PasswordField;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("password-field");
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PasswordField {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("name").build(),
                ParamSpecString::builder("input").build(),
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
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "input" => self.input.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        self.obj().setup_bindings();
    }
}
impl WidgetImpl for PasswordField {}
impl BoxImpl for PasswordField {}
