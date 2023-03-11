/// This module contains the CategorizedStackSidebar widget.
mod imp;

use glib::Object;
use gtk::ffi::GTK_INVALID_LIST_POSITION;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    glib, BuilderListItemFactory, BuilderScope, CustomFilter, FilterListModel, ListView,
    Orientation, SelectionModel, Separator, SingleSelection, Stack, StackPage,
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

        self.setup_default_view();
    }

    /// Get the selection model of the stack pages
    fn pages(&self) -> SelectionModel {
        self.imp().pages.borrow().clone().unwrap()
    }

    /// Select the given page in the stack (make it visible)
    fn select_page(&self, page: StackPage) {
        let Some(name) = page.name() else {return};
        self.stack().set_visible_child_name(&name);
    }

    /// Called after the stack is set or after a category is added
    ///
    /// This method sets up the default view of the sidebar
    ///
    /// All uncategorized pages will be displayed here
    fn setup_default_view(&self) {
        let factory = self.build_factory();
        let model = self.build_model_for_default_key();

        // The order of the following lines is important.
        // Program will panic if not.

        self.add_default_model(&model);

        self.imp().default_view.set_factory(Some(&factory));
        self.imp().default_view.set_model(Some(&model));

        self.connect_default_model(&model);
        self.connect_model_to_pages(&model);
    }

    /// Add a category to the sidebar with an associated key
    ///
    /// Only pages with the same key will be displayed in the category
    pub fn add_category(&self, key: &str) {
        let model = self.build_model_for_key(key);
        let factory = self.build_factory();

        self.add_model(key, &model);

        let list_view = gtk::ListView::new(Some(model.clone()), Some(factory));

        self.append_new_view(list_view);

        self.connect_model(key, &model);
        self.connect_model_to_pages(&model);

        self.update_default_model();
    }

    /// Connects the selection of a stack page with the selection of the model containing it
    fn connect_model_to_pages(&self, model: &SingleSelection) {
        let pages = self.pages();

        pages.connect_selection_changed(clone!(@weak model as model => move |pages, _, _| {
            let selection = pages.selection();
            if selection.is_empty() {
                return;
            }

            let Some(page) = pages.item(selection.nth(0)) else {return};

            for i in 0..model.n_items() {

                let Some(item) = model.item(i) else {continue};

                if item == page {
                    model.set_selected(i);
                    return;
                }
            }

            model.set_selected(GTK_INVALID_LIST_POSITION);
        }));
    }

    /// Adds the model to the model hashmap
    fn add_model(&self, key: &str, model: &SingleSelection) {
        self.imp()
            .models
            .borrow_mut()
            .insert(key.to_string(), model.clone());
    }

    /// Adds the default model to the structure
    fn add_default_model(&self, model: &SingleSelection) {
        self.imp().default_model.borrow_mut().replace(model.clone());
    }

    /// Append a new view to the sidebar, with its corresponding separator
    fn append_new_view(&self, view: ListView) {
        let separator = build_category_separator();

        self.imp().container.append(&separator);
        self.imp().container.append(&view);
    }

    /// Build a selection model that contains all uncategorized pages
    fn build_model_for_default_key(&self) -> SingleSelection {
        let filter = self.build_default_filter();

        let filter_model = FilterListModel::new(Some(self.pages()), Some(filter));

        self.imp()
            .default_filter_model
            .borrow_mut()
            .replace(filter_model.clone());

        self.build_selection_for_filter(filter_model)
    }

    /// Builds a filter that filters out all pages that belong to a category
    fn build_default_filter(&self) -> CustomFilter {
        CustomFilter::new(
            clone!(@weak self as sidebar => @default-return false, move |object| {
                let page: &StackPage = object.downcast_ref().unwrap();
                let Some(name) = page.name() else {return false};

                let hashmap = sidebar.imp().models.borrow();
                let keys = hashmap.keys().map(String::to_owned).collect::<Vec<String>>();

                for key in keys {
                    if name.starts_with(&key) {
                        return false;
                    }
                }
                true
            }),
        )
    }

    /// Connect the key selection model to the sidebar
    ///
    /// When a page is selected, the sidebar will select the corresponding page in the stack
    /// and unset the selection of all other models
    ///
    /// When a page is added to the model, the corresponding model will select it if is visible
    fn connect_model(&self, key: &str, selection_model: &SingleSelection) {
        self.connect_selection_for_model(key, selection_model);
    }

    /// Connect the default selection model to the sidebar
    ///
    /// When a page is selected, the sidebar will select the corresponding page in the stack
    /// and unset the selection of all other models
    ///
    /// When a page is added to the model, the corresponding model will select it if is visible
    fn connect_default_model(&self, selection_model: &SingleSelection) {
        self.connect_selection_for_default_model(selection_model);
    }

    /// When an item is selected in the model, the sidebar will select the corresponding page in the stack
    /// and unset the selection of all other models
    fn connect_selection_for_model(&self, key: &str, selection_model: &SingleSelection) {
        selection_model.connect_selected_item_notify(
            clone!(@weak self as sidebar, @to-owned key => move |model| {
                let Some(page) = model.selected_item() else {return};
                let page: StackPage = page.downcast().unwrap();
                sidebar.select_page(page);

                sidebar.imp().models.borrow_mut().iter_mut().for_each(|(k, model)| {
                    if *k != key {
                        model.set_selected(GTK_INVALID_LIST_POSITION);
                    };
                });

                let mut default_model = sidebar.imp().default_model.borrow_mut();
                if let Some(model) = default_model.as_mut() {
                    model.set_selected(GTK_INVALID_LIST_POSITION);
                }
            }),
        );
    }

    /// When an item is selected in the model, the sidebar will select the corresponding page in the stack
    /// and unset the selection of all other models
    fn connect_selection_for_default_model(&self, selection_model: &SingleSelection) {
        selection_model.connect_selected_item_notify(
            clone!(@weak self as sidebar => move |model| {
                let Some(page) = model.selected_item() else {return};
                let page: StackPage = page.downcast().unwrap();
                sidebar.select_page(page);

                sidebar.imp().models.borrow_mut().values_mut().for_each(|model| {
                    model.set_selected(GTK_INVALID_LIST_POSITION);
                });
            }),
        );
    }

    /// Build a selection model that filters pages for the given filter
    fn build_selection_for_filter(&self, filter: FilterListModel) -> SingleSelection {
        let selection_model = SingleSelection::new(Some(filter));
        selection_model.set_autoselect(false);

        selection_model
    }

    /// Build a selection model that filters stack pages for the given key
    fn build_model_for_key(&self, key: &str) -> SingleSelection {
        let filter = CustomFilter::new(clone!(@to-owned key => move |object| {
            let page: &StackPage = object.downcast_ref().unwrap();
            let Some(name) = page.name() else {return false};
            name.starts_with(&key)
        }));

        let filter_model = FilterListModel::new(Some(self.pages()), Some(filter));

        self.build_selection_for_filter(filter_model)
    }

    /// Build a list item factory for the sidebar
    fn build_factory(&self) -> BuilderListItemFactory {
        BuilderListItemFactory::from_resource(
            BuilderScope::NONE,
            "/com/jgcalderon/irc-client/ui/components/stack-page-label.ui",
        )
    }

    /// Update the default model by reapplying the filter
    fn update_default_model(&self) {
        let filter = self.build_default_filter();
        self.imp()
            .default_filter_model
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_filter(Some(&filter));
    }
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
