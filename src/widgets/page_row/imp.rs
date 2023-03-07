use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::ParamSpec;
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use super::PageRowProperty;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/page-row.ui")]
pub struct PageRow {
    icon: RefCell<String>,
    title: RefCell<String>,
    name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for PageRow {
    const NAME: &'static str = "PageRow";
    type Type = super::PageRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("iconed-row")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PageRow {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(PageRowProperty::vec);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match PageRowProperty::from(pspec.name()) {
            PageRowProperty::Title => {
                let title = value.get().unwrap();
                self.title.replace(title);
            }
            PageRowProperty::Icon => {
                let icon = value.get().unwrap();
                self.icon.replace(icon);
            }
            PageRowProperty::Name => {
                let name = value.get().unwrap();
                self.name.replace(name);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match PageRowProperty::from(pspec.name()) {
            PageRowProperty::Title => self.title.borrow().to_value(),
            PageRowProperty::Icon => self.icon.borrow().to_value(),
            PageRowProperty::Name => self.name.borrow().to_value(),
        }
    }
}
impl WidgetImpl for PageRow {}
impl BoxImpl for PageRow {}
