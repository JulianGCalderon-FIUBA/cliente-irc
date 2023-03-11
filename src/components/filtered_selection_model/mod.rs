//! This module contains the FilteredSelectionModel

mod imp;

use gtk::ffi::GTK_INVALID_LIST_POSITION;
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CustomFilter, FilterListModel, SelectionModel, SingleSelection};

glib::wrapper! {
    /// The FilteredSelectionModel is a selection model that filters the items of another selection model
    ///
    /// The selections on the parent are binded to the selection of the FilteredSelectionModel, if it is contained
    /// in the filtered items.
    ///
    /// The selection of the FilteredSelectionModel is always binded to the selection of the parent.
    ///
    /// This widgets allows to split a selection model into multiple selection models, each with a different filter.
    ///
    /// note: The FilteredSelectionModel is not a subclass of SelectionModel,
    /// but of Object, because it is not possible to subclass SelectionModel.
    /// Or at least, I don't know how to do it.
    pub struct FilteredSelectionModel(ObjectSubclass<imp::FilteredSelectionModel>);
}

impl FilteredSelectionModel {
    /// Creates a new FilteredSelectionModel with the given parent selection model and filter
    pub fn new(parent_selection: SelectionModel, filter: CustomFilter) -> Self {
        let self_: Self = Object::builder().build();

        self_.setup_parent(parent_selection);
        self_.setup_filter(filter);
        self_.setup_model();

        self_.bind_with_parent();

        self_
    }

    /// Updates the filter of the FilteredSelectionModel
    pub fn update_filter(&self, filter: CustomFilter) {
        self.filter_model().set_filter(Some(&filter));
    }

    /// Setups the parent selection model
    fn setup_parent(&self, parent_selection: SelectionModel) {
        self.imp()
            .parent_selection_model
            .set(parent_selection)
            .unwrap();
    }

    /// Returns the parent selection model
    fn parent_selection(&self) -> SelectionModel {
        self.imp().parent_selection_model.get().unwrap().clone()
    }

    /// Returns the filter model
    fn filter_model(&self) -> FilterListModel {
        self.imp().filter_model.get().unwrap().clone()
    }

    /// Setups the filter model with the given filter
    ///
    /// Parent selection model must already be setup
    fn setup_filter(&self, filter: CustomFilter) {
        let filter = FilterListModel::new(Some(self.parent_selection()), Some(filter));
        self.imp().filter_model.set(filter).unwrap();
    }

    /// Setups the main selection model. from the filter model
    ///
    /// Filter model must already be setup
    fn setup_model(&self) {
        let selection = SingleSelection::new(Some(self.filter_model()));
        self.imp().selection_model.set(selection).unwrap();
    }

    /// Returns the main selection model
    pub fn selection_model(&self) -> SingleSelection {
        self.imp().selection_model.get().unwrap().clone()
    }

    /// Binds the selection of the parent selection model to the selection of the FilteredSelectionModel
    /// if the selected item is contained in the filtered items.
    ///
    /// Binds the selection of the FilteredSelectionModel to the selection of the parent selection model.
    fn bind_with_parent(&self) {
        let parent_selection = self.parent_selection();
        let selection = self.selection_model();

        parent_selection.connect_selection_changed(
            clone!(@weak selection => move |parent_selection, _, _| {
                let selected = parent_selection.selection();
                if selected.is_empty() {
                    return;
                }
                let position = selected.nth(0);

                let Some(object) = parent_selection.item(position) else {return};

                let casted_selection = selection.upcast::<SelectionModel>();
                select_item(casted_selection, object);
            }),
        );

        selection.connect_selected_notify(clone!(@weak parent_selection => move |selection| {
            let Some(object) = selection.selected_item() else {return};


            select_item(parent_selection, object)
        }));
    }
}

/// Selects the item in the selection model that is equal to the given object
///
/// If no item is equal to the given object, the selection is cleared
fn select_item(model: SelectionModel, original_item: Object) {
    for i in 0..model.n_items() {
        let Some(item) = model.item(i) else {continue};
        if original_item == item {
            model.select_item(i, true);
            return;
        }
    }
    model.select_item(GTK_INVALID_LIST_POSITION, true);
}
