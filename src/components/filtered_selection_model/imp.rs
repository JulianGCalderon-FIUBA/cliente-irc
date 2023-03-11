use gtk::glib::object_subclass;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, FilterListModel, SelectionModel, SingleSelection};

#[derive(Default)]
pub struct FilteredSelectionModel {
    pub parent_selection_model: OnceCell<SelectionModel>,
    pub selection_model: OnceCell<SingleSelection>,
    pub filter_model: OnceCell<FilterListModel>,
}

#[object_subclass]
impl ObjectSubclass for FilteredSelectionModel {
    const NAME: &'static str = "FilteredSelectionModel";
    type Type = super::FilteredSelectionModel;
    type ParentType = glib::Object;
}

impl ObjectImpl for FilteredSelectionModel {}
impl SelectionModelImpl for FilteredSelectionModel {}
impl ListModelImpl for FilteredSelectionModel {
    fn item_type(&self) -> glib::Type {
        self.selection_model.get().unwrap().item_type()
    }

    fn n_items(&self) -> u32 {
        self.selection_model.get().unwrap().n_items()
    }

    fn item(&self, position: u32) -> Option<glib::Object> {
        self.selection_model.get().unwrap().item(position)
    }
}
