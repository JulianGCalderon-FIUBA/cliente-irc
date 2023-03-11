//! Implementation of the FilteredSelectionModel component.

use gtk::glib::object_subclass;
use gtk::glib::once_cell::sync::OnceCell;
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
