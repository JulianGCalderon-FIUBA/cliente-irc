/// This module contains the CategorizedStackSidebar widget.
mod imp;

use glib::Object;
use gtk::glib::clone;
use gtk::prelude::{Cast, ListModelExt, ObjectExt};
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
    /// All pages belonging to no category will be displayed here
    fn setup_default_view(&self) {
        let model = self.build_model_for_default_key();
        let factory = self.build_factory();

        self.imp().default_view.set_model(Some(&model));
        self.imp().default_view.set_factory(Some(&factory));
    }

    /// Add a category to the sidebar with an associated key
    ///
    /// Only pages with the same key will be displayed in the category
    fn _add_category(&self, key: &str) {
        let model = self._build_model_for_key(key);
        let factory = self.build_factory();

        let _list_view = gtk::ListView::new(Some(model), Some(factory));
    }

    fn build_model_for_default_key(&self) -> SingleSelection {
        let filter = CustomFilter::new(|_| true);
        let selection_model = self.build_model_for_filter(filter);

        self.imp()
            .default_model
            .borrow_mut()
            .replace(selection_model.clone());

        selection_model.connect_selected_item_notify(
            clone!(@weak self as sidebar => move |model| {
                let Some(page) = model.selected_item() else {return};
                let page: StackPage = page.downcast().unwrap();
                sidebar.select_page(page);
            }),
        );

        // selection_model.connect_items_changed(
        //     clone!(@weak self as sidebar => move |model, pos, _, add| {
        //             if add == 0 {return};
        //             model.set_selected(pos);
        //         }
        //     ),
        // );

        selection_model
    }

    fn build_model_for_filter(&self, filter: CustomFilter) -> SingleSelection {
        let filter_model = FilterListModel::new(Some(self.pages()), Some(filter));

        SingleSelection::new(Some(filter_model))
    }

    /// Build a selection model that filters pages for the given key
    fn _build_model_for_key(&self, key: &str) -> SingleSelection {
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
            "/com/jgcalderon/irc-client/ui/sidebar-row.ui",
        )
    }
}
