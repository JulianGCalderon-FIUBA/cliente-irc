mod imp;

use gtk::ffi::GTK_INVALID_LIST_POSITION;
use gtk::glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CustomFilter, FilterListModel, SelectionModel, SingleSelection};

glib::wrapper! {
    pub struct FilteredSelectionModel(ObjectSubclass<imp::FilteredSelectionModel>)
        @implements gtk::SelectionModel, gio::ListModel;
}

impl FilteredSelectionModel {
    pub fn new(parent_selection: SelectionModel, filter: CustomFilter) -> Self {
        let self_: Self = Object::builder().build();

        self_.setup_parent(parent_selection);
        self_.setup_filter(filter);
        self_.setup_model();

        self_.bind_with_parent();

        self_
    }

    pub fn update_filter(&self, filter: CustomFilter) {
        self.filter_model().set_filter(Some(&filter));
    }

    fn setup_parent(&self, parent_selection: SelectionModel) {
        self.imp()
            .parent_selection_model
            .set(parent_selection)
            .unwrap();
    }

    fn parent_selection(&self) -> SelectionModel {
        self.imp().parent_selection_model.get().unwrap().clone()
    }

    fn filter_model(&self) -> FilterListModel {
        self.imp().filter_model.get().unwrap().clone()
    }

    fn setup_filter(&self, filter: CustomFilter) {
        let filter = FilterListModel::new(Some(self.parent_selection()), Some(filter));
        self.imp().filter_model.set(filter).unwrap();
    }

    fn setup_model(&self) {
        let selection = SingleSelection::new(Some(self.filter_model()));
        self.imp().selection_model.set(selection).unwrap();
    }

    pub fn selection_model(&self) -> SingleSelection {
        self.imp().selection_model.get().unwrap().clone()
    }

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
