//! Contains the implementation of the CategorizedStackSidebar widget.

use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::{Lazy, OnceCell};
use gtk::glib::{ParamSpec, ParamSpecObject};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Box, CompositeTemplate, ListView, Stack};

use crate::components::filtered_selection_model::FilteredSelectionModel;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/components/categorized-stack-sidebar.ui")]
pub struct CategorizedStackSidebar {
    #[template_child]
    pub container: TemplateChild<Box>,
    #[template_child]
    pub default_view: TemplateChild<ListView>,
    pub default_model: OnceCell<FilteredSelectionModel>,
    pub categories: RefCell<Vec<String>>,
    pub models: RefCell<Vec<FilteredSelectionModel>>,
    stack: RefCell<Stack>,
}

#[glib::object_subclass]
impl ObjectSubclass for CategorizedStackSidebar {
    const NAME: &'static str = "CategorizedStackSidebar";
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

        self.obj()
            .connect_notify(Some("stack"), |sidebar, _| sidebar.setup_stack());
    }
}

impl WidgetImpl for CategorizedStackSidebar {}
impl BoxImpl for CategorizedStackSidebar {}
