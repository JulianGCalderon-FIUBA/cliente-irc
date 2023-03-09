//! Contains the implementation of the CategorizedStackSidebar widget.

use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecObject};
use gtk::prelude::ToValue;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListView, SingleSelection, Stack};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/sidebar.ui")]
pub struct CategorizedStackSidebar {
    #[template_child]
    pub config_list: TemplateChild<ListView>,
    #[template_child]
    pub chat_list: TemplateChild<ListView>,
    pub config_selection: RefCell<Option<SingleSelection>>,
    pub chat_selection: RefCell<Option<SingleSelection>>,
    stack: RefCell<Stack>,
}

#[glib::object_subclass]
impl ObjectSubclass for CategorizedStackSidebar {
    const NAME: &'static str = "Sidebar";
    type Type = super::CategorizedStackSidebar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("categorized-stack-sidebar")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CategorizedStackSidebar {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecObject::builder::<Stack>("stack").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "stack" => {
                let stack: Stack = value.get().unwrap();
                self.stack.replace(stack);
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "stack" => self.stack.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        self.obj().setup();
    }
}

impl WidgetImpl for CategorizedStackSidebar {}
impl BoxImpl for CategorizedStackSidebar {}
