use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecBoolean, ParamSpecString};
use gtk::prelude::{ObjectExt, ToValue};
use gtk::subclass::prelude::*;
use gtk::traits::WidgetExt;
use gtk::{glib, CompositeTemplate, Entry, Label};
use std::cell::RefCell;

use crate::window::registration::field::FieldProperty;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/field.ui")]
pub struct Field {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub error_label: TemplateChild<Label>,
    name: RefCell<String>,
    input: RefCell<String>,
    default: RefCell<String>,
    locked: RefCell<bool>,
    error: RefCell<String>,
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
                ParamSpecString::builder(&FieldProperty::Name).build(),
                ParamSpecString::builder(&FieldProperty::Input).build(),
                ParamSpecString::builder(&FieldProperty::Default).build(),
                ParamSpecString::builder(&FieldProperty::Error).build(),
                ParamSpecBoolean::builder(&FieldProperty::Locked).build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match FieldProperty::from(pspec.name()) {
            FieldProperty::Name => {
                let value = value.get().unwrap();
                self.name.replace(value);
            }
            FieldProperty::Input => {
                let value = value.get().unwrap();
                self.input.replace(value);
            }
            FieldProperty::Default => {
                let value = value.get().unwrap();
                self.default.replace(value);
            }

            FieldProperty::Locked => {
                let value = value.get().unwrap();
                self.locked.replace(value);
            }
            FieldProperty::Error => {
                let value = value.get().unwrap();
                self.error.replace(value);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match FieldProperty::from(pspec.name()) {
            FieldProperty::Name => self.name.borrow().to_value(),
            FieldProperty::Input => self.input.borrow().to_value(),
            FieldProperty::Default => self.default.borrow().to_value(),
            FieldProperty::Locked => self.locked.borrow().to_value(),
            FieldProperty::Error => self.error.borrow().to_value(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let field = self.obj();

        field
            .bind_property::<Label>(&FieldProperty::Error, &self.error_label, "visible")
            .transform_to(|_, error: String| Some(!error.is_empty()))
            .build();

        field
            .bind_property::<Entry>(&FieldProperty::Locked, &self.entry, "secondary-icon-name")
            .transform_to(|_, locked: bool| {
                if locked {
                    Some("system-lock-screen-symbolic")
                } else {
                    Some("")
                }
            })
            .build();

        field.connect_notify(Some("error"), |field, _| {
            let error: String = field.property("error");
            if error.is_empty() {
                field.remove_css_class("invalid");
            } else {
                field.add_css_class("invalid");
            };
        });
    }
}
impl WidgetImpl for Field {}
impl BoxImpl for Field {}
