/// This module contains the CategorizedStackSidebar widget.
mod imp;

use glib::Object;
use gtk::glib::clone;
use gtk::prelude::{Cast, ObjectExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    glib, BuilderListItemFactory, BuilderScope, CustomFilter, FilterListModel, SelectionModel,
    SingleSelection, Stack, StackPage,
};

glib::wrapper! {
    /// The CategorizedStackSidebar is a sidebar that displays the pages of a [`Stack`].
    ///
    /// It is used to navigate between the pages of the stack.
    ///
    /// Stack pages are categorized by their name
    ///
    /// Categories can be added with the `add_category` method.
    ///
    /// # Properties
    ///
    /// * `stack` - The stack to display
    ///   - Type: [`Stack`]
    ///
    /// # CSS nodes
    ///
    /// `CategorizedStackSidebar` has a single CSS node with name `categorized-stack-sidebar`.
    pub struct CategorizedStackSidebar(ObjectSubclass<imp::CategorizedStackSidebar>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl CategorizedStackSidebar {
    /// Creates a new CategorizedStackSidebar with the given stack
    pub fn new(stack: Stack) -> Self {
        Object::builder().property("stack", stack).build()
    }

    /// Returns the stack of the sidebar
    fn stack(&self) -> Stack {
        self.property("stack")
    }

    /// Called after the stack property is set
    fn setup_stack(&self) {
        let stack = self.stack();
        let pages = stack.pages();

        self.imp().pages.borrow_mut().replace(pages);
    }

    fn pages(&self) -> SelectionModel {
        self.imp().pages.borrow().clone().unwrap()
    }

    fn select_page(&self, page: StackPage) {
        let Some(name) = page.name() else {return};
        self.stack().set_visible_child_name(&name);
    }

    /// Add a category to the sidebar with an associated key
    ///
    /// Only pages with the same key will be displayed in the category
    fn add_category(&self, key: &str) {
        let model = self.build_model_for_key(key);
        let factory = self.build_factory();

        let list_view = gtk::ListView::new(Some(model), Some(factory));
    }

    /// Build a selection model that filters pages for the given key
    fn build_model_for_key(&self, key: &str) -> SingleSelection {
        let filter = CustomFilter::new(clone!(@to-owned key => move |object| {
            let page: &StackPage = object.downcast_ref().unwrap();
            let Some(name) = page.name() else {return false};
            name.starts_with(&key)
        }));
        let filter_model = FilterListModel::new(Some(self.pages()), Some(filter));

        SingleSelection::new(Some(filter_model))
    }

    /// Build a list item factory for the sidebar
    fn build_factory(&self) -> BuilderListItemFactory {
        BuilderListItemFactory::from_resource(
            BuilderScope::NONE,
            "/com/jgcalderon/irc-client/ui/sidebar_row.ui",
        )
    }
}
