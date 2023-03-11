/// This module contains the CategorizedStackSidebar widget.
mod imp;

use glib::Object;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    glib, BuilderListItemFactory, BuilderScope, CustomFilter, ListView, Orientation,
    SelectionModel, Separator, Stack, StackPage,
};

use super::filtered_selection_model::FilteredSelectionModel;

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
        self.setup_default_view();
    }

    /// Get the selection model of the stack pages
    fn pages(&self) -> SelectionModel {
        self.stack().pages()
    }

    /// Called after the stack is set or after a category is added
    ///
    /// This method sets up the default view of the sidebar
    ///
    /// All uncategorized pages will be displayed here
    fn setup_default_view(&self) {
        let factory = self.build_factory();

        let filter = self.build_default_filter();
        let model = FilteredSelectionModel::new(self.pages(), filter);

        // The order of the following lines is important.
        // Program will panic if not.

        self.imp().default_model.set(model.clone()).unwrap();

        let inner_model = model.selection_model();

        self.imp().default_view.set_factory(Some(&factory));
        self.imp().default_view.set_model(Some(&inner_model));
    }

    /// Add a category to the sidebar with an associated key
    ///
    /// Only pages with the same key will be displayed in the category
    pub fn add_category(&self, key: &str) {
        let factory = self.build_factory();

        let filter = build_filter_for_key(key);
        let model = FilteredSelectionModel::new(self.pages(), filter);

        self.imp().models.borrow_mut().push(model.clone());
        self.imp().categories.borrow_mut().push(key.to_string());

        let inner_model = model.selection_model();

        let list_view = gtk::ListView::new(Some(inner_model), Some(factory));

        self.append_new_view(list_view);

        self.imp()
            .default_model
            .get()
            .unwrap()
            .update_filter(self.build_default_filter())
    }

    /// Append a new view to the sidebar, with its corresponding separator
    fn append_new_view(&self, view: ListView) {
        let separator = build_category_separator();

        self.imp().container.append(&separator);
        self.imp().container.append(&view);
    }

    /// Builds a filter that filters out all pages that belong to a category
    fn build_default_filter(&self) -> CustomFilter {
        CustomFilter::new(
            clone!(@weak self as sidebar => @default-return false, move |object| {
                let page: &StackPage = object.downcast_ref().unwrap();
                let Some(name) = page.name() else {return false};

                let categories = sidebar.imp().categories.borrow();

                for key in categories.iter() {
                    if name.starts_with(key) {
                        return false;
                    }
                }
                true
            }),
        )
    }

    /// Build a list item factory for the sidebar
    fn build_factory(&self) -> BuilderListItemFactory {
        BuilderListItemFactory::from_resource(
            BuilderScope::NONE,
            "/com/jgcalderon/irc-client/ui/components/stack-page-label.ui",
        )
    }
}

fn build_filter_for_key(key: &str) -> CustomFilter {
    let key = key.to_owned();
    CustomFilter::new(move |object| {
        let page: &StackPage = object.downcast_ref().unwrap();
        let Some(name) = page.name() else {return false};

        name.starts_with(&key)
    })
}

/// Build a separator for the sidebar
fn build_category_separator() -> Separator {
    Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build()
}
