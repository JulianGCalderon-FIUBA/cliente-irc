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

    fn pages(&self) -> SelectionModel {
        self.imp().pages.borrow().clone().unwrap()
    }

    fn select_page(&self, page: StackPage) {
        let Some(name) = page.name() else {return};
        self.stack().set_visible_child_name(&name);
    }

    /// Called after the stack is set
    ///
    /// This method sets up the default view of the sidebar
    ///
    /// All uncategorized pages will be displayed here
    fn setup_default_view(&self) {
        let factory = self.build_factory();
        let model = self.build_model_for_default_key();

        // The order of the following lines is important.
        // Program will panic if not.

        self.add_model("default", &model);

        self.imp().default_view.set_factory(Some(&factory));
        self.imp().default_view.set_model(Some(&model));

        self.connect_model("default", &model);
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

        self.setup_default_view();
    }

    fn add_model(&self, key: &str, model: &SingleSelection) {
        self.imp()
            .models
            .borrow_mut()
            .insert(key.to_string(), model.clone());
    }

    fn append_new_view(&self, view: ListView) {
        let separator = build_category_separator();

        self.imp().container.append(&separator);
        self.imp().container.append(&view);
    }

    /// Build a selection model that contains all uncategorized pages
    fn build_model_for_default_key(&self) -> SingleSelection {
        let filter = CustomFilter::new(
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
        );

        self.build_model_for_filter(filter)
    }

    /// Connect the selection model to the sidebar
    fn connect_model(&self, key: &str, selection_model: &SingleSelection) {
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
            }),
        );

        selection_model.connect_items_changed(
            clone!(@weak self as sidebar => move |model, pos, _, add| {
                    if add == 0 {return};
                    model.set_selected(pos);
                }
            ),
        );
    }

    /// Build a selection model that filters pages for the given filter
    fn build_model_for_filter(&self, filter: CustomFilter) -> SingleSelection {
        let filter_model = FilterListModel::new(Some(self.pages()), Some(filter));

        SingleSelection::new(Some(filter_model))
    }

    /// Build a selection model that filters stack pages for the given key
    fn build_model_for_key(&self, key: &str) -> SingleSelection {
        let filter = CustomFilter::new(clone!(@to-owned key => move |object| {
            let page: &StackPage = object.downcast_ref().unwrap();
            let Some(name) = page.name() else {return false};
            name.starts_with(&key)
        }));

        self.build_model_for_filter(filter)
    }

    /// Build a list item factory for the sidebar
    fn build_factory(&self) -> BuilderListItemFactory {
        BuilderListItemFactory::from_resource(
            BuilderScope::NONE,
            "/com/jgcalderon/irc-client/ui/sidebar-row.ui",
        )
    }
}

fn build_category_separator() -> Separator {
    Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build()
}
