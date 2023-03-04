use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecString};
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, PasswordEntry};
use std::cell::RefCell;

use super::PasswordFieldProperty;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/password-field.ui")]
pub struct PasswordField {
    #[template_child]
    pub entry: TemplateChild<PasswordEntry>,
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
        klass.set_css_name("field")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PasswordField {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder(&PasswordFieldProperty::Name).build(),
                ParamSpecString::builder(&PasswordFieldProperty::Input).build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match PasswordFieldProperty::from(pspec.name()) {
            PasswordFieldProperty::Name => {
                let value = value.get().unwrap();
                self.name.replace(value);
            }
            PasswordFieldProperty::Input => {
                let value = value.get().unwrap();
                self.input.replace(value);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match PasswordFieldProperty::from(pspec.name()) {
            PasswordFieldProperty::Name => self.name.borrow().to_value(),
            PasswordFieldProperty::Input => self.input.borrow().to_value(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for PasswordField {}
impl BoxImpl for PasswordField {}
